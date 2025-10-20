use std::path::PathBuf;

pub fn get_local_package_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Cannot find home dir");
    if cfg!(target_os = "linux") {
        std::env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join(".local/share/typst/packages"))
    } else if cfg!(target_os = "macos") {
        let dir1 = home.join(".local/share/typst/packages");
        if dir1.exists() {
            dir1
        } else {
            home.join("Library/Application Support/typst/packages")
        }
    } else if cfg!(target_os = "windows") {
        std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join("AppData/Roaming"))
            .join("typst/packages")
    } else {
        panic!("Unsupported OS");
    }
}
