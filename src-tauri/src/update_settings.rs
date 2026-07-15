use ini::Ini;
use std::path::Path;

#[tauri::command]
pub fn update_settings(key: String, value: String) -> Result<(), String> {
    let path = "settings.ini";
    let mut conf = if Path::new(path).exists() {
        Ini::load_from_file(path).map_err(|e| e.to_string())?
    } else {
        Ini::new()
    };

    conf.with_section(Some("Settings"))
        .set(key, value);

    conf.write_to_file(path).map_err(|e| e.to_string())?;
    Ok(())
}