pub static DB_PATH_STRING: &str = "config";

#[derive(Serialize, Deserialize, Debug)]
pub struct DB {
    pub ip: String,
    pub username: String,
}
