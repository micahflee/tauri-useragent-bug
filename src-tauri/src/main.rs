#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{LogicalPosition, LogicalSize, Manager, WebviewUrl};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let width = 800.;
            let height = 600.;
            let window = app.get_window("main").unwrap();
            let _ = window.add_child(
                tauri::webview::WebviewBuilder::new(
                    "main3",
                    WebviewUrl::External("https://www.whatsmyua.info/".parse().unwrap()),
                )
                .auto_resize(),
                LogicalPosition::new(0., height / 2.),
                LogicalSize::new(width, height / 2.),
            )?;
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
