#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

extern crate clap;
extern crate futures;
extern crate hyper;
extern crate regex;
extern crate tokio_core;

mod actions;
mod structs;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use structs::*;
use actions::*;

use clap::{App, Arg, SubCommand};

fn main() {
    let path = Path::new(DB_PATH_STRING);

    let mut db: DB = match path.exists() {
        true => {
            let mut s = String::new();
            File::open(&path).unwrap().read_to_string(&mut s).unwrap();
            serde_json::from_str(&s).unwrap()
        }
        false => DB { ip: String::new(), username: String::new() },
    };

    let matches = App::new("lights")
        .version("1.0")
        .author("Nathan J. <njaremko@gmail.com>")
        .about("Home Lighting Controller")
        .subcommand(
            SubCommand::with_name("init")
            .about("Pair with Hue Bridge"))
        .subcommand(
            SubCommand::with_name("sleep")
            .about("Turn all lights off"))
        .subcommand(
            SubCommand::with_name("on")
            .about("Turn light on")
            .arg(Arg::with_name("INPUT")
                 .help("Sets the input file to use")
                 .required(true)
                 .index(1)))
        .subcommand(
            SubCommand::with_name("off")
            .about("Turn light off")
            .arg(Arg::with_name("INPUT")
                 .help("Sets the input file to use")
                 .required(true)
                 .index(1)))
        .get_matches();

    let output = match matches.subcommand_name() {
        Some("init") => auto_pair_hue(&mut db),
        Some("sleep") => sleep(db),
        Some("on") => light_on(db, matches.subcommand_matches("on").unwrap().value_of("INPUT").unwrap()),
        Some("off") => light_off(db, matches.subcommand_matches("off").unwrap().value_of("INPUT").unwrap()),
        _ => return,
    };

    match output {
        Ok(stuff) => println!("{}", stuff),
        Err(err) => println!("{}", err),
    }
}
