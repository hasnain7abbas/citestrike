mod api_server;
mod citation;
mod commands;
mod crossref;
mod db;
mod pdf;
mod rich_clipboard;
mod word_insert;

use commands::DbState;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                window.hide().ok();
                            } else {
                                window.show().ok();
                                window.center().ok();
                                window.set_focus().ok();
                            }
                        }
                    }
                })
                .build(),
        )
        .setup(|app| {
            // Initialize database
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&app_dir).ok();
            let db_path = app_dir.join("citestrike.db");
            let conn = Connection::open(&db_path).expect("failed to open database");
            db::init_db(&conn).expect("failed to init database");
            app.manage(DbState(Mutex::new(conn)));

            // Start Office add-in API server on port 27182
            let api_db_path = db_path.clone();
            tauri::async_runtime::spawn(async move {
                api_server::start_server(api_db_path).await;
            });

            // Setup logging in debug
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Register global shortcut
            use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
            let shortcut =
                Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyC);
            match app.handle().global_shortcut().register(shortcut) {
                Ok(_) => log::info!("Global shortcut Ctrl+Shift+C registered"),
                Err(e) => log::warn!("Failed to register global shortcut: {}", e),
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::search_references,
            commands::add_reference,
            commands::delete_reference,
            commands::move_reference,
            commands::format_ref,
            commands::fetch_doi,
            commands::search_online,
            commands::import_pdf,
            commands::create_folder,
            commands::get_folders,
            commands::rename_folder,
            commands::delete_folder,
            commands::get_ref_count,
            commands::format_inline_citation,
            commands::format_batch_bibliography,
            commands::copy_rich_citation,
            commands::copy_rich_inline,
            commands::copy_rich_bibliography,
            commands::insert_citation_into_word,
            commands::insert_inline_into_word,
            commands::insert_citation_into_ppt,
            commands::insert_bibliography_into_word,
            // New citation workflow commands
            commands::cite_reference,
            commands::uncite_reference,
            commands::get_cited_references,
            commands::copy_cited_bibliography,
            commands::reset_citations,
            commands::update_reference,
            commands::write_bibtex_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
