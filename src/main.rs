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

mod colors;
mod group_actions;
mod light_actions;
mod structs;
mod utils;

use clap::{App, Arg, SubCommand};
use group_actions::*;
use light_actions::*;
use structs::*;

fn main() {
    let state = State::new();

    let matches = App::new("lights")
        .version("0.1.0")
        .author("Nathan J. <njaremko@gmail.com>")
        .about("Home Lighting Controller")
        .subcommand(SubCommand::with_name("init").about("Pair with Hue Bridge"))
        .subcommand(SubCommand::with_name("sleep").about("Turn all lights off"))
        .subcommand(SubCommand::with_name("list").about("Lists all lights"))
        .subcommand(
            SubCommand::with_name("group")
                .about("Perform operations on groups")
                .subcommand(SubCommand::with_name("list").about("Lists all groups"))
                .subcommand(
                    SubCommand::with_name("on").about("Turn group on").arg(
                        Arg::with_name("REGEX")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1),
                    ),
                )
                .subcommand(
                    SubCommand::with_name("off").about("Turn group off").arg(
                        Arg::with_name("REGEX")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1),
                    ),
                )
                .subcommand(
                    SubCommand::with_name("color")
                        .about("Change group color")
                        .arg(
                            Arg::with_name("REGEX")
                                .help("Sets the input file to use")
                                .required(true)
                                .index(1),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("on").about("Turn light on").arg(
                Arg::with_name("REGEX")
                    .help("Sets the input file to use")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            SubCommand::with_name("off").about("Turn light off").arg(
                Arg::with_name("REGEX")
                    .help("Sets the input file to use")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    let output = match matches.subcommand_name() {
        Some("init") => auto_pair_hue(state),
        Some("sleep") => sleep(state),
        Some("list") => list_lights(state),
        Some("group") => {
            let group_matches = matches.subcommand_matches("group").unwrap();
            match group_matches.subcommand_name() {
                Some("list") => list_groups(state),
                Some("on") => {
                    group_on(
                        state,
                        group_matches
                            .subcommand_matches("on")
                            .unwrap()
                            .value_of("REGEX")
                            .unwrap(),
                    )
                }
                Some("off") => {
                    group_off(
                        state,
                        group_matches
                            .subcommand_matches("off")
                            .unwrap()
                            .value_of("REGEX")
                            .unwrap(),
                    )
                }
                Some("color") => {
                    group_color(
                        state,
                        group_matches
                        .subcommand_matches("color")
                        .unwrap()
                        .value_of("REGEX")
                        .unwrap(),
                        )
                }
                _ => return,
            }
        }
        Some("on") => {
            light_on(
                state,
                matches
                .subcommand_matches("on")
                .unwrap()
                .value_of("REGEX")
                .unwrap(),
                )
        }
        Some("off") => {
            light_off(
                state,
                matches
                .subcommand_matches("off")
                .unwrap()
                .value_of("REGEX")
                .unwrap(),
                )
        }
        _ => return,
    };

    match output {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err),
    }
}
