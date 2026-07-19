// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Emitter;
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

fn predef_from_root(root: &str) -> Option<RegKey> {
    match root {
        "HKEY_LOCAL_MACHINE" => Some(RegKey::predef(HKEY_LOCAL_MACHINE)),
        "HKEY_CURRENT_USER" => Some(RegKey::predef(HKEY_CURRENT_USER)),
        _ => None,
    }
}

#[tauri::command]
async fn scan_apps(app: tauri::AppHandle, _params: std::collections::HashMap<String, String>) -> Option<()> {
    let samples = [
        ("Chrome", "Electron", "248.6 MB"),
        ("Discord", "Electron", "412.3 MB"),
        ("Slack", "Electron", "301.9 MB"),
        ("Spotify", "CEF", "187.4 MB"),
        ("VS Code", "Electron", "356.1 MB"),
    ];

    // 1x1 transparent PNG as placeholder base64 icon
    let placeholder_icon = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+M8AAAMBAQDJ/pLvAAAAAElFTkSuQmCC".to_string();

    for (i, (name, app_type, size)) in samples.iter().enumerate() {
        let entry = AppEntry {
            icon: placeholder_icon.clone(),
            name: name.to_string(),
            app_type: app_type.to_string(),
            size: size.to_string(),
        };
        app.emit("app-entry", entry).unwrap();
        if i < samples.len() - 1 {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
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
