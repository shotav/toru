use clap::{ArgMatches};

pub fn init(matches: ArgMatches) {
    let packages: Vec<&str> = matches.subcommand_matches("remove").unwrap().values_of("PACKAGES").unwrap().collect();
    crate::lib::execute("sudo pacman -Rns ".to_owned() + packages.join(" ").as_str());
}