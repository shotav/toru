use std::process::Command;

pub fn init() {
    let output = Command::new("pacman").arg("-Qtdq").output().unwrap();
    let lines = String::from_utf8(output.stdout).unwrap();
    let packages: Vec<&str> = lines.lines().collect();
    if !packages.is_empty() {
        let status = Command::new("sudo").arg("pacman").args(["-Rns", "--noconfirm"]).args(packages).status().unwrap();
        if !status.success() {
            return;
        }
    }

    let folder = dirs::cache_dir().unwrap().join("toru");
    if folder.exists() {
        rm_rf::remove(folder).unwrap();
    }
}