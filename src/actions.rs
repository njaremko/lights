use std::io;
use futures::{Future, Stream};
use hyper::{self, Client, Request, Method, Uri};
use tokio_core::reactor::Core;
use serde_json::{self, Value};
use structs::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{stdin, stdout, Write};
use std::str::FromStr;

pub fn pair_hue(db: &mut DB) -> Result<String, hyper::Error> {
    // If first time, print message to press button
    let ip = get_str_line("Enter Hue Bridge IP: ");
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let mut uri = String::from("http://");
    uri.push_str(&ip);
    uri.push_str("/api");
    
    let mut request = Request::new(Method::Post, Uri::from_str(&uri).unwrap());
    request.set_body("{\"devicetype\":\"lights cli\"}");
    let work = client.request(request).and_then(|res| {
        res.body().concat2().and_then(move |body| {
            let v: Value = serde_json::from_slice(&body).unwrap();    

            if v["success"] == Value::Null {
                println!("Press the pairing button on Hue Bridge and run init again...");
                return
            } 
            db.username = String::from(v["username"].as_str().unwrap());
            println!("{}", v["username"]);
            save_db(db);
            Ok(String::from("Pairing Successful!"));
        });
    });
    core.run(work)?
}

pub fn get_str_line(line: &str) -> String {
    let mut s = String::new();
    print!("{}", line);
    stdout().flush().expect("Failed to flush stdout");
    stdin().read_line(&mut s).unwrap();
    s.trim_right_matches(|c| c == '\n' || c == '\r').to_string()
}

pub fn save_db(db: &mut DB) {
    let path = Path::new(DB_PATH_STRING);
    let serialized = serde_json::to_string(&db).unwrap();
    let mut file = File::create(&path).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}
