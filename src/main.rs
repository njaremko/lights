extern crate clap;
extern crate regex;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod colors;
mod group_actions;
mod light_actions;
mod structs;
mod utils;

use clap::{App, Arg, SubCommand};
use group_actions::*;
use light_actions::*;
use structs::*;
use utils::*;

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
                    SubCommand::with_name("off")
                        .about("Turn group off")
                        .arg(
                            Arg::with_name("REGEX")
                                .help("Sets the input file to use")
                                .required(true)
                                .index(1),
                        )
                        .arg(
                            Arg::with_name("except")
                                .short("e")
                                .long("except")
                                .help("Turns off all lights except matches"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("color")
                        .about("Change group color")
                        .arg(
                            Arg::with_name("REGEX")
                                .help("Sets regex used to find matches")
                                .required(true)
                                .index(1),
                        )
                        .arg(
                            Arg::with_name("COLOR")
                                .help("Sets the color")
                                .required(true)
                                .index(2),
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
            SubCommand::with_name("off")
                .about("Turn light off")
                .arg(
                    Arg::with_name("REGEX")
                        .help("Sets the input file to use")
                        .index(1),
                )
                .arg(
                    Arg::with_name("except")
                        .short("e")
                        .long("except")
                        .help("Turns off all lights except matches"),
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
                    let off_matches = group_matches.subcommand_matches("off").unwrap();
                    if off_matches.occurrences_of("except") > 0 {
                        group_off_except(state, off_matches.value_of("REGEX").unwrap())
                    } else {
                        group_off(state, off_matches.value_of("REGEX").unwrap())
                    }
                }
                Some("color") => {
                    let color_matches = group_matches.subcommand_matches("color").unwrap();
                    group_color(
                        state,
                        color_matches.value_of("REGEX").unwrap(),
                        color_matches.value_of("COLOR").unwrap(),
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
            let off_matches = matches.subcommand_matches("off").unwrap();
            if off_matches.occurrences_of("REGEX") > 0 {
                if off_matches.occurrences_of("except") > 0 {
                    light_off_except(state, off_matches.value_of("REGEX").unwrap())
                } else {
                    light_off(state, off_matches.value_of("REGEX").unwrap())
                }
            } else {
                all_lights_off(state)
            }
        }
        _ => return,
    };

    match output {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err),
    }
}
