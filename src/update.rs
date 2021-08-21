pub fn init() {
    crate::lib::execute("sudo pacman -Syu".to_owned());
}