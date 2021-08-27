use std::process::Command;
use miniserde::{Deserialize};
use nix::unistd::Uid;
use git2::Repository;
use srcinfo::Srcinfo;

pub fn install(package: String) {
    if !Uid::effective().is_root() {
        let folder = dirs::cache_dir().unwrap().join("toru").join(&package);
        if folder.exists() {
            rm_rf::remove(&folder).unwrap();
        }
        std::fs::create_dir_all(&folder).unwrap();
        Repository::clone(format!("https://aur.archlinux.org/{}.git", package).as_str(), &folder).unwrap();

        let srcinfo = Srcinfo::parse_file(folder.join(".SRCINFO")).unwrap();
        let mut packages: Vec<String> = vec![];
        if !srcinfo.pkg.depends.is_empty() { packages.append(&mut srcinfo.pkg.depends.first().unwrap().vec.iter().map(|p| p.to_owned()).collect()); }
        if !srcinfo.pkg.optdepends.is_empty() { packages.append(&mut srcinfo.pkg.optdepends.first().unwrap().vec.iter().map(|p| p.to_owned()).collect()); }
        if !srcinfo.base.checkdepends.is_empty() { packages.append( &mut srcinfo.base.checkdepends.first().unwrap().vec.iter().map(|p| p.to_owned()).collect()); }
        if !srcinfo.base.makedepends.is_empty() { packages.append( &mut srcinfo.base.makedepends.first().unwrap().vec.iter().map(|p| p.to_owned()).collect()); }

        let response: Vec<String> = get(&packages).iter().map(|p| p.name.to_owned()).collect();
        for package in packages {
            if response.contains(&package) {
                install(package);
            }
        }
        Command::new("makepkg").arg("-si").arg("--noconfirm").current_dir(folder).status().unwrap();
    }
}

pub fn get(packages: &Vec<String>) -> Vec<Package> {
    let mut url = "https://aur.archlinux.org/rpc/?v=5&type=info".to_string();
    for package in packages {
        url.push_str("&arg[]=");
        url.push_str(package.as_str());
    }
    let json = ureq::get(url.as_str()).call().unwrap().into_string().unwrap();
    let response: Response = miniserde::json::from_str(&json).unwrap();
    response.results
}

#[derive(Deserialize)]
struct Response {
    results: Vec<Package>
}

#[derive(Deserialize)]
pub struct Package {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String
}