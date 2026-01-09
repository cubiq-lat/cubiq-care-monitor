// src-tauri/src/main.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    cubiq_care_monitor_lib::run()
}

// src-tauri/src/main.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::{
//     menu::{Menu, MenuItem},
//     tray::{TrayIconBuilder, TrayIconEvent},
//     Manager, Runtime,
// };

// fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
//     // Create menu items
//     let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
//     let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    
//     // Build the menu
//     let menu = Menu::with_items(app, &[&show_item, &quit_item])?;
    
//     // Build the tray icon
//     let _tray = TrayIconBuilder::new()
//         .menu(&menu)
//         .tooltip("Cubiq Care Monitor")
//         .icon(app.default_window_icon().unwrap().clone())
//         .on_tray_icon_event(|tray, event| {
//             if let TrayIconEvent::Click {
//                 button: tauri::tray::MouseButton::Left,
//                 ..
//             } = event
//             {
//                 // Show window on left click
//                 if let Some(window) = tray.app_handle().get_webview_window("main") {
//                     let _ = window.show();
//                     let _ = window.set_focus();
//                 }
//             }
//         })
//         .on_menu_event(|app, event| match event.id.as_ref() {
//             "show" => {
//                 if let Some(window) = app.get_webview_window("main") {
//                     let _ = window.show();
//                     let _ = window.set_focus();
//                 }
//             }
//             "quit" => {
//                 app.exit(0);
//             }
//             _ => {}
//         })
//         .build(app)?;
    
//     Ok(())
// }

// fn main() {
//     tauri::Builder::default()
//         .setup(|app| {
//             // Create the system tray
//             create_tray(app.handle())?;
            
//             // Handle window close event to minimize to tray instead of closing
//             if let Some(window) = app.get_webview_window("main") {
//                 window.on_window_event(|event| {
//                     if let tauri::WindowEvent::CloseRequested { api, .. } = event {
//                         // Prevent the default close and hide instead
//                         api.prevent_close();
//                         event.window().hide().unwrap();
//                     }
//                 });
//             }
            
//             Ok(())
//         })
//         .invoke_handler(tauri::generate_handler![])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }