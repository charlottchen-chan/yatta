// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn rockstar_games_save_ip_block() {
    let _output = std::process::Command::new("netsh")
        .args([
            "advfirewall",
            "firefwall",
            "add",
            "rule",
            "name=BlockRockstarSaveIp",
            "dir=in",
            "action=block",
            "remoteip=192.81.241.171",
        ])
        .output()
        .map_err(|e| e.to_string());
}

#[tauri::command]
fn alwaysontop() {
    
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![rockstar_games_save_ip_block])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
