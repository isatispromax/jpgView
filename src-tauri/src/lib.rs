use std::fs;
use tauri_plugin_fs;

#[tauri::command]
fn get_images_from_directory(img_dir: String) -> Result<Vec<String>, String> {
    //let img_dir = "./img"; // 假设图片存放在 Vue 项目中的 assets/img 目录
    let paths = fs::read_dir(img_dir)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map(|ext| ext == "jpg" || ext == "png")
                .unwrap_or(false)
        })
        .map(|entry| entry.path().to_string_lossy().to_string())
        .collect::<Vec<String>>();

    if paths.is_empty() {
        Err("No images found".to_string())
    } else {
        Ok(paths)
    }
}

#[tauri::command]
fn read_config_file_context(file_path: String) -> String {
    // 读取 config_file的文件内容，并将内容返回
    //let resolved_path = resolve_path(&file_path)?;
    let content = fs::read_to_string(file_path);
    content
}

#[tauri::command]
fn save_config_file(file_path: String, content: String) {
    //let resolved_path = resolve_path(&file_path)?;
    fs::write(file_path, content);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            get_images_from_directory,
            read_config_file_context,
            save_config_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
