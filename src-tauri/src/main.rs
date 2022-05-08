#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start, stop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start(capture: String) -> String {
    println!("Starting Recording... {}", capture);
    "200".into()
}

#[tauri::command]
fn stop() -> String {
    println!("Stopping Recording...");
    "200".into()
}
