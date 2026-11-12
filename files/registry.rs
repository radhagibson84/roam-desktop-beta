//! Windows registry read helpers for Roam Desktop Beta.

pub const APP_KEY: &str = "Software\\Roam\\Roam Desktop Beta";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Roam Desktop Beta";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Roam Desktop Beta") },
        RegValue { name: "Version",     value: String::from("v4.1.17") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Roam Desktop Beta") },
        RegValue { name: "Publisher",       value: String::from("Roam") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Roam Desktop Beta\\uninstall.exe\"") },
    ]
}
