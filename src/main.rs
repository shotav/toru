extern crate clap;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;
extern crate dirs;

use std::process::{Command, ExitStatus};
use clap::{Arg, App, AppSettings};
use serde_derive::{Serialize, Deserialize};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(App::new("clean")
            .about("Clean cache."))
        .subcommand(App::new("install")
            .about("Install the specified packages.")
            .arg(Arg::new("PACKAGES")
                .about("Packages to install.")
                .multiple_values(true)
                .required(true)))
        .subcommand(App::new("remove")
            .about("Remove the specified packages.")
            .arg(Arg::new("PACKAGES")
                .about("Packages to remove.")
                .multiple_values(true)
                .required(true)))
        .subcommand(App::new("search")
            .about("Search and install a package.")
            .arg(Arg::new("TERMS")
                .about("Search terms.")
                .multiple_values(true)
                .required(true)))
        .subcommand(App::new("update")
            .about("Update all packages."))
        .get_matches();

    match matches.subcommand() {
        Some(("clean", _)) => {
            std::fs::remove_dir_all(dirs::cache_dir().unwrap().join("toru")).unwrap();
        },
        Some(("install", _)) => {
            let mut pacman: Vec<String> = vec![];
            let mut aur: Vec<String> = vec![];

            let packages: Vec<&str> = matches.subcommand_matches("install").unwrap().values_of("PACKAGES").unwrap().collect();
            for package in packages {
                let response = reqwest::blocking::get("https://aur.archlinux.org/rpc/?v=5&type=search&by=name&arg=".to_owned() + package.to_lowercase().as_str()).unwrap().text().unwrap();
                let v: Response = serde_json::from_str(&response).unwrap();
                let mut found = false;
                for pkg in v.results {
                    if package.to_lowercase() == pkg.name {
                        aur.push(pkg.name);
                        found = true;
                    }
                }
                if !found {
                    pacman.push(package.to_lowercase());
                }
            }

            let mut status = 0;
            if !pacman.is_empty() {
                status = execute("sudo pacman -S ".to_owned() + pacman.join(" ").as_str()).code().unwrap();
            }
            if status == 0 {
                for package in aur {
                    let folder = dirs::cache_dir().unwrap().join("toru").join(&package);
                    std::fs::create_dir_all(&folder).unwrap();
                    let path = folder.as_path().to_str().unwrap();
                    if folder.read_dir().unwrap().next().is_none() {
                        execute("git clone https://aur.archlinux.org/".to_owned() + package.as_str() + ".git " + path);
                    } else {
                        execute(
                            "cd ".to_owned() + path + " && " +
                            "git pull"
                        );
                    }
                    execute(
                        "cd ".to_owned() + path + " && " +
                        "makepkg -si"
                    );
                }
            }
        },
        Some(("remove", _)) => {
            let packages: Vec<&str> = matches.subcommand_matches("remove").unwrap().values_of("PACKAGES").unwrap().collect();
            execute("sudo pacman -Rns ".to_owned() + packages.join(" ").as_str());
        },
        Some(("search", _)) => {
            println!("Coming Soon!");
        },
        Some(("update", _)) => {
            execute("sudo pacman -Syu --noconfirm".to_owned());
        },
        _ => {}
    }
}

#[derive(Serialize, Deserialize)]
struct Response {
    results: Vec<Package>
}

#[derive(Serialize, Deserialize)]
struct Package {
    #[serde(rename = "Name")]
    name: String
}

fn execute(command: String) -> ExitStatus {
    return Command::new("sh").arg("-c").arg(command).status().expect("");
}