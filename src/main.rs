extern crate clap;
extern crate futures;
extern crate hyper;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate tokio_core;

mod actions;
mod structs;

use actions::*;
use clap::{App, Arg, SubCommand};
use structs::*;

fn main() {
    let state = State::new();

    let matches = App::new("lights")
        .version("1.0")
        .author("Nathan J. <njaremko@gmail.com>")
        .about("Home Lighting Controller")
        .subcommand(SubCommand::with_name("init").about("Pair with Hue Bridge"))
        .subcommand(SubCommand::with_name("sleep").about("Turn all lights off"))
        .subcommand(
            SubCommand::with_name("on").about("Turn light on").arg(
                Arg::with_name("INPUT")
                    .help("Sets the input file to use")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            SubCommand::with_name("off").about("Turn light off").arg(
                Arg::with_name("INPUT")
                    .help("Sets the input file to use")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    let output = match matches.subcommand_name() {
        Some("init") => auto_pair_hue(state),
        Some("sleep") => sleep(state),
        Some("on") => {
            light_on(
                state,
                matches
                    .subcommand_matches("on")
                    .unwrap()
                    .value_of("INPUT")
                    .unwrap(),
            )
        }
        Some("off") => {
            light_off(
                state,
                matches
                    .subcommand_matches("off")
                    .unwrap()
                    .value_of("INPUT")
                    .unwrap(),
            )
        }
        _ => return,
    };

    println!("{}", output);
}
