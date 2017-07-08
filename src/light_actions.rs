use futures::{Future, Stream};
use hyper::header::{ContentLength, ContentType};
use hyper::{self, Request, Method};
use regex::Regex;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, stdin, stdout, Write};
use std::path::Path;
use structs::*;

pub fn auto_pair_hue(mut state: State) -> Result<String, hyper::Error> {
    if state.db.ip.is_empty() {
        let uri = String::from("https://www.meethue.com/api/nupnp");
        let get = state
            .client
            .get(uri.parse()?)
            .and_then(|res| res.body().concat2());
        let got = state.core.run(get)?;
        let v: Value = serde_json::from_slice(&got).unwrap();
        if v["internalipaddress"] != Value::Null {
            state.db.ip = match v["internalipaddress"].as_str() {
                Some(val) => String::from(val),
                None => String::new(),
            };
            println!("Found Hue Bridge: {}", state.db.ip);
        }
        match save_db(&state.db) {
            Err(err) => println!("Failed to save db: {}", err),
            _ => (),
        }
    }
    pair_hue(state)
}

pub fn pair_hue(mut state: State) -> Result<String, hyper::Error> {
    let ip: String = match state.db.ip.is_empty() {
        true => {
            let temp = match get_str_line("Enter Hue Bridge IP: ") {
                Ok(input) => input,
                Err(err) => {
                    println!("{}", err);
                    String::new()
                }
            };
            match save_db(&state.db) {
                Err(err) => println!("Failed to save db: {}", err),
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
        let posted = state.core.run(work)?;
        let v: Value = serde_json::from_slice(&posted).unwrap();

        if v[0]["error"] != Value::Null {
            String::from(
                "Press the pairing button on Hue Bridge and run init again...",
            )
        } else if v[0]["success"]["username"] != Value::Null {
            state.db.username = match v[0]["success"]["username"].as_str() {
                Some(val) => String::from(val),
                None => String::new(),
            };
            match save_db(&state.db) {
                Err(err) => println!("Failed to save db: {}", err),
                _ => (),
            }
            String::from("Pairing Successful!")
        } else {
            String::from("Seems that that IP was wrong...")
        }
    };
    Ok(output)
}

fn get_light_map(state: &mut State) -> Result<HashMap<String, Light>, hyper::Error> {
    let uri = format!("http://{}/api/{}/lights", &state.db.ip, &state.db.username);
    let get = state
        .client
        .get(uri.parse()?)
        .and_then(|res| res.body().concat2());
    let got = state.core.run(get)?;
    let v: HashMap<String, Light> = serde_json::from_slice(&got).unwrap();
    Ok(v)
}

pub fn list_lights(mut state: State) -> Result<String, hyper::Error> {
    let mut result = String::new();
    let lights = get_light_map(&mut state)?;
    for (_, light) in lights {
        result.push_str(&light.name);
        result.push_str("\n");
    }
    Ok(result)
}

fn toggle_light(state: &mut State, id: &str, on: bool) -> Result<(), hyper::Error> {
    let json_state = json!({ "on": on });
    let json = Value::to_string(&json_state);
    let uri: String = format!(
        "http://{}/api/{}/lights/{}/state",
        &state.db.ip,
        &state.db.username,
        id
    );
    let mut request = Request::new(Method::Put, uri.parse()?);
    request.headers_mut().set(ContentType::json());
    request.headers_mut().set(ContentLength(json.len() as u64));
    request.set_body(json);
    let work = state
        .client
        .request(request)
        .and_then(|res| res.body().concat2());
    state.core.run(work)?;
    Ok(())
}

pub fn light_on(mut state: State, search: &str) -> Result<String, hyper::Error> {
    let re = Regex::new(&search).expect("Failed to parse regex");
    let v = get_light_map(&mut state)?;

    for (light_num, light) in &v {
        if re.is_match(&light.name) {
            match toggle_light(&mut state, light_num, true) {
                Err(err) => println!("{}", err),
                _ => (),
            }
        }
    }
    Ok(String::from("Turning matches on!"))
}

pub fn light_off(mut state: State, search: &str) -> Result<String, hyper::Error> {
    let re = Regex::new(&search).expect("Failed to parse regex");
    let v = get_light_map(&mut state)?;

    for (light_num, light) in &v {
        if re.is_match(&light.name) {
            match toggle_light(&mut state, light_num, false) {
                Err(err) => println!("{}", err),
                _ => (),
            }
        }
    }
    Ok(String::from("Turning matches off!"))
}

pub fn sleep(mut state: State) -> Result<String, hyper::Error> {
    let v = get_light_map(&mut state)?;
    for (light_num, _) in &v {
        match toggle_light(&mut state, light_num, false) {
            Err(err) => println!("{}", err),
            _ => (),
        }
    }
    Ok(String::from("Goodnight!"))
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
    let serialized = serde_json::to_string(&db)?;
    let mut file = File::create(&path)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
