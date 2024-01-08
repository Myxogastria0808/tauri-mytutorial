// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn simple_command() {
    println!("Hello, World!");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![simple_command,])
        .run(tauri::generate_context!())
        .expect("Error!");
}
