pub fn init() {
    std::fs::remove_dir_all(dirs::cache_dir().unwrap().join("toru")).unwrap();
}