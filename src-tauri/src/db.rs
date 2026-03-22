use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};

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
    pub ref_type: String,
    pub bibtex_key: String,
    pub folder_id: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub color: String,
    pub created_at: String,
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS folders (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            color TEXT NOT NULL DEFAULT '#38BDF8',
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS refs (
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
            folder_id TEXT REFERENCES folders(id) ON DELETE SET NULL,
            created_at TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_refs_title ON refs(title);
        CREATE INDEX IF NOT EXISTS idx_refs_authors ON refs(authors);
        CREATE INDEX IF NOT EXISTS idx_refs_doi ON refs(doi);
        CREATE INDEX IF NOT EXISTS idx_refs_folder ON refs(folder_id);",
    )?;
    Ok(())
}

pub fn search_refs(conn: &Connection, query: &str, folder_id: Option<&str>) -> Result<Vec<Reference>> {
    let pattern = format!("%{}%", query);
    let sql = if folder_id.is_some() {
        "SELECT id, title, authors, year, doi, journal, volume, issue, pages,
                abstract_text, url, ref_type, bibtex_key, folder_id, created_at
         FROM refs
         WHERE (title LIKE ?1 OR authors LIKE ?1 OR doi LIKE ?1 OR bibtex_key LIKE ?1)
           AND folder_id = ?2
         ORDER BY created_at DESC LIMIT 100"
    } else {
        "SELECT id, title, authors, year, doi, journal, volume, issue, pages,
                abstract_text, url, ref_type, bibtex_key, folder_id, created_at
         FROM refs
         WHERE title LIKE ?1 OR authors LIKE ?1 OR doi LIKE ?1 OR bibtex_key LIKE ?1
         ORDER BY created_at DESC LIMIT 100"
    };

    let mut stmt = conn.prepare(sql)?;
    let rows = if let Some(fid) = folder_id {
        stmt.query_map(params![pattern, fid], row_to_ref)?
    } else {
        stmt.query_map(params![pattern], row_to_ref)?
    };
    rows.collect()
}

pub fn get_all_refs(conn: &Connection, folder_id: Option<&str>) -> Result<Vec<Reference>> {
    let sql = if folder_id.is_some() {
        "SELECT id, title, authors, year, doi, journal, volume, issue, pages,
                abstract_text, url, ref_type, bibtex_key, folder_id, created_at
         FROM refs WHERE folder_id = ?1 ORDER BY created_at DESC"
    } else {
        "SELECT id, title, authors, year, doi, journal, volume, issue, pages,
                abstract_text, url, ref_type, bibtex_key, folder_id, created_at
         FROM refs ORDER BY created_at DESC"
    };

    let mut stmt = conn.prepare(sql)?;
    let rows = if let Some(fid) = folder_id {
        stmt.query_map(params![fid], row_to_ref)?
    } else {
        stmt.query_map([], row_to_ref)?
    };
    rows.collect()
}

pub fn insert_ref(conn: &Connection, new_ref: &NewReference, folder_id: Option<&str>) -> Result<Reference> {
    let id = uuid::Uuid::new_v4().to_string();
    let bibtex_key = generate_bibtex_key(&new_ref.authors, new_ref.year);
    let created_at = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO refs (id, title, authors, year, doi, journal, volume, issue, pages,
                           abstract_text, url, ref_type, bibtex_key, folder_id, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
        params![
            id, new_ref.title, new_ref.authors, new_ref.year, new_ref.doi,
            new_ref.journal, new_ref.volume, new_ref.issue, new_ref.pages,
            new_ref.abstract_text, new_ref.url, new_ref.ref_type, bibtex_key, folder_id, created_at,
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
        folder_id: folder_id.map(String::from),
        created_at,
    })
}

pub fn delete_ref(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM refs WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn move_ref_to_folder(conn: &Connection, ref_id: &str, folder_id: Option<&str>) -> Result<()> {
    conn.execute(
        "UPDATE refs SET folder_id = ?1 WHERE id = ?2",
        params![folder_id, ref_id],
    )?;
    Ok(())
}

// Folder CRUD
pub fn create_folder(conn: &Connection, name: &str, color: &str) -> Result<Folder> {
    let id = uuid::Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO folders (id, name, color, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![id, name, color, created_at],
    )?;
    Ok(Folder { id, name: name.to_string(), color: color.to_string(), created_at })
}

pub fn get_folders(conn: &Connection) -> Result<Vec<Folder>> {
    let mut stmt = conn.prepare("SELECT id, name, color, created_at FROM folders ORDER BY name")?;
    let rows = stmt.query_map([], |row| {
        Ok(Folder {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;
    rows.collect()
}

pub fn rename_folder(conn: &Connection, id: &str, name: &str) -> Result<()> {
    conn.execute("UPDATE folders SET name = ?1 WHERE id = ?2", params![name, id])?;
    Ok(())
}

pub fn delete_folder(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("UPDATE refs SET folder_id = NULL WHERE folder_id = ?1", params![id])?;
    conn.execute("DELETE FROM folders WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_ref_count(conn: &Connection, folder_id: Option<&str>) -> Result<i64> {
    if let Some(fid) = folder_id {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM refs WHERE folder_id = ?1", params![fid], |r| r.get(0))?;
        Ok(count)
    } else {
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM refs", [], |r| r.get(0))?;
        Ok(count)
    }
}

fn row_to_ref(row: &rusqlite::Row) -> rusqlite::Result<Reference> {
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
        folder_id: row.get(13)?,
        created_at: row.get(14)?,
    })
}

fn generate_bibtex_key(authors: &str, year: Option<i32>) -> String {
    let last_name = authors
        .split(';')
        .next()
        .unwrap_or("unknown")
        .split(',')
        .next()
        .unwrap_or("unknown")
        .trim()
        .to_lowercase()
        .replace(' ', "");
    let year_str = year.map(|y| y.to_string()).unwrap_or_default();
    format!("{}{}", last_name, year_str)
}
