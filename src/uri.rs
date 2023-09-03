use std::{env::current_exe, path::Path};

use winreg::{RegKey, enums::HKEY_CLASSES_ROOT};

pub fn register() {
    let path = current_exe().unwrap();
    let path = ["C:", &path.to_string_lossy().split("C:").nth(1).unwrap()].concat();
    let hkcu = RegKey::predef(HKEY_CLASSES_ROOT);
    hkcu.create_subkey(Path::new("ndnd").join("shell").join("open").join("command")).unwrap()
        .0.set_value("", &format!("\"{path}\" \"%1\"")).unwrap();
    hkcu.create_subkey(Path::new("ndnd")).unwrap()
        .0.set_value("URL Protocol", &"").unwrap();
}