use reqwest;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{self, stdin, stdout, Write};
use std::path::PathBuf;
use structs::*;

pub fn get_str_line(line: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    print!("{}", line);
    stdout().flush()?;
    stdin().read_line(&mut s)?;
    Ok(s.trim_right_matches(|c| c == '\n' || c == '\r').to_string())
}

pub fn save_db(db: &DB) -> Result<(), io::Error> {
    let serialized = serde_json::to_string(&db)?;
    let mut file = File::create(get_config_path())?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

pub fn get_config_path() -> PathBuf {
    let mut path = env::home_dir().unwrap();
    path.push(".light_config");
    path
}

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
        let mut json = HashMap::new();
        json.insert("devicetype", "lights cli");

        let v: Value = state.client.post(&uri)?.json(&json)?.send()?.json()?;

        if v[0]["error"] != Value::Null {
            String::from("Press the pairing button on Hue Bridge and run init again...")
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
