use lopdf::Document;
use std::path::Path;

/// Extract DOI from a PDF file by scanning its text content.
pub fn extract_doi_from_pdf(path: &Path) -> Result<String, String> {
    let doc = Document::load(path).map_err(|e| format!("Failed to load PDF: {}", e))?;

    let mut text = String::new();
    let pages: Vec<u32> = doc.page_iter().map(|(num, _)| num).collect();
    for page_num in &pages {
        if let Ok(content) = doc.extract_text(&[*page_num]) {
            text.push_str(&content);
            // Only scan first few pages for DOI
            if text.len() > 20_000 {
                break;
            }
        }
    }

    find_doi_in_text(&text).ok_or_else(|| "No DOI found in PDF".to_string())
}

fn find_doi_in_text(text: &str) -> Option<String> {
    // DOI patterns: 10.XXXX/...
    let doi_prefixes = ["doi:", "doi.org/", "DOI:", "DOI ", "doi "];

    for line in text.lines() {
        let line_lower = line.to_lowercase();
        for prefix in &doi_prefixes {
            if let Some(pos) = line_lower.find(&prefix.to_lowercase()) {
                let start = pos + prefix.len();
                let doi = extract_doi_string(&line[start..]);
                if !doi.is_empty() {
                    return Some(doi);
                }
            }
        }
        // Also try raw 10.XXXX/ pattern
        if let Some(pos) = line.find("10.") {
            let doi = extract_doi_string(&line[pos..]);
            if doi.contains('/') && doi.len() > 8 {
                return Some(doi);
            }
        }
    }
    None
}

fn extract_doi_string(s: &str) -> String {
    s.trim()
        .chars()
        .take_while(|c| !c.is_whitespace() && *c != '>' && *c != '"' && *c != ']' && *c != ')')
        .collect::<String>()
        .trim_end_matches(|c: char| c == '.' || c == ',')
        .to_string()
}
