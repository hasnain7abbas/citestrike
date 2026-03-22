use crate::citation::{CitationStyle, format_citation};
use crate::crossref;
use crate::db::{self, NewReference, Reference};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

pub struct DbState(pub Mutex<Connection>);

#[tauri::command]
pub fn search_references(query: &str, state: State<DbState>) -> Result<Vec<Reference>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    if query.trim().is_empty() {
        db::get_all_refs(&conn).map_err(|e| e.to_string())
    } else {
        db::search_refs(&conn, query).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn add_reference(new_ref: NewReference, state: State<DbState>) -> Result<Reference, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::insert_ref(&conn, &new_ref).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_reference(id: &str, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::delete_ref(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn format_ref(reference: Reference, style: CitationStyle) -> String {
    format_citation(&reference, style)
}

#[tauri::command]
pub async fn fetch_doi(doi: String) -> Result<NewReference, String> {
    crossref::fetch_by_doi(&doi).await
}

#[tauri::command]
pub async fn search_online(query: String) -> Result<Vec<NewReference>, String> {
    crossref::search_crossref(&query).await
}

#[tauri::command]
pub fn import_pdf(path: String) -> Result<String, String> {
    let path = std::path::Path::new(&path);
    crate::pdf::extract_doi_from_pdf(path)
}
