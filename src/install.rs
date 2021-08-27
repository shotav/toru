use std::process::Command;
use clap::{ArgMatches};

pub fn init(matches: ArgMatches) {
    let args: Vec<&str> = matches.subcommand_matches("install").unwrap().values_of("PACKAGES").unwrap().collect();
    let packages: Vec<String> = args.iter().map(|p| p.to_lowercase()).collect();
    let aur: Vec<String> = crate::aur::get(&packages).iter().map(|p| p.name.to_owned()).collect();
    let mut pacman: Vec<String> = vec![];
    for package in packages {
        if !aur.contains(&package) {
            pacman.push(package);
        }
    }

    if !pacman.is_empty() {
        let status = Command::new("sudo").arg("pacman").args(["-S", "--noconfirm"]).args(pacman).status().unwrap();
        if !status.success() {
            return;
        }
    }
    for package in aur {
        crate::aur::install(package);
    }
}