use reqwest;
use serde_json;
use std::collections::HashMap;
use std::io::Read;
use crate::structs::*;
use crate::utils::*;

fn get_light_map(state: &mut State) -> Result<HashMap<String, Light>, reqwest::Error> {
    let uri = format!("http://{}/api/{}/lights", &state.db.ip, &state.db.username);
    let mut resp = reqwest::get(&uri)?;
    let mut content = String::new();
    if let Err(err) = resp.read_to_string(&mut content) {
        eprintln!("Error: {}", err);
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
        &state.db.ip, &state.db.username, id
    );
    state.client.put(&uri).json(&json).send()?;
    Ok(())
}

pub fn light_on(mut state: State, search: &str) -> Result<String, reqwest::Error> {
    let regexes = parse_lights(&search);
    let v = get_light_map(&mut state)?;
    for (light_num, light) in &v {
        for re in &regexes {
            if re.is_match(&light.name) {
                if let Err(err) = toggle_light(&mut state, light_num, true) {
                    eprintln!("Error: {}", err);
                }
            }
        }
    }
    Ok(String::from("Turning matches on!"))
}

pub fn light_off(mut state: State, search: &str) -> Result<String, reqwest::Error> {
    let regexes = parse_lights(&search);
    let v = get_light_map(&mut state)?;

    for (light_num, light) in &v {
        for re in &regexes {
            if re.is_match(&light.name) {
                if let Err(err) = toggle_light(&mut state, light_num, false) {
                    eprintln!("Error: {}", err);
                }
            }
        }
    }
    Ok(String::from("Turning matches off!"))
}

pub fn light_off_except(mut state: State, search: &str) -> Result<String, reqwest::Error> {
    let regexes = parse_lights(&search);
    let v = get_light_map(&mut state)?;

    for (light_num, light) in &v {
        for re in &regexes {
            if !re.is_match(&light.name) {
                if let Err(err) = toggle_light(&mut state, light_num, false) {
                    eprintln!("Error: {}", err);
                }
            }
        }
    }
    Ok(String::from("Turning matches off!"))
}

pub fn all_lights_off(mut state: State) -> Result<String, reqwest::Error> {
    let v = get_light_map(&mut state)?;

    for light_num in v.keys() {
        if let Err(err) = toggle_light(&mut state, light_num, false) {
            eprintln!("Error: {}", err);
        }
    }
    Ok(String::from("Turning matches off!"))
}

pub fn sleep(mut state: State) -> Result<String, reqwest::Error> {
    let v = get_light_map(&mut state)?;
    for light_num in v.keys() {
        if let Err(err) = toggle_light(&mut state, light_num, false) {
            eprintln!("Error: {}", err);
        }
    }
    Ok(String::from("Goodnight!"))
}
