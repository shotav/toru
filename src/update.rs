use std::process::Command;
use std::collections::HashMap;

pub fn init() {
    let status = Command::new("sudo").arg("pacman").args(["-Syu", "--noconfirm"]).status().unwrap();
    if !status.success() {
        return;
    }

    let output = Command::new("pacman").arg("-Qm").output().unwrap();
    let mut packages = HashMap::new();
    let mut aur = vec![];
    let lines = String::from_utf8(output.stdout).unwrap();
    for line in lines.lines() {
        let mut split = line.split_whitespace();
        let name = split.next().unwrap();
        packages.insert(name, split.next().unwrap());
        aur.push(name.to_owned());
    }

    for package in crate::aur::get(&aur) {
        if package.version != packages.get(package.name.as_str()).unwrap().to_string() {
            crate::aur::install(package.name);
        }
    }
}