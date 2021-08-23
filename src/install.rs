use std::process::Command;
use clap::{ArgMatches};
use miniserde::{Deserialize};

pub fn init(matches: ArgMatches) {
    let args: Vec<&str> = matches.subcommand_matches("install").unwrap().values_of("PACKAGES").unwrap().collect();
    for arg in args {
        let package = arg.to_lowercase();
        let json = ureq::get(format!("https://aur.archlinux.org/rpc/?v=5&type=info&arg[]={}", package).as_str()).call().unwrap().into_string().unwrap();
        let response: Response = miniserde::json::from_str(&json).unwrap();
        if response.results.is_empty() {
            let status = Command::new("sudo").arg("pacman").args(["-S", "--noconfirm"]).arg(package).status().unwrap();
            if !status.success() {
                return;
            }
        } else {
            crate::aur::install(package);
        }
    }
}

#[derive(Deserialize)]
struct Response {
    results: Vec<Package>
}

#[derive(Deserialize)]
struct Package {
}