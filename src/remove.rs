use std::process::Command;
use clap::{ArgMatches};

pub fn init(matches: ArgMatches) {
    let args: Vec<&str> = matches.subcommand_matches("remove").unwrap().values_of("PACKAGES").unwrap().collect();
    let packages: Vec<String> = args.iter().map(|p| p.to_lowercase()).collect();
    Command::new("sudo").arg("pacman").args(["-Rns", "--noconfirm"]).args(packages).status().unwrap();
}