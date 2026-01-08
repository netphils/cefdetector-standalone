use winreg::{enums::*, RegKey};
use std::path::Path;

use crate::utils::common::*;

pub fn get_installed_apps() -> Vec<BrowserInfo> {
    let mut apps = Vec::new();
    
    // 要检查的注册表路径
    let paths = vec![
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall", true),  // 64位
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall", false),  // 32位
        (HKEY_CURRENT_USER, r"Software\Microsoft\Windows\CurrentVersion\Uninstall", false),  // 用户程序
    ];

    for (hive, subkey, is_64bit) in paths {
        if let Ok(hkey) = RegKey::predef(hive).open_subkey(subkey) {
            for subkey_name in hkey.enum_keys().filter_map(Result::ok) {
                if let Ok(subkey) = hkey.open_subkey(&subkey_name) {
                    if let Ok(display_name) = subkey.get_value::<String, _>("DisplayName") {
                        // 获取安装路径
                        let install_location = subkey.get_value::<String, _>("InstallLocation")
                            .unwrap_or_default();
                            
                        // 获取显示图标
                        let display_icon = subkey.get_value::<String, _>("DisplayIcon")
                            .unwrap_or_default();
                            
                        // 获取卸载字符串
                        let uninstall_string = subkey.get_value::<String, _>("UninstallString")
                            .or_else(|_| subkey.get_value::<String, _>("QuietUninstallString"))
                            .unwrap_or_default();
                        
                        // 如果安装路径为空，尝试从其他位置获取
                        let install_location = if install_location.is_empty() {
                            subkey.get_value::<String, _>("InstallSource")
                                .or_else(|_| subkey.get_value::<String, _>("URLInfoAbout"))
                                .unwrap_or_default()
                        } else {
                            install_location
                        };

                        // 检查是否是有效的安装位置
                        let valid_location = if !install_location.is_empty() {
                            Path::new(&install_location).exists()
                        } else {
                            false
                        };

                        // 添加到结果列表
                        apps.push(BrowserInfo {
                            display_name,
                            install_location: if valid_location { install_location } else { String::new() },
                            display_icon,
                            uninstall_string,
                            size: 0,  // 后续计算
                            is_browser: false,  // 后续验证
                        });
                    }
                }
            }
        }
    }
    
    apps
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[cfg(windows)]
    fn test_get_installed_apps_basic() {
        let apps = get_installed_apps();
        
        // 基本验证：应该能获取到一些程序
        assert!(!apps.is_empty(), "应该至少获取到一个已安装程序");
        
        // 检查一些基本属性
        for app in apps.iter().take(5) {  // 只检查前5个
            assert!(!app.display_name.is_empty(), "程序名不应为空");
            // 其他字段可能为空，但display_name必须有
        }
        
        println!("获取到 {} 个已安装程序", apps.len());
    }
    
    #[test]
    #[cfg(windows)]
    fn test_get_installed_apps_structure() {
        let apps = get_installed_apps();
        
        // 查找一些常见程序（如果存在）
        let common_names = ["Chrome", "Firefox", "Edge", "Opera"];
        let mut found_any = false;
        
        for app in &apps {
            for name in &common_names {
                if app.display_name.contains(name) {
                    found_any = true;
                    break;
                }
            }
        }
        
        // 不要求一定找到，但可以记录
        if found_any {
            println!("找到了常见浏览器");
        }
    }
}