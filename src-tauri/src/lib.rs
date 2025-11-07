mod style_extractor;

use style_extractor::{extract_styles, fetch_website_blocking, generate_markdown};

#[tauri::command]
fn extract_website_styles(url: String) -> Result<String, String> {
    // Fetch the website HTML
    let html = fetch_website_blocking(&url)
        .map_err(|e| format!("Failed to fetch website: {}", e))?;
    
    // Extract styles from HTML
    let style_guide = extract_styles(&html, &url)
        .map_err(|e| format!("Failed to extract styles: {}", e))?;
    
    // Generate markdown
    let markdown = generate_markdown(&style_guide);
    
    Ok(markdown)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![extract_website_styles])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
