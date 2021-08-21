use std::process::Command;

pub fn install(package: String) {
    let folder = dirs::cache_dir().unwrap().join("toru").join(&package);
    std::fs::create_dir_all(&folder).unwrap();
    if folder.read_dir().unwrap().next().is_none() {
        Command::new("git").arg("clone").arg("https://aur.archlinux.org/".to_owned() + package.as_str() + ".git").arg(".").current_dir(&folder).status().unwrap();
    } else {
        Command::new("git").arg("pull").current_dir(&folder).status().unwrap();
    }
    Command::new("makepkg").arg("-si").arg("--noconfirm").current_dir(folder).status().unwrap();
}