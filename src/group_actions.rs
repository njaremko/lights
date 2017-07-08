use structs::*;
use regex::Regex;
use hyper::header::{ContentLength, ContentType};
use std::collections::HashMap;
use futures::{Future, Stream};
use hyper::{self, Request, Method};
use serde_json::{self, Value};

fn get_group_map(state: &mut State) -> Result<HashMap<String, Group>, hyper::Error> {
    let uri = format!("http://{}/api/{}/groups", &state.db.ip, &state.db.username);
    let get = state
        .client
        .get(uri.parse()?)
        .and_then(|res| res.body().concat2());
    let got = state.core.run(get)?;
    let v: HashMap<String, Group> = serde_json::from_slice(&got).unwrap();
    Ok(v)
}

pub fn list_groups(mut state: State) -> Result<String, hyper::Error> {
    let mut result = String::new();
    let groups = get_group_map(&mut state)?;
    for (_, group) in groups {
        result.push_str(&group.name);
        result.push_str("\n");
    }
    Ok(result)
}

fn toggle_group(state: &mut State, id: &str, on: bool) -> Result<(), hyper::Error> {
    let json_state = json!({ "on": on });
    let json = Value::to_string(&json_state);
    let uri: String = format!(
        "http://{}/api/{}/groups/{}/action",
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

pub fn group_on(mut state: State, search: &str) -> Result<String, hyper::Error> {
    let re = Regex::new(&search).expect("Failed to parse regex");
    let v = get_group_map(&mut state)?;

    for (light_num, light) in &v {
        if re.is_match(&light.name) {
            match toggle_group(&mut state, light_num, true) {
                Err(err) => println!("{}", err),
                _ => (),
            }
        }
    }
    Ok(String::from("Turning matches on!"))
}

pub fn group_off(mut state: State, search: &str) -> Result<String, hyper::Error> {
    let re = Regex::new(&search).expect("Failed to parse regex");
    let v = get_group_map(&mut state)?;

    for (light_num, light) in &v {
        if re.is_match(&light.name) {
            match toggle_group(&mut state, light_num, false) {
                Err(err) => println!("{}", err),
                _ => (),
            }
        }
    }
    Ok(String::from("Turning matches off!"))
}
