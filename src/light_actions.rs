use regex::Regex;
use reqwest;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::io::Read;
use structs::*;
use utils::*;

pub fn auto_pair_hue(mut state: State) -> Result<String, reqwest::Error> {
    if state.db.ip.is_empty() {
        let mut resp = reqwest::get("https://www.meethue.com/api/nupnp").unwrap();
        let mut content = String::new();
        match resp.read_to_string(&mut content) {
            Err(err) => println!("{}", err),
            _ => (),
        }
        let v: Value = serde_json::from_str(&content).unwrap();
        if v[0]["internalipaddress"] != Value::Null {
            state.db.ip = match v[0]["internalipaddress"].as_str() {
                Some(val) => String::from(val),
                None => String::new(),
            };
            println!("Found Hue Bridge: {}", state.db.ip);
        } else {
            println!("Failed to auto-locate Hue bridge...");
        }
        match save_db(&state.db) {
            Err(err) => println!("Failed to save db: {}", err),
            _ => (),
        }
    }
    pair_hue(state)
}

pub fn pair_hue(mut state: State) -> Result<String, reqwest::Error> {
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

        let v: Value = state.client.post(&uri)?.json(&json)?.send()?.json()?;

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
            String::from("Seems that IP was wrong...")
        }
    };
    Ok(output)
}

fn get_light_map(state: &mut State) -> Result<HashMap<String, Light>, reqwest::Error> {
    let uri = format!("http://{}/api/{}/lights", &state.db.ip, &state.db.username);
    let mut resp = reqwest::get(&uri)?;
    let mut content = String::new();
    match resp.read_to_string(&mut content) {
        Err(err) => println!("{}", err),
        _ => (),
    }
    let v: HashMap<String, Light> = serde_json::from_str(&content).unwrap();
    Ok(v)
}

pub fn list_lights(mut state: State) -> Result<String, reqwest::Error> {
    let mut result = String::new();
    let lights = get_light_map(&mut state)?;
    for (_, light) in lights {
        result.push_str(&light.name);
        result.push_str("\n");
    }
    Ok(result)
}

fn toggle_light(state: &mut State, id: &str, on: bool) -> Result<(), reqwest::Error> {
    let json = json!({ "on": on });
    let uri: String = format!(
        "http://{}/api/{}/lights/{}/state",
        &state.db.ip,
        &state.db.username,
        id
    );
    state.client.put(&uri)?.json(&json)?.send()?;
    Ok(())
}

pub fn light_on(mut state: State, search: &str) -> Result<String, reqwest::Error> {
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

pub fn light_off(mut state: State, search: &str) -> Result<String, reqwest::Error> {
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

pub fn light_off_except(mut state: State, search: &str) -> Result<String, reqwest::Error> {
    let re = Regex::new(&search).expect("Failed to parse regex");
    let v = get_light_map(&mut state)?;

    for (light_num, light) in &v {
        if !re.is_match(&light.name) {
            match toggle_light(&mut state, light_num, false) {
                Err(err) => println!("{}", err),
                _ => (),
            }
        }
    }
    Ok(String::from("Turning matches off!"))
}

pub fn sleep(mut state: State) -> Result<String, reqwest::Error> {
    let v = get_light_map(&mut state)?;
    for (light_num, _) in &v {
        match toggle_light(&mut state, light_num, false) {
            Err(err) => println!("{}", err),
            _ => (),
        }
    }
    Ok(String::from("Goodnight!"))
}
