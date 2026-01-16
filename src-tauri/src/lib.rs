mod utils;
use crate::utils::get_installed_apps;
use crate::utils::detect_browser_apps;

#[tauri::command]
fn search_installed() -> usize {
    let apps = get_installed_apps();
    apps.len()
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct BrowserSummary {
    size: u64,
    count: usize,
}

#[tauri::command]
async fn search_browsers(app_handle: tauri::AppHandle) -> BrowserSummary {
    let apps = get_installed_apps();

    let browsers = detect_browser_apps(apps, app_handle);

    let count: usize = browsers
    .iter()
    .filter(|info| info.is_browser)
    .count();

    let total_size: u64 = browsers.iter()
    .filter(|browser| browser.is_browser)
    .map(|browser| browser.size)
    .sum();

    BrowserSummary {
        size: total_size,
        count: count
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
                greet,
                search_installed,
                search_browsers
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
