// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ffi::ffi_loop;

pub mod ffi;

fn main() {
    ffi_loop();
    // tauri_screen_recorder_lib::run()
}
