mod common;
mod installed_apps;
mod search_browsers;

pub use installed_apps::get_installed_apps;
pub use search_browsers::detect_browser_apps;