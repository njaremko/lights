#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod actions;
mod structs;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use structs::*;
use actions::*;

use clap::{App, /*Arg,*/ SubCommand};

fn main() {
    let path = Path::new(DB_PATH_STRING);

    let mut db: DB = match path.exists() {
        true => {
            let mut s = String::new();
            File::open(&path).unwrap().read_to_string(&mut s).unwrap();
            serde_json::from_str(&s).unwrap()
        }
        false => DB { username: String::from("") },
    };

    let matches = App::new("lights")
        .version("1.0")
        .author("Nathan J. <njaremko@gmail.com>")
        .about("Home Lighting Controller")
        .subcommand(
            SubCommand::with_name("init")
            .about("Pair with Hue Bridge"))
        .get_matches();

    let output = match matches.subcommand_name() {
        Some("init") => pair_hue(&mut db),
        _ => return,
    };

    match output {
        Ok(stuff) => println!("{}", stuff),
        Err(err) => println!("{}", err),
    }
}
