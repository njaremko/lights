use serde_json;
use std::fs::File;
use std::io::{self, stdin, stdout, Write};
use std::path::Path;
use structs::*;

pub fn get_str_line(line: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    print!("{}", line);
    stdout().flush()?;
    stdin().read_line(&mut s)?;
    Ok(s.trim_right_matches(|c| c == '\n' || c == '\r').to_string())
}

pub fn save_db(db: &DB) -> Result<(), io::Error> {
    let path = Path::new(DB_PATH_STRING);
    let serialized = serde_json::to_string(&db)?;
    let mut file = File::create(&path)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
