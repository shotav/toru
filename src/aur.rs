use std::process::Command;
use nix::unistd::Uid;
use git2::Repository;

pub fn install(package: String) {
    if !Uid::effective().is_root() {
        let folder = dirs::cache_dir().unwrap().join("toru").join(&package);
        if folder.exists() {
            rm_rf::remove(&folder).unwrap();
        }
        std::fs::create_dir_all(&folder).unwrap();
        Repository::clone(format!("https://aur.archlinux.org/{}.git", package).as_str(), &folder).unwrap();
        Command::new("makepkg").arg("-si").arg("--noconfirm").current_dir(folder).status().unwrap();
    }
}