pub mod screenshot;

static mut RECORDING: bool = false;

#[tauri::command]
fn screenshot(capture: String) -> String {
    unsafe {
        if RECORDING {
            println!("Captured Screenshot...");
            "400".into()
        } else {
            println!("Capturing Screenshot... {}", capture);
            RECORDING = true;
            std::panic::catch_unwind(|| {
                screenshot::capture();
            })
            .unwrap_or_else(|_| {
                println!("Error while capturing...");
            });
            RECORDING = false;
            "200".into()
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![screenshot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
