use clap::{ArgMatches};
use serde_derive::{Serialize, Deserialize};
use dialoguer::Confirm;

pub fn init(matches: ArgMatches) {
    let mut pacman: Vec<PacmanPackage> = vec![];
    let mut aur: Vec<AurPackage> = vec![];
    let mut not_found: Vec<String> = vec![];

    let packages: Vec<&str> = matches.subcommand_matches("install").unwrap().values_of("PACKAGES").unwrap().collect();
    for package in &packages {
        let pacman_json = reqwest::blocking::get("https://archlinux.org/packages/search/json/?name=".to_owned() + package.to_lowercase().as_str()).unwrap().text().unwrap();
        let pacman_response: PacmanResponse = serde_json::from_str(&pacman_json).unwrap();
        if pacman_response.results.is_empty() {
            let aur_json = reqwest::blocking::get("https://aur.archlinux.org/rpc/?v=5&type=info&arg[]=".to_owned() + package.to_lowercase().as_str()).unwrap().text().unwrap();
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

    let mut all: Vec<String> = vec![];
    for package in &pacman {
        all.push(package.pkgname.to_owned() + "-" + package.pkgver.as_str() + "-" + package.pkgrel.as_str());
    }
    for package in &aur {
        all.push(package.name.to_owned() + "-" + package.version.as_str());
    }

    println!("Packages: {}", packages.len());
    println!();
    println!("{}" , all.join(" "));
    println!();

    if Confirm::new().with_prompt("Do you want to proceed?").interact().unwrap() {
        if !pacman.is_empty() {
            let mut pkgs: Vec<String> = vec![];
            for package in &pacman {
                pkgs.push(package.pkgname.to_owned());
            }
            crate::lib::execute("sudo pacman -S --noconfirm ".to_owned() + pkgs.join(" ").as_str()).code().unwrap();
        }
        for package in aur {
            let folder = dirs::cache_dir().unwrap().join("toru").join(&package.name);
            std::fs::create_dir_all(&folder).unwrap();
            let path = folder.as_path().to_str().unwrap();
            if folder.read_dir().unwrap().next().is_none() {
                crate::lib::execute("git clone https://aur.archlinux.org/".to_owned() + package.name.as_str() + ".git " + path);
            } else {
                crate::lib::execute(
                    "cd ".to_owned() + path + " && " +
                    "git pull"
                );
            }
            crate::lib::execute(
                "cd ".to_owned() + path + " && " +
                "makepkg -si --noconfirm"
            );
        }
    }
}



#[derive(Serialize, Deserialize)]
struct PacmanResponse {
    results: Vec<PacmanPackage>
}

#[derive(Serialize, Deserialize)]
struct PacmanPackage {
    pkgname: String,
    pkgver: String,
    pkgrel: String
}

#[derive(Serialize, Deserialize)]
struct AurResponse {
    results: Vec<AurPackage>
}

#[derive(Serialize, Deserialize)]
struct AurPackage {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    version: String
}