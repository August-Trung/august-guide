// main.rs — Tauri application entry point.
// Prevents an extra console window on Windows in release mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    august_guide_lib::run()
}
