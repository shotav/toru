use std::process::Command;
use clap::{ArgMatches};

pub fn init(matches: ArgMatches) {
    let packages: Vec<&str> = matches.subcommand_matches("remove").unwrap().values_of("PACKAGES").unwrap().collect();
    Command::new("sudo").arg("pacman").arg("-Rns").args(packages).status().unwrap();
}