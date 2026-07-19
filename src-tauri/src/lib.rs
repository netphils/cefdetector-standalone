// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use tauri::Emitter;
use walkdir::WalkDir;
use winreg::enums::*;
use winreg::RegKey;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct AppEntry {
    icon: String,
    name: String,
    app_type: String,
    size: String,
}

const UNINSTALL_KEYS: &[&str] = &[
    "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
    "HKEY_LOCAL_MACHINE\\SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
    "HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
];

const BROWSER_MATCH: &[(&str, &[&str])] = &[
    ("Electron", &["electron.exe", "app.asar", "node.dll"]),
    (
        "CEF",
        &[
            "libcef.dll",
            "chrome_100_percent.pak",
            "chrome_200_percent.pak",
            "resources.pak",
        ],
    ),
];

fn predef_from_root(root: &str) -> Option<RegKey> {
    match root {
        "HKEY_LOCAL_MACHINE" => Some(RegKey::predef(HKEY_LOCAL_MACHINE)),
        "HKEY_CURRENT_USER" => Some(RegKey::predef(HKEY_CURRENT_USER)),
        _ => None,
    }
}

fn read_uninstall_entries() -> HashMap<String, HashMap<String, String>> {
    let mut apps: HashMap<String, HashMap<String, String>> = HashMap::new();

    for full in UNINSTALL_KEYS {
        let (root, sub) = match full.split_once('\\') {
            Some(pair) => pair,
            None => continue,
        };
        let hkey = match predef_from_root(root) {
            Some(k) => k,
            None => continue,
        };
        let key = match hkey.open_subkey(sub) {
            Ok(k) => k,
            Err(_) => continue,
        };

        for subkey_name in key.enum_keys().filter_map(|r| r.ok()) {
            let subkey = match key.open_subkey(&subkey_name) {
                Ok(k) => k,
                Err(_) => continue,
            };

            let mut values: HashMap<String, String> = HashMap::new();
            for (value_name, _) in subkey.enum_values().filter_map(|r| r.ok()) {
                if let Ok(v) = subkey.get_value::<String, _>(&value_name) {
                    values.insert(value_name, v);
                }
            }

            let key_name = if let Some(display) = values.get("DisplayName") {
                display.clone()
            } else {
                subkey_name.clone()
            };
            apps.insert(key_name, values);
        }
    }

    apps
}

fn install_dir(values: &HashMap<String, String>) -> Option<PathBuf> {
    if let Some(loc) = values.get("InstallLocation") {
        if !loc.trim().is_empty() {
            return Some(PathBuf::from(loc.trim()));
        }
    }
    if let Some(uninstall) = values.get("UninstallString") {
        if !uninstall.trim().is_empty() {
            let path = Path::new(uninstall.trim());
            if let Some(parent) = path.parent() {
                return Some(parent.to_path_buf());
            }
        }
    }
    None
}

fn matches_browser(dir: &Path) -> Option<&'static str> {
    for entry in WalkDir::new(dir).into_iter().filter_map(|r| r.ok()) {
        let file_name = match entry.file_name().to_str() {
            Some(n) => n.to_lowercase(),
            None => continue,
        };
        for (browser_type, patterns) in BROWSER_MATCH {
            if patterns.iter().any(|p| p.to_lowercase() == file_name) {
                return Some(browser_type);
            }
        }
    }
    None
}

fn dir_size(dir: &Path) -> u64 {
    let mut total = 0u64;
    for entry in WalkDir::new(dir).into_iter().filter_map(|r| r.ok()) {
        if let Ok(meta) = entry.metadata() {
            if meta.is_file() {
                total += meta.len();
            }
        }
    }
    total
}

fn icon_base64(values: &HashMap<String, String>) -> String {
    if let Some(display_icon) = values.get("DisplayIcon") {
        let path_part = display_icon.split(',').next().unwrap_or("").trim();
        if !path_part.is_empty() {
            if let Ok(b64) = windows_icons::get_icon_base64_by_path(path_part) {
                return b64;
            }
        }
    }
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+M8AAAMBAQDJ/pLvAAAAAElFTkSuQmCC".to_string()
}

#[tauri::command]
async fn scan_apps(
    app: tauri::AppHandle,
    _params: HashMap<String, String>,
) -> Option<()> {
    let apps = read_uninstall_entries();

    for values in apps.values() {
        if !values.contains_key("DisplayName")
            || !values.contains_key("DisplayIcon")
            || !values.contains_key("InstallLocation")
            || !values.contains_key("UninstallString")
        {
            continue;
        }

        let name = match values.get("DisplayName") {
            Some(n) if !n.trim().is_empty() => n.clone(),
            _ => continue,
        };

        let dir = match install_dir(values) {
            Some(d) => d,
            None => continue,
        };
        if !dir.exists() {
            continue;
        }

        let browser_type = match matches_browser(&dir) {
            Some(t) => t.to_string(),
            None => continue,
        };

        let size_bytes = dir_size(&dir);
        let size = humansize::format_size(size_bytes, humansize::BINARY);

        let icon = icon_base64(values);

        let entry = AppEntry {
            icon,
            name,
            app_type: browser_type,
            size,
        };
        app.emit("app-entry", entry).ok();

        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    }

    None
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan_apps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
