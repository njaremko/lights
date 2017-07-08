use std::collections::HashMap;

pub static DB_PATH_STRING: &str = "config";

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
