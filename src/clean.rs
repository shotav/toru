pub fn init() {
    let folder = dirs::cache_dir().unwrap().join("toru");
    if folder.exists() {
        std::fs::remove_dir_all(folder).unwrap();
    }
}