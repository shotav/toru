use std::process::Command;
use clap::{ArgMatches};
use miniserde::{Deserialize};

pub fn init(matches: ArgMatches) {
    let args: Vec<&str> = matches.subcommand_matches("install").unwrap().values_of("PACKAGES").unwrap().collect();
    let packages: Vec<String> = args.iter().map(|p| p.to_lowercase()).collect();
    let mut url = "https://aur.archlinux.org/rpc/?v=5&type=info".to_string();
    for package in &packages {
        url.push_str("&arg[]=");
        url.push_str(package.as_str());
    }
    let json = ureq::get(url.as_str()).call().unwrap().into_string().unwrap();
    let response: Response = miniserde::json::from_str(&json).unwrap();
    let aur: Vec<String> = response.results.iter().map(|p| p.name.to_owned()).collect();
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

#[derive(Deserialize)]
struct Response {
    results: Vec<Package>
}

#[derive(Deserialize)]
struct Package {
    #[serde(rename = "Name")]
    name: String
}