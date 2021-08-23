pub fn init() {
    let folder = dirs::cache_dir().unwrap().join("toru");
    if folder.exists() {
        rm_rf::remove(folder).unwrap();
    }
}