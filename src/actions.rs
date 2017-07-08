use futures::{Future, Stream};
use hyper::header::{ContentLength, ContentType};
use hyper::{Request, Method};
use regex::Regex;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, stdin, stdout, Write};
use std::path::Path;
use structs::*;

pub fn auto_pair_hue(mut state: State) -> String {
    if state.db.ip.is_empty() {
        let uri = String::from("https://www.meethue.com/api/nupnp");
        let get = state
            .client
            .get(uri.parse().unwrap())
            .and_then(|res| res.body().concat2());
        let got = state.core.run(get).unwrap();
        let v: Value = serde_json::from_slice(&got).unwrap();
        if v["internalipaddress"] != Value::Null {
            state.db.ip = String::from(v["internalipaddress"].as_str().unwrap());
            println!("Found Hue Bridge: {}", state.db.ip);
        }
        match save_db(&state.db) {
            Err(err) => println!("{}", err),
            _ => (),
        }
    }
    pair_hue(state)
}

pub fn pair_hue(mut state: State) -> String {
    let ip: String = match state.db.ip.is_empty() {
        true => {
            let temp = match get_str_line("Enter Hue Bridge IP: ") {
                Ok(input) => input,
                Err(err) => {
                    println!("{}", err);
                    String::new()
                },
            };
            match save_db(&state.db) {
                Err(err) => println!("{}", err),
                _ => (),
            }
            temp
        }
        false => state.db.ip.clone(),
    };

    let output = if !state.db.username.is_empty() {
        String::from("Already configured")
    } else {
        let uri = format!("http://{}/api", &ip);
        let json = r#"{"devicetype":"lights cli"}"#;
        let mut request = Request::new(Method::Post, uri.parse().unwrap());
        request.headers_mut().set(ContentType::json());
        request.headers_mut().set(ContentLength(json.len() as u64));
        request.set_body(json);

        let work = state
            .client
            .request(request)
            .and_then(|res| res.body().concat2());
        let posted = state.core.run(work).unwrap();
        let v: Value = serde_json::from_slice(&posted).unwrap();

        if v[0]["error"] != Value::Null {
            String::from(
                "Press the pairing button on Hue Bridge and run init again...",
            )
        } else if v[0]["success"]["username"] != Value::Null {
            state.db.username = String::from(v[0]["success"]["username"].as_str().unwrap());
            match save_db(&state.db) {
                Err(err) => println!("{}", err),
                _ => (),
            }
            String::from("Pairing Successful!")
        } else {
            String::from("Seems that that IP was wrong...")
        }
    };
    output
}

fn get_light_map(state: &mut State) -> HashMap<String, Light> {
    let uri = format!("http://{}/api/{}/lights", &state.db.ip, &state.db.username);
    let get = state
        .client
        .get(uri.parse().unwrap())
        .and_then(|res| res.body().concat2());
    let got = state.core.run(get).unwrap();
    let v: HashMap<String, Light> = serde_json::from_slice(&got).unwrap();
    v
}

fn toggle_light(state: &mut State, id: &str, on: bool) {
    let json_state = json!({ "on": on });
    let json = Value::to_string(&json_state);
    let uri: String = format!(
        "http://{}/api/{}/lights/{}/state",
        &state.db.ip,
        &state.db.username,
        id
        );
    let mut request = Request::new(Method::Put, uri.parse().unwrap());
    request.headers_mut().set(ContentType::json());
    request.headers_mut().set(ContentLength(json.len() as u64));
    request.set_body(json);
    let work = state
        .client
        .request(request)
        .and_then(|res| res.body().concat2());
    state.core.run(work).unwrap();
}

pub fn light_on(mut state: State, search: &str) -> String {
    let re = Regex::new(&search).unwrap();
    let v = get_light_map(&mut state);

    for (light_num, light) in &v {
        if re.is_match(&light.name) {
            toggle_light(&mut state, light_num, true);
        }
    }
    String::from("Turning matches on!")
}

pub fn light_off(mut state: State, search: &str) -> String {
    let re = Regex::new(&search).unwrap();
    let v = get_light_map(&mut state);

    for (light_num, light) in &v {
        if re.is_match(&light.name) {
            toggle_light(&mut state, light_num, false);
        }
    }
    String::from("Turning matches off!")
}

pub fn sleep(mut state: State) -> String {
    let v = get_light_map(&mut state);
    for (light_num, _) in &v {
        toggle_light(&mut state, light_num, false);
    }
    String::from("Goodnight!")
}

pub fn get_str_line(line: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    print!("{}", line);
    stdout().flush()?;
    stdin().read_line(&mut s)?;
    Ok(s.trim_right_matches(|c| c == '\n' || c == '\r').to_string())
}

pub fn save_db(db: &DB) -> Result<(), io::Error> {
    let path = Path::new(DB_PATH_STRING);
    let serialized = serde_json::to_string(&db).unwrap();
    let mut file = File::create(&path)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
