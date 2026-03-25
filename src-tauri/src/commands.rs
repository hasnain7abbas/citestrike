use crate::citation::{CitationStyle, format_citation, format_inline, format_bibliography};
use crate::crossref;
use crate::db::{self, Folder, NewReference, Reference};
use crate::rich_clipboard;
use crate::word_insert;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

pub struct DbState(pub Mutex<Connection>);

#[tauri::command]
pub fn search_references(query: &str, folder_id: Option<&str>, state: State<DbState>) -> Result<Vec<Reference>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    if query.trim().is_empty() {
        db::get_all_refs(&conn, folder_id).map_err(|e| e.to_string())
    } else {
        db::search_refs(&conn, query, folder_id).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn add_reference(new_ref: NewReference, folder_id: Option<&str>, state: State<DbState>) -> Result<Reference, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::insert_ref(&conn, &new_ref, folder_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_reference(id: &str, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::delete_ref(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn move_reference(ref_id: &str, folder_id: Option<&str>, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::move_ref_to_folder(&conn, ref_id, folder_id).map_err(|e| e.to_string())
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

// Folder commands
#[tauri::command]
pub fn create_folder(name: &str, color: &str, state: State<DbState>) -> Result<Folder, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::create_folder(&conn, name, color).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_folders(state: State<DbState>) -> Result<Vec<Folder>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_folders(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn rename_folder(id: &str, name: &str, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::rename_folder(&conn, id, name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_folder(id: &str, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::delete_folder(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_ref_count(folder_id: Option<&str>, state: State<DbState>) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_ref_count(&conn, folder_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn format_inline_citation(reference: Reference, style: CitationStyle, number: Option<usize>) -> String {
    format_inline(&reference, style, number)
}

#[tauri::command]
pub fn format_batch_bibliography(references: Vec<Reference>, style: CitationStyle) -> String {
    format_bibliography(&references, style)
}

// --- Rich text clipboard commands ---

/// Copy a full citation as RTF (rich text) to clipboard — italics preserved in Word/PPT
#[tauri::command]
pub fn copy_rich_citation(reference: Reference, style: CitationStyle) -> Result<(), String> {
    let rtf = rich_clipboard::citation_to_rtf(&reference, style);
    let plain = format_citation(&reference, style);
    rich_clipboard::copy_rtf_to_clipboard(&rtf, &plain)
}

/// Copy an in-text citation as RTF to clipboard
#[tauri::command]
pub fn copy_rich_inline(reference: Reference, style: CitationStyle, number: Option<usize>) -> Result<(), String> {
    let plain = format_inline(&reference, style, number);
    let rtf = rich_clipboard::inline_to_rtf(&plain);
    rich_clipboard::copy_rtf_to_clipboard(&rtf, &plain)
}

/// Copy a batch bibliography as RTF to clipboard with proper formatting
#[tauri::command]
pub fn copy_rich_bibliography(references: Vec<Reference>, style: CitationStyle) -> Result<(), String> {
    let rtf = rich_clipboard::bibliography_to_rtf(&references, style);
    let plain = format_bibliography(&references, style);
    rich_clipboard::copy_rtf_to_clipboard(&rtf, &plain)
}

// --- Direct Word/PowerPoint insertion ---

/// Copy citation as RTF then paste directly into Word
#[tauri::command]
pub fn insert_citation_into_word(reference: Reference, style: CitationStyle) -> Result<String, String> {
    let rtf = rich_clipboard::citation_to_rtf(&reference, style);
    let plain = format_citation(&reference, style);
    rich_clipboard::copy_rtf_to_clipboard(&rtf, &plain)?;
    word_insert::insert_into_word()
}

/// Copy in-text citation then paste into Word
#[tauri::command]
pub fn insert_inline_into_word(reference: Reference, style: CitationStyle, number: Option<usize>) -> Result<String, String> {
    let plain = format_inline(&reference, style, number);
    let rtf = rich_clipboard::inline_to_rtf(&plain);
    rich_clipboard::copy_rtf_to_clipboard(&rtf, &plain)?;
    word_insert::insert_into_word()
}

/// Copy citation as RTF then paste directly into PowerPoint
#[tauri::command]
pub fn insert_citation_into_ppt(reference: Reference, style: CitationStyle) -> Result<String, String> {
    let rtf = rich_clipboard::citation_to_rtf(&reference, style);
    let plain = format_citation(&reference, style);
    rich_clipboard::copy_rtf_to_clipboard(&rtf, &plain)?;
    word_insert::insert_into_powerpoint()
}

/// Copy bibliography as RTF then paste into Word
#[tauri::command]
pub fn insert_bibliography_into_word(references: Vec<Reference>, style: CitationStyle) -> Result<String, String> {
    let rtf = rich_clipboard::bibliography_to_rtf(&references, style);
    let plain = format_bibliography(&references, style);
    rich_clipboard::copy_rtf_to_clipboard(&rtf, &plain)?;
    word_insert::insert_into_word()
}

// --- Citation workflow commands ---

/// Mark a reference as cited, copy its in-text citation to clipboard, return the inline text.
#[tauri::command]
pub fn cite_reference(id: &str, style: CitationStyle, state: State<DbState>) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::mark_cited(&conn, id).map_err(|e| e.to_string())?;

    // Get the reference
    let reference = db::get_ref_by_id(&conn, id).map_err(|e| e.to_string())?;

    // For IEEE/Vancouver, we need the sequential position among all cited papers
    let cited_refs = db::get_cited_refs(&conn).map_err(|e| e.to_string())?;
    let position = cited_refs.iter().position(|r| r.id == id).map(|p| p + 1).unwrap_or(1);

    let inline = format_inline(&reference, style, Some(position));

    // Copy to clipboard
    let rtf = rich_clipboard::inline_to_rtf(&inline);
    rich_clipboard::copy_rtf_to_clipboard(&rtf, &inline)?;

    Ok(inline)
}

/// Unmark a reference as cited.
#[tauri::command]
pub fn uncite_reference(id: &str, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::unmark_cited(&conn, id).map_err(|e| e.to_string())
}

/// Get all cited references in cite_order.
#[tauri::command]
pub fn get_cited_references(state: State<DbState>) -> Result<Vec<Reference>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_cited_refs(&conn).map_err(|e| e.to_string())
}

/// Copy the bibliography of all cited papers to clipboard.
/// APA/MLA/Chicago/Harvard → sorted alphabetically; IEEE/Vancouver → sorted by cite order.
#[tauri::command]
pub fn copy_cited_bibliography(style: CitationStyle, state: State<DbState>) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut refs = db::get_cited_refs(&conn).map_err(|e| e.to_string())?;
    drop(conn);

    if refs.is_empty() {
        return Err("No cited references".to_string());
    }

    // Sort based on style conventions
    match style {
        CitationStyle::IEEE | CitationStyle::Vancouver => {
            // Already in cite_order from DB query
        }
        _ => {
            // Sort alphabetically by first author's last name
            refs.sort_by(|a, b| a.authors.to_lowercase().cmp(&b.authors.to_lowercase()));
        }
    }

    let rtf = rich_clipboard::bibliography_to_rtf(&refs, style);
    let plain = format_bibliography(&refs, style);
    rich_clipboard::copy_rtf_to_clipboard(&rtf, &plain)?;

    Ok(plain)
}

/// Reset all citation marks.
#[tauri::command]
pub fn reset_citations(state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::reset_all_citations(&conn).map_err(|e| e.to_string())
}

/// Update an existing reference's metadata.
#[tauri::command]
pub fn update_reference(id: String, updated: NewReference, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::update_ref(&conn, &id, &updated).map_err(|e| e.to_string())
}

/// Write BibTeX export to the given file path.
#[tauri::command]
pub fn write_bibtex_file(path: String, state: State<DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let refs = db::get_all_refs(&conn, None).map_err(|e| e.to_string())?;
    drop(conn);

    let content = refs.iter()
        .map(|r| format_citation(r, CitationStyle::BibTeX))
        .collect::<Vec<_>>()
        .join("\n\n");

    std::fs::write(&path, content).map_err(|e| e.to_string())
}
