use sysinfo::System;
use std::process::Command;
use serde::Serialize;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager, Runtime, State,
};
use std::sync::Mutex;

// Simple session state - just stores if user is logged in
pub struct SessionState {
    token: Mutex<Option<String>>,
}

#[derive(Default, Debug, Serialize)]
pub struct Dmi {
    pub product_name: String,
    pub manufacturer: String,
    pub serial_number: String,
    pub uuid: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_dmi_report() -> serde_json::Value {
    let mut dmi = Dmi::default();
    fill_os_specific_info(&mut dmi);

    serde_json::json!({
        // static methods return Option<String>
        "os_name": System::name().unwrap_or_default(),
        "os_version": System::os_version().unwrap_or_default(),
        "kernel_version": System::kernel_version().unwrap_or_default(),
        "host_name": System::host_name().unwrap_or_default(),
        // cpu_arch() returns String directly in 0.37.x
        "cpu_arch": System::cpu_arch(), 
        "hardware": {
            "product_name": dmi.product_name,
            "manufacturer": dmi.manufacturer,
            "serial_number": dmi.serial_number,
            "uuid": dmi.uuid,
        }
    })
}

// --- Session Commands ---

#[tauri::command]
fn is_logged_in(state: State<'_, SessionState>) -> bool {
    state.token.lock().unwrap().is_some()
}

#[tauri::command]
fn login(state: State<'_, SessionState>, username: String, password: String) -> Result<bool, String> {
    // For now, just accept any login (we'll add real auth later)
    if !username.is_empty() && !password.is_empty() {
        *state.token.lock().unwrap() = Some(format!("token_{}", username));
        Ok(true)
    } else {
        Err("Invalid credentials".to_string())
    }
}

#[tauri::command]
fn logout(state: State<'_, SessionState>) {
    *state.token.lock().unwrap() = None;
}

// --- OS Specific Implementations ---

#[cfg(target_os = "linux")]
fn fill_os_specific_info(dmi: &mut Dmi) {
    use std::fs;
    let read_dmi = |path: &str| fs::read_to_string(path).unwrap_or_default().trim().to_string();
    dmi.product_name = read_dmi("/sys/class/dmi/id/product_name");
    dmi.manufacturer = read_dmi("/sys/class/dmi/id/sys_vendor");
    dmi.serial_number = read_dmi("/sys/class/dmi/id/product_serial");
    dmi.uuid = read_dmi("/sys/class/dmi/id/product_uuid");
}

#[cfg(target_os = "windows")]
fn fill_os_specific_info(dmi: &mut Dmi) {
    let get_wmic = |category: &str, field: &str| {
        Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!("Get-CimInstance {} | Select-Object -ExpandProperty {}", category, field),
            ])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default()
    };
    dmi.product_name = get_wmic("Win32_ComputerSystemProduct", "Name");
    dmi.manufacturer = get_wmic("Win32_ComputerSystemProduct", "Vendor");
    dmi.serial_number = get_wmic("Win32_Bios", "SerialNumber");
    dmi.uuid = get_wmic("Win32_ComputerSystemProduct", "IdentifyingNumber");
}

#[cfg(target_os = "macos")]
fn fill_os_specific_info(dmi: &mut Dmi) {
    let output = Command::new("system_profiler")
        .arg("SPHardwareDataType")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    for line in output.lines() {
        if line.contains("Model Name:") {
            dmi.product_name = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.contains("Serial Number (system):") {
            dmi.serial_number = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.contains("Hardware UUID:") {
            dmi.uuid = line.split(':').nth(1).unwrap_or("").trim().to_string();
        }
    }
    dmi.manufacturer = "Apple Inc.".to_string();
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
fn fill_os_specific_info(_dmi: &mut Dmi) {}

// --- System Tray Setup ---

fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    // Create menu items
    let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    
    // Build the menu
    let menu = Menu::with_items(app, &[&show_item, &quit_item])?;
    
    // Build the tray icon
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("Cubiq Care Monitor")
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                ..
            } = event
            {
                // Show window on left click
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;
    
    Ok(())
}

// --- The Run Function with Tray ---

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(SessionState {
            token: Mutex::new(None),
        })
        .setup(|app| {
            // Create the system tray
            create_tray(app.handle())?;
            
            // Handle window close event to minimize to tray instead of closing
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // Prevent the default close and hide instead
                        api.prevent_close();
                        let _ = window_clone.hide();
                    }
                });
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet, 
            get_dmi_report,
            is_logged_in,
            login,
            logout,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}