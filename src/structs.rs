use reqwest;
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub static DB_PATH_STRING: &str = "config";

pub struct State {
    pub client: reqwest::Client,
    pub db: DB,
}

impl State {
    pub fn new() -> State {
        let path = Path::new(DB_PATH_STRING);
        State {
            client: reqwest::Client::new().unwrap(),
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
    pub hue: u16,
    #[serde(default)]
    pub sat: u8,
    #[serde(default)]
    pub xy: [f64; 2],
    #[serde(default)]
    pub ct: u16,
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
pub struct Group {
    pub name: String,
    pub lights: Vec<String>,
    #[serde(rename = "type")]
    pub group_type: String,
    pub state: GroupState,
    pub recycle: bool,
    pub class: String,
    pub action: GroupAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupState {
    pub all_on: bool,
    pub any_on: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupAction {
    pub on: bool,
    pub bri: u8,
    #[serde(default)]
    pub hue: u16,
    #[serde(default)]
    pub sat: u8,
    #[serde(default)]
    pub effect: String,
    #[serde(default)]
    pub xy: [f64; 2],
    #[serde(default)]
    pub ct: u16,
    pub alert: String,
    #[serde(default)]
    pub colormode: String,
}
