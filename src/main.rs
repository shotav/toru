mod aur;
mod clean;
mod install;
mod remove;
mod update;

use clap::{Arg, App, SubCommand, AppSettings};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(SubCommand::with_name("clean")
            .about("Clean cache."))
        .subcommand(SubCommand::with_name("install")
            .about("Install the specified packages.")
            .arg(Arg::with_name("PACKAGES")
                .help("Packages to install.")
                .multiple(true)
                .required(true)))
        .subcommand(SubCommand::with_name("remove")
            .about("Remove the specified packages.")
            .arg(Arg::with_name("PACKAGES")
                .help("Packages to remove.")
                .multiple(true)
                .required(true)))
        .subcommand(SubCommand::with_name("update")
            .about("Update all packages."))
        .get_matches();

    match matches.subcommand_name() {
        Some("clean") => clean::init(),
        Some("install") => install::init(matches),
        Some("remove") => remove::init(matches),
        Some("update") => update::init(),
        _ => {}
    }
}