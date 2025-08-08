// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command



#[cfg_attr(mobile, tauri::mobile_entry_point)]


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}