use clap::{Arg, App, AppSettings};
use nix::unistd::Uid;
use std::process::Command;

fn main() {
    let matches = App::new("toru")
        .version("0.1.0")
        .author("PryosCode")
        .about("Pacman wrapper with AUR support.")
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(App::new("install")
            .about("Install the specified packages.")
            .arg(Arg::new("PACKAGES")
                .about("")
                .multiple_values(true)
                .required(true))
            .arg(Arg::new("yes")
                .short('y')
                .long("yes")
                .multiple_occurrences(true)
                .about("Accept automatically.")))
        .subcommand(App::new("remove")
            .about("Remove the specified packages.")
            .arg(Arg::new("PACKAGES")
                .about("")
                .multiple_values(true)
                .required(true))
            .arg(Arg::new("yes")
                .short('y')
                .long("yes")
                .multiple_occurrences(true)
                .about("Accept automatically.")))
        .subcommand(App::new("update")
            .about("Update all packages.")
            .arg(Arg::new("yes")
                .short('y')
                .long("yes")
                .multiple_occurrences(true)
                .about("Accept automatically.")))
        .subcommand(App::new("clean")
            .about("Clean cache."))
        .get_matches();

    if Uid::effective().is_root() {
        println!("Please avoid running toru with root.");
        return;
    }

    match matches.subcommand() {
        Some(("install", _)) => {
            let packages: Vec<&str> = matches.subcommand_matches("install").unwrap().values_of("PACKAGES").unwrap().collect();
            Command::new("sh").arg("-c").arg("sudo pacman -S ".to_owned() + packages.join(" ").as_str()).status().expect("");
        },
        Some(("remove", _)) => {
            let packages: Vec<&str> = matches.subcommand_matches("remove").unwrap().values_of("PACKAGES").unwrap().collect();
            Command::new("sh").arg("-c").arg("sudo pacman -Rns ".to_owned() + packages.join(" ").as_str()).status().expect("");
        },
        Some(("update", _)) => {
            Command::new("sh").arg("-c").arg("sudo pacman -Syu --noconfirm").status().expect("");
        },
        _ => {}
    }
}