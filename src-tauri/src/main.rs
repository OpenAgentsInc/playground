// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use extism::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let url = Wasm::url(
        "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm"
    );
    let manifest = Manifest::new([url]);
    let mut plugin = Plugin::new(&manifest, [], true).unwrap();
    let res = plugin.call::<&str, &str>("count_vowels", "Hello, world!").unwrap();
    format!("Hello, {}! Your name has vowels: {}", name, res)
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
