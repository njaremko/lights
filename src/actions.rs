use futures::{Future, Stream};
use hyper::header::{ContentLength, ContentType};
use hyper::{self, Client, Request, Method};
use serde_json::{self, Value};
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use structs::*;
use tokio_core::reactor::Core;
use std::collections::HashMap;

pub fn pair_hue(db: &mut DB) -> Result<String, hyper::Error> {
    let mut output = String::new();

    let ip: String = match db.ip.is_empty() {
        true => get_str_line("Enter Hue Bridge IP: "),
        false  => db.ip.clone(),
    };

    if !db.username.is_empty() {
        output = String::from("Already configured");
    } else {
        let mut core = Core::new()?;
        let client = Client::new(&core.handle());
        let mut uri = String::from("http://");
        uri.push_str(&ip);
        uri.push_str("/api");

        let json = r#"{"devicetype":"lights cli"}"#; 
        let mut request = Request::new(Method::Post, uri.parse().unwrap());
        request.headers_mut().set(ContentType::json());
        request.headers_mut().set(ContentLength(json.len() as u64));
        request.set_body(json);

        let work = client.request(request).and_then(|res| {
            res.body().concat2()
        });
        let posted = core.run(work).unwrap();
        let v: Value = serde_json::from_slice(&posted).unwrap();    
        
        if v[0]["error"] != Value::Null {
            db.ip = ip;
            output = String::from("Press the pairing button on Hue Bridge and run init again...");
            save_db(db);
        } else if v[0]["success"]["username"] != Value::Null { 
            db.ip = ip;
            db.username = String::from(v[0]["success"]["username"].as_str().unwrap());
            output = String::from("Pairing Successful!");
            save_db(db);
        } else {
            output = String::from("Seems that that IP was wrong...");
        }
    };
    Ok(output)
}

fn get_light_map(db: &DB) -> Result<HashMap<String, Light>, hyper::Error> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let mut uri = String::from("http://");
    uri.push_str(&db.ip);
    uri.push_str("/api/");
    uri.push_str(&db.username);
    uri.push_str("/lights");
    let get = client.get(uri.parse()?).and_then(|res| {
        res.body().concat2()
    });
    let got = core.run(get).unwrap();
    let v: HashMap<String, Light> = serde_json::from_slice(&got).unwrap();
    Ok(v)
}

pub fn sleep(db: DB) -> Result<String, hyper::Error> {
    let v = get_light_map(&db).unwrap();
    println!("{}", v.get("1").unwrap().name);
    Ok(String::from("Goodnight!"))
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
