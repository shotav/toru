use std::process::Command;
use std::collections::HashMap;
use miniserde::{Deserialize};

pub fn init() {
    Command::new("sudo").arg("pacman").args(["-Syu", "--noconfirm"]).status().unwrap();
    let output = Command::new("sudo").arg("pacman").arg("-Qm").output().unwrap();
    let mut packages = HashMap::new();
    let mut url = "https://aur.archlinux.org/rpc/?v=5&type=info".to_string();
    for line in String::from_utf8(output.stdout).unwrap().lines() {
        let mut split = line.split_whitespace();
        let name = split.next().unwrap();
        let version = split.next().unwrap();
        url.push_str("&arg[]=");
        url.push_str(name);
        packages.insert(name.to_string(), version.to_string());
    }
    let json = ureq::get(url.as_str()).call().unwrap().into_string().unwrap();
    let response: Response = miniserde::json::from_str(&json).unwrap();
    for package in response.results {
        if package.version != packages.get(package.name.as_str()).unwrap().to_string() {
            crate::aur::install(package.name);
        }
    }
}

#[derive(Deserialize)]
struct Response {
    results: Vec<Package>
}

#[derive(Deserialize)]
struct Package {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    version: String
}