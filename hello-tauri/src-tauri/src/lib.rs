use std::fs;

#[tauri::command]
fn open(file_path: String) -> String {
    match fs::read_to_string(file_path) {
        Ok(c) => c.into(),
        Err(e) => format!("Error: {}", e).into(),
    }
}

#[tauri::command]
fn save(file_path: String, contents: String) -> Result<(), String> {
    match fs::write(&file_path, contents) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![open, save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
