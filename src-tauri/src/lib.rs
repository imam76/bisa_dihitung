mod config;
mod database;
mod repository;
mod commands;

use config::DatabaseConfig;
use database::Database;
use commands::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

    rt.block_on(async {
        let config = DatabaseConfig::from_env();
        let db = Database::new(config)
            .await
            .expect("Failed to initialize database");

        tauri::Builder::default()
            .plugin(tauri_plugin_opener::init())
            .manage(db)
            .invoke_handler(tauri::generate_handler![
                greet,
                create_user,
                get_all_users,
                get_user,
                update_user,
                delete_user
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    });
}
