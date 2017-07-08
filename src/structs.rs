use hyper::client::HttpConnector;
use hyper::{Body, Client};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tokio_core::reactor::Core;

pub static DB_PATH_STRING: &str = "config";

#[derive(Debug)]
pub struct State {
    pub client: Client<HttpConnector, Body>,
    pub core: Core,
    pub db: DB,
}

impl State {
    pub fn new() -> State {
        let path = Path::new(DB_PATH_STRING);
        let core = Core::new().unwrap();
        State {
            client: Client::new(&core.handle()),
            core: core,
            db: match path.exists() {
                true => {
                    let mut s = String::new();
                    File::open(&path).unwrap().read_to_string(&mut s).unwrap();
                    serde_json::from_str(&s).unwrap()
                }
                false => DB {
                    ip: String::new(),
                    username: String::new(),
                },
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DB {
    pub ip: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightState {
    pub on: bool,
    pub bri: u8,
    #[serde(default)]
    pub hue: u32,
    #[serde(default)]
    pub sat: u8,
    #[serde(default)]
    pub xy: [f64; 2],
    #[serde(default)]
    pub ct: u32,
    pub alert: String,
    #[serde(default)]
    pub effect: String,
    #[serde(default)]
    pub colormode: String,
    pub reachable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Light {
    pub state: LightState,
    #[serde(rename = "type")]
    pub light_type: String,
    pub name: String,
    pub modelid: String,
    pub manufacturername: String,
    pub uniqueid: String,
    pub swversion: String,
    #[serde(default)]
    pub swconfigid: String,
    #[serde(default)]
    pub productid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lights(pub HashMap<String, Light>);
