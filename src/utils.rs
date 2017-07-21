use serde_json;
use std::fs::File;
use std::io::{self, stdin, stdout, Write};
use std::path::PathBuf;
use std::env;
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
