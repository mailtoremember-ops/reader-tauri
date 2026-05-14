// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_persisted_scope::init()) // <--- התיקון נמצא כאן! (init במקום Builder)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
