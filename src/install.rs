use std::process::Command;
use clap::{ArgMatches};
use serde::{Deserialize};
use dialoguer::Confirm;

pub fn init(matches: ArgMatches) {
    let args: Vec<&str> = matches.subcommand_matches("install").unwrap().values_of("PACKAGES").unwrap().collect();

    let mut pacman: Vec<PacmanPackage> = vec![];
    let mut aur: Vec<AurPackage> = vec![];
    let mut not_found: Vec<String> = vec![];

    for package in args {
        let pacman_json = reqwest::blocking::get(format!("https://archlinux.org/packages/search/json/?name={}", package.to_lowercase())).unwrap().text().unwrap();
        let pacman_response: PacmanResponse = serde_json::from_str(&pacman_json).unwrap();
        if pacman_response.results.is_empty() {
            let aur_json = reqwest::blocking::get(format!("https://aur.archlinux.org/rpc/?v=5&type=info&arg[]={}", package.to_lowercase())).unwrap().text().unwrap();
            let aur_response: AurResponse = serde_json::from_str(&aur_json).unwrap();
            if aur_response.results.is_empty() {
                not_found.push(package.to_lowercase());
            } else {
                let pkg = aur_response.results.first().unwrap();
                aur.push(AurPackage {name: pkg.name.to_owned(), version: pkg.version.to_owned()});
            }
        } else {
            let pkg = pacman_response.results.first().unwrap();
            pacman.push(PacmanPackage {pkgname: pkg.pkgname.to_owned(), pkgver: pkg.pkgver.to_owned(), pkgrel: pkg.pkgrel.to_owned()});
        }
    }

    if !not_found.is_empty() {
        println!("Packages not found: {}", not_found.join(" "));
        return;
    }

    let mut all: Vec<String> = pacman.iter().map(|p| format!("{}-{}-{}", p.pkgname, p.pkgver, p.pkgrel)).collect();
    all.append(&mut aur.iter().map(|p| format!("{}-{}", p.name, p.version)).collect());

    println!("Packages: {}", pacman.len() + aur.len());
    println!();
    println!("{}" , all.join(" "));
    println!();

    if Confirm::new().with_prompt("Do you want to proceed?").interact().unwrap() {
        if !pacman.is_empty() {
            let pkgs: Vec<String> = pacman.iter().map(|p| p.pkgname.to_owned()).collect();
            Command::new("sudo").arg("pacman").args(["-S", "--noconfirm"]).args(pkgs).status().unwrap();
        }
        for package in aur {
            crate::aur::install(package.name);
        }
    }
}

#[derive(Deserialize)]
struct PacmanResponse {
    results: Vec<PacmanPackage>
}

#[derive(Deserialize)]
struct PacmanPackage {
    pkgname: String,
    pkgver: String,
    pkgrel: String
}

#[derive(Deserialize)]
struct AurResponse {
    results: Vec<AurPackage>
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AurPackage {
    name: String,
    version: String
}