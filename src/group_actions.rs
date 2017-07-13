use colors::*;
use regex::Regex;
use reqwest;
use serde_json;
use std::collections::HashMap;
use std::io::Read;
use structs::*;

fn get_group_map(state: &mut State) -> Result<HashMap<String, Group>, reqwest::Error> {
    let uri = format!("http://{}/api/{}/groups", &state.db.ip, &state.db.username);
    let mut resp = reqwest::get(&uri)?;
    let mut content = String::new();
    match resp.read_to_string(&mut content) {
        Err(err) => println!("{}", err),
        _ => (),
    }
    let v: HashMap<String, Group> = serde_json::from_str(&content).unwrap();
    Ok(v)
}

pub fn list_groups(mut state: State) -> Result<String, reqwest::Error> {
    let mut result = String::new();
    let groups = get_group_map(&mut state)?;
    for (_, group) in groups {
        result.push_str(&group.name);
        result.push_str("\n");
    }
    Ok(result)
}

fn toggle_group(state: &mut State, id: &str, on: bool) -> Result<(), reqwest::Error> {
    let json = json!({ "on": on });
    let uri: String = format!(
        "http://{}/api/{}/groups/{}/action",
        &state.db.ip,
        &state.db.username,
        id
        );
    state.client.put(&uri)?
        .json(&json)?
        .send()?;
    Ok(())
}

pub fn group_on(mut state: State, search: &str) -> Result<String, reqwest::Error> {
    let re = Regex::new(&search).expect("Failed to parse regex");
    let v = get_group_map(&mut state)?;

    for (group_num, group) in &v {
        if re.is_match(&group.name) {
            match toggle_group(&mut state, group_num, true) {
                Err(err) => println!("{}", err),
                _ => (),
            }
        }
    }
    Ok(String::from("Turning matches on!"))
}

pub fn group_off(mut state: State, search: &str) -> Result<String, reqwest::Error> {
    let re = Regex::new(&search).expect("Failed to parse regex");
    let v = get_group_map(&mut state)?;

    for (group_num, group) in &v {
        if re.is_match(&group.name) {
            match toggle_group(&mut state, group_num, false) {
                Err(err) => println!("{}", err),
                _ => (),
            }
        }
    }
    Ok(String::from("Turning matches off!"))
}

pub fn set_group_color(state: &mut State, id: &str, color: Color) -> Result<(), reqwest::Error> {
    let json = json!({ "hue": color.value().0, "sat": color.value().1 });
    let uri: String = format!(
        "http://{}/api/{}/groups/{}/action",
        &state.db.ip,
        &state.db.username,
        id
        );
    state.client.put(&uri)?
        .json(&json)?
        .send()?;
    Ok(())
}

pub fn group_color(mut state: State, search: &str) -> Result<String, reqwest::Error> {
    let re = Regex::new(&search).expect("Failed to parse regex");
    let v = get_group_map(&mut state)?;

    for (group_num, group) in &v {
        if re.is_match(&group.name) {
            match set_group_color(&mut state, group_num, Color::CYAN) {
                Err(err) => println!("{}", err),
                _ => (),
            }
        }
    }
    Ok(String::from("Changing matches to color!"))
}
