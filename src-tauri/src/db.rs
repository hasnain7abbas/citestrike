use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reference {
    pub id: String,
    pub title: String,
    pub authors: String,
    pub year: Option<i32>,
    pub doi: Option<String>,
    pub journal: Option<String>,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub pages: Option<String>,
    pub abstract_text: Option<String>,
    pub url: Option<String>,
    pub ref_type: String, // article, book, conference, etc.
    pub bibtex_key: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewReference {
    pub title: String,
    pub authors: String,
    pub year: Option<i32>,
    pub doi: Option<String>,
    pub journal: Option<String>,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub pages: Option<String>,
    pub abstract_text: Option<String>,
    pub url: Option<String>,
    pub ref_type: String,
}

pub fn get_db_path(app_handle: &tauri::AppHandle) -> PathBuf {
    use tauri::Manager;
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("failed to get app data dir");
    std::fs::create_dir_all(&app_dir).ok();
    app_dir.join("citestrike.db")
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS refs (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            authors TEXT NOT NULL,
            year INTEGER,
            doi TEXT,
            journal TEXT,
            volume TEXT,
            issue TEXT,
            pages TEXT,
            abstract_text TEXT,
            url TEXT,
            ref_type TEXT NOT NULL DEFAULT 'article',
            bibtex_key TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_refs_title ON refs(title);
        CREATE INDEX IF NOT EXISTS idx_refs_authors ON refs(authors);
        CREATE INDEX IF NOT EXISTS idx_refs_doi ON refs(doi);
        CREATE INDEX IF NOT EXISTS idx_refs_bibtex_key ON refs(bibtex_key);",
    )?;
    Ok(())
}

pub fn search_refs(conn: &Connection, query: &str) -> Result<Vec<Reference>> {
    let pattern = format!("%{}%", query);
    let mut stmt = conn.prepare(
        "SELECT id, title, authors, year, doi, journal, volume, issue, pages,
                abstract_text, url, ref_type, bibtex_key, created_at
         FROM refs
         WHERE title LIKE ?1 OR authors LIKE ?1 OR doi LIKE ?1 OR bibtex_key LIKE ?1
         ORDER BY created_at DESC
         LIMIT 50",
    )?;
    let rows = stmt.query_map(params![pattern], |row| {
        Ok(Reference {
            id: row.get(0)?,
            title: row.get(1)?,
            authors: row.get(2)?,
            year: row.get(3)?,
            doi: row.get(4)?,
            journal: row.get(5)?,
            volume: row.get(6)?,
            issue: row.get(7)?,
            pages: row.get(8)?,
            abstract_text: row.get(9)?,
            url: row.get(10)?,
            ref_type: row.get(11)?,
            bibtex_key: row.get(12)?,
            created_at: row.get(13)?,
        })
    })?;
    rows.collect()
}

pub fn insert_ref(conn: &Connection, new_ref: &NewReference) -> Result<Reference> {
    let id = uuid::Uuid::new_v4().to_string();
    let bibtex_key = generate_bibtex_key(&new_ref.authors, new_ref.year);
    let created_at = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO refs (id, title, authors, year, doi, journal, volume, issue, pages,
                           abstract_text, url, ref_type, bibtex_key, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            id, new_ref.title, new_ref.authors, new_ref.year, new_ref.doi,
            new_ref.journal, new_ref.volume, new_ref.issue, new_ref.pages,
            new_ref.abstract_text, new_ref.url, new_ref.ref_type, bibtex_key, created_at,
        ],
    )?;

    Ok(Reference {
        id,
        title: new_ref.title.clone(),
        authors: new_ref.authors.clone(),
        year: new_ref.year,
        doi: new_ref.doi.clone(),
        journal: new_ref.journal.clone(),
        volume: new_ref.volume.clone(),
        issue: new_ref.issue.clone(),
        pages: new_ref.pages.clone(),
        abstract_text: new_ref.abstract_text.clone(),
        url: new_ref.url.clone(),
        ref_type: new_ref.ref_type.clone(),
        bibtex_key,
        created_at,
    })
}

pub fn delete_ref(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM refs WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_all_refs(conn: &Connection) -> Result<Vec<Reference>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, authors, year, doi, journal, volume, issue, pages,
                abstract_text, url, ref_type, bibtex_key, created_at
         FROM refs ORDER BY created_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Reference {
            id: row.get(0)?,
            title: row.get(1)?,
            authors: row.get(2)?,
            year: row.get(3)?,
            doi: row.get(4)?,
            journal: row.get(5)?,
            volume: row.get(6)?,
            issue: row.get(7)?,
            pages: row.get(8)?,
            abstract_text: row.get(9)?,
            url: row.get(10)?,
            ref_type: row.get(11)?,
            bibtex_key: row.get(12)?,
            created_at: row.get(13)?,
        })
    })?;
    rows.collect()
}

fn generate_bibtex_key(authors: &str, year: Option<i32>) -> String {
    let last_name = authors
        .split(',')
        .next()
        .unwrap_or("unknown")
        .split_whitespace()
        .last()
        .unwrap_or("unknown")
        .to_lowercase();
    let year_str = year.map(|y| y.to_string()).unwrap_or_default();
    format!("{}{}", last_name, year_str)
}
