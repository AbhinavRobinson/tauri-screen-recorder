#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

static mut RECORDING: bool = false;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start, stop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start(capture: String) -> String {
    unsafe {
        if RECORDING {
            println!("Already Recording...");
            "400".into()
        } else {
            println!("Starting Recording... {}", capture);
            RECORDING = true;
            "200".into()
        }
    }
}

#[tauri::command]
fn stop() -> String {
    println!("Stopping Recording...");
    unsafe { RECORDING = false };
    "200".into()
}
