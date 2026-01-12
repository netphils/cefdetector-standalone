use tauri::{AppHandle, Emitter};
use serde::Serialize;

use std::path::Path;
use walkdir::WalkDir;
use tauri::Error;

use windows_icons::get_icon_base64_by_path;

use crate::utils::common::*;

const ERROR_IMAGE : &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAACXBIWXMAAAsTAAALEwEAmpwYAAADm0lEQVR4nO2bPWgUQRSAv0TQ/GACIoiVgprS0kpQwcr4g/EHlVzAQlQEyanxF8KBojaKjYSLSUBFGwtBsQrBJkRLGwVjooU2WiQRRYmiJwOz4WVudm8v3mb3NvPgwTJvZ96+b3dm583OQrE0AruAU8C5lGgW2Ak0ECA1QBcwBRRSqpPAaR3rLKkFHibgAudLH5gQuowTxoA8cD0lmtcxyRhVF5/p8/KxvwMsIX2iYuo3ukMDenDwCt+lNHhPVGzjIt7tqvCMKOgh/ZI3u0FOFKhjc3DMiNdJRpeFtSdRiuLNBQDosIygmTLsVQ8gYwmwvQx71QOo1QF5r5N2SxcIslc9gDSKA4ADgC+AwQRkblHrYBCAwgLTnAPAbADDCcjcotbhIAA50i/uLYADgAOAA4AD4J4AHICctSDlEimARuA2MAA0B5zXAlwFnmpVx+vm4K8VeAzsTgqAHtFWp885F4Hflnm5Kjtfhq9VwE9dV33XiB1AixGYra0TIRKUoyH93TXqxQ7gkS3TEqK6xFdhHwEOAAeBF8YXm6YSvtYDf5IEYAPwtwSA/cL2FqgTNnU8Kuz7Svh7ZnlyYgXw3C/XFnJB2G5Y2rgp7EFjwSafrhMbgG1Biw1Bjsu0ezKSJAC1wCtRfzpiAHt8fMUGoEPU/QH0RQhgEfBGnHMrbgCLjU/N10q09b8Ajgn7N2BF3ACyot4EsCxCAPXAR2Hv1uWxAVgKfBb11B4DIgRwSdi+aP+xArgi6nwS28+iALDc2MKjZpPECWAl8F3UORwyyLkCkPOD93rsiRVApzj/tR6dowQg776aNhM3gJw4/4P+1ubpuDHdbasAABnkkOFP2gZC5BAVAdBtOA5S9XawTYXVYx12KmzmGEF6cj4AbPHJ522q7pItGRo1kqF6vUXPs+8VtqGQvtQ1bZ4PAEpWA1stek+0lTcGrGajP6sU+JDWl8ZT02Ts7dvo408CUNcU63pAmLaOh7iTRwgvsQyCYWeHfktiZ4FflsCnxYQqrEyJRZREAGjW+3H7SozIa/Vk6onWy8CaOfhr04uqiVkUrQZxAHAAcABwAHAA3BOAA5BD/0cn5+9pl14Rb1YV7BAFYyn/aapOryp58bai1/MmRWF/SiHU6UUTmXXWe0bZDQp6ZSefgJ3dldJe484XJWs1+nfSwgLR+7b/h2v0f3SyO6RNJ/SdLwoeIWpMUAOjGiHj3tldKVWxqL9EZ/q8J/8AgrX3iTePfdwAAAAASUVORK5CYII=";

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct FrontendBrowserInfo {
    display_name: String,
    size: u64,
    browser_type: String,
    icon: String
}

// 同步函数，验证应用是否为浏览器并计算大小
pub fn detect_browser_apps(mut apps: Vec<BrowserInfo>, app_handle: AppHandle) -> Vec<BrowserInfo> {
    for app in apps.iter_mut() {
        // 检查是否为浏览器应用
        app.is_browser = is_browser_application(&app.install_location);
        
        if app.is_browser {
            // 计算安装文件夹大小
            app.size = calculate_folder_size(&app.install_location);
            
            // 检测浏览器类型
            let browser_type = detect_browser_type(&app.install_location);
            
            // 尝试读取图标 base64
            let browser_icon: String = get_icon_base64(&app.display_icon).unwrap_or(ERROR_IMAGE.to_string());

            // 构造发送到前端的数据
            let frontend_info = FrontendBrowserInfo {
                display_name: app.display_name.clone(),
                size: app.size,
                browser_type,
                icon: browser_icon
            };
            
            // 发送到前端
            if let Err(e) = send_browser_to_frontend(&app_handle, frontend_info) {
                eprintln!("发送浏览器信息到前端失败: {:?}", e);
            }
        }
    }
    
    apps
}

// 检查是否为浏览器应用
fn is_browser_application(install_location: &str) -> bool {
    let path = Path::new(install_location);
    if !path.exists() || !path.is_dir() {
        return false;
    }
    
    // 检查是否存在浏览器相关的特征文件
    let browser_patterns = vec![
        "libcef", "libcef.dll", "cef.dll", "cef.pak",               // libcef相关
        "electron", "electron.exe", "electron.asar", "app.asar",    // Electron相关
        "nwjs", "nw.exe", "nwjc.exe",                               // NWJS相关
        "CefSharp.BrowserSubprocess.exe", "CefSharp.dll",           // CefSharp相关
        "miniblink", "node.dll", "miniblink.dll",                   // MiniBlink相关
        "chrome", "chromium"                                       // Chrome/Chromium核心
    ];
    
    // 遍历目录查找特征文件
    for entry in WalkDir::new(path)
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_string_lossy().to_lowercase();
            
            // 检查文件名是否包含浏览器特征
            for pattern in &browser_patterns {
                if file_name.contains(&pattern.to_lowercase()) {
                    return true;
                }
            }
            
            // 检查文件扩展名
            if let Some(ext) = entry.path().extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ext_str == "asar" || ext_str == "pak" || ext_str == "dat" {
                    // 常见浏览器资源文件扩展名
                    return true;
                }
            }
        }
    }
    
    false
}

// 检测浏览器类型
fn detect_browser_type(install_location: &str) -> String {
    let path: &Path = Path::new(install_location);
    if !path.exists() {
        return String::from("未知");
    }
    
    // 遍历文件查找特定特征
    for entry in WalkDir::new(path)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_string_lossy().to_lowercase();
            
            if file_name.contains("libcef.dll") || file_name.contains("cef.dll") {
                return String::from("libcef");
            } else if file_name.contains("electron.exe") || file_name.contains("electron.asar") || file_name.contains("apps.asar") {
                return String::from("Electron");
            } else if file_name.contains("nw.exe") || file_name.contains("nwjs") {
                return String::from("NWJS");
            } else if file_name.contains("cefsharp.dll") {
                return String::from("CefSharp");
            } else if file_name.contains("miniblink.dll") || file_name.contains("node.dll") {
                return String::from("MiniBlink");
            } else if file_name.contains("chrome.exe") || file_name.contains("chromium") {
                return String::from("Chrome/Chromium");
            } else if file_name.contains("msedge.exe") {
                return String::from("Edge");
            } else if file_name.contains("firefox.exe") {
                return String::from("Firefox");
            }
        }
    }
    
    String::from("其他浏览器")
}

// 计算文件夹大小
fn calculate_folder_size(path: &str) -> u64 {
    let mut total_size: u64 = 0;
    let dir_path = Path::new(path);
    
    if !dir_path.exists() || !dir_path.is_dir() {
        return 0;
    }
    
    // 使用walkdir递归遍历文件夹
    for entry in WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                total_size += metadata.len();
            }
        }
    }
    
    total_size
}

pub fn get_icon_base64(file_path: &str) -> Option<String> {

    match get_icon_base64_by_path(file_path) {
        Ok(base64_string) => {
            // 检查并确保返回的字符串包含数据URI前缀
            let result = if base64_string.starts_with("data:image/") {
                // 如果已经包含数据URI前缀，直接返回
                base64_string
            } else {
                // 没有前缀就添加一个png
                format!("data:image/png;base64,{}", base64_string)
            };
            
            Some(result)
        }
        Err(e) => {
            eprintln!("获取图标base64失败: {:?}", e);
            None
        }
    }
}

// 发送进度信息到前端
fn send_browser_to_frontend(
    apphandle : &AppHandle, 
    info : FrontendBrowserInfo
) -> Result<(), Error> {
        // 发送 Global 事件
    apphandle.emit("detection-started", info)
}
