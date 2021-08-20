use clap::{Arg, App, AppSettings};
use std::process::Command;

fn main() {
    let matches = App::new("toru")
        .version("0.1.2")
        .author("PryosCode")
        .about("Pacman wrapper with AUR support.")
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
        },
        Some(("install", _)) => {
            let packages: Vec<&str> = matches.subcommand_matches("install").unwrap().values_of("PACKAGES").unwrap().collect();
            Command::new("sh").arg("-c").arg("sudo pacman -S ".to_owned() + packages.join(" ").as_str()).status().expect("");
        },
        Some(("remove", _)) => {
            let packages: Vec<&str> = matches.subcommand_matches("remove").unwrap().values_of("PACKAGES").unwrap().collect();
            Command::new("sh").arg("-c").arg("sudo pacman -Rns ".to_owned() + packages.join(" ").as_str()).status().expect("");
        },
        Some(("search", _)) => {
        },
        Some(("update", _)) => {
            Command::new("sh").arg("-c").arg("sudo pacman -Syu --noconfirm").status().expect("");
        },
        _ => {}
    }
}