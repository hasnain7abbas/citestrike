use crate::db::Reference;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum CitationStyle {
    APA,
    MLA,
    Chicago,
    IEEE,
    Harvard,
    BibTeX,
}

pub fn format_citation(reference: &Reference, style: CitationStyle) -> String {
    match style {
        CitationStyle::APA => format_apa(reference),
        CitationStyle::MLA => format_mla(reference),
        CitationStyle::Chicago => format_chicago(reference),
        CitationStyle::IEEE => format_ieee(reference),
        CitationStyle::Harvard => format_harvard(reference),
        CitationStyle::BibTeX => format_bibtex(reference),
    }
}

fn format_apa(r: &Reference) -> String {
    let year = r.year.map(|y| format!("({})", y)).unwrap_or_default();
    let journal = r.journal.as_deref().unwrap_or("");
    let volume = r.volume.as_deref().unwrap_or("");
    let issue = r.issue.as_ref().map(|i| format!("({})", i)).unwrap_or_default();
    let pages = r.pages.as_deref().unwrap_or("");

    let mut citation = format!("{} {}. {}", apa_authors(&r.authors), year, r.title);
    if !journal.is_empty() {
        citation.push_str(&format!(". {}", journal));
        if !volume.is_empty() {
            citation.push_str(&format!(", {}{}", volume, issue));
        }
        if !pages.is_empty() {
            citation.push_str(&format!(", {}", pages));
        }
    }
    citation.push('.');
    if let Some(doi) = &r.doi {
        citation.push_str(&format!(" https://doi.org/{}", doi));
    }
    citation
}

fn apa_authors(authors: &str) -> String {
    let parts: Vec<&str> = authors.split(';').map(|a| a.trim()).collect();
    match parts.len() {
        0 => "Unknown".to_string(),
        1 => parts[0].to_string(),
        2 => format!("{} & {}", parts[0], parts[1]),
        _ => {
            let init = parts[..parts.len() - 1].join(", ");
            format!("{}, & {}", init, parts.last().unwrap())
        }
    }
}

fn format_mla(r: &Reference) -> String {
    let year = r.year.map(|y| y.to_string()).unwrap_or_default();
    let journal = r.journal.as_deref().unwrap_or("");
    let volume = r.volume.as_deref().unwrap_or("");
    let pages = r.pages.as_deref().unwrap_or("");

    let mut citation = format!("{}. \"{}\"", mla_authors(&r.authors), r.title);
    if !journal.is_empty() {
        citation.push_str(&format!(". *{}*", journal));
        if !volume.is_empty() {
            citation.push_str(&format!(", vol. {}", volume));
        }
        if let Some(issue) = &r.issue {
            citation.push_str(&format!(", no. {}", issue));
        }
    }
    if !year.is_empty() {
        citation.push_str(&format!(", {}", year));
    }
    if !pages.is_empty() {
        citation.push_str(&format!(", pp. {}", pages));
    }
    citation.push('.');
    citation
}

fn mla_authors(authors: &str) -> String {
    let parts: Vec<&str> = authors.split(';').map(|a| a.trim()).collect();
    match parts.len() {
        0 => "Unknown".to_string(),
        1 => parts[0].to_string(),
        2 => format!("{} and {}", parts[0], parts[1]),
        _ => format!("{}, et al.", parts[0]),
    }
}

fn format_chicago(r: &Reference) -> String {
    let year = r.year.map(|y| y.to_string()).unwrap_or_default();
    let journal = r.journal.as_deref().unwrap_or("");
    let volume = r.volume.as_deref().unwrap_or("");
    let pages = r.pages.as_deref().unwrap_or("");

    let mut citation = format!("{}. \"{}\"", chicago_authors(&r.authors), r.title);
    if !journal.is_empty() {
        citation.push_str(&format!(". *{}*", journal));
        if !volume.is_empty() {
            citation.push_str(&format!(" {}", volume));
        }
        if let Some(issue) = &r.issue {
            citation.push_str(&format!(", no. {}", issue));
        }
    }
    if !year.is_empty() {
        citation.push_str(&format!(" ({})", year));
    }
    if !pages.is_empty() {
        citation.push_str(&format!(": {}", pages));
    }
    citation.push('.');
    citation
}

fn chicago_authors(authors: &str) -> String {
    apa_authors(authors)
}

fn format_ieee(r: &Reference) -> String {
    let year = r.year.map(|y| y.to_string()).unwrap_or_default();
    let journal = r.journal.as_deref().unwrap_or("");
    let volume = r.volume.as_deref().unwrap_or("");
    let pages = r.pages.as_deref().unwrap_or("");

    let authors = ieee_authors(&r.authors);
    let mut citation = format!("{}, \"{}\"", authors, r.title);
    if !journal.is_empty() {
        citation.push_str(&format!(", *{}*", journal));
    }
    if !volume.is_empty() {
        citation.push_str(&format!(", vol. {}", volume));
    }
    if let Some(issue) = &r.issue {
        citation.push_str(&format!(", no. {}", issue));
    }
    if !pages.is_empty() {
        citation.push_str(&format!(", pp. {}", pages));
    }
    if !year.is_empty() {
        citation.push_str(&format!(", {}", year));
    }
    citation.push('.');
    citation
}

fn ieee_authors(authors: &str) -> String {
    let parts: Vec<&str> = authors.split(';').map(|a| a.trim()).collect();
    match parts.len() {
        0 => "Unknown".to_string(),
        _ => parts.join(", "),
    }
}

fn format_harvard(r: &Reference) -> String {
    let year = r.year.map(|y| y.to_string()).unwrap_or_default();
    let journal = r.journal.as_deref().unwrap_or("");
    let volume = r.volume.as_deref().unwrap_or("");
    let pages = r.pages.as_deref().unwrap_or("");

    let mut citation = format!("{} ({}). '{}'", apa_authors(&r.authors), year, r.title);
    if !journal.is_empty() {
        citation.push_str(&format!(", *{}*", journal));
        if !volume.is_empty() {
            citation.push_str(&format!(", {}", volume));
        }
        if let Some(issue) = &r.issue {
            citation.push_str(&format!("({})", issue));
        }
        if !pages.is_empty() {
            citation.push_str(&format!(", pp. {}", pages));
        }
    }
    citation.push('.');
    citation
}

fn format_bibtex(r: &Reference) -> String {
    let mut bib = format!("@{}{{{},\n", r.ref_type, r.bibtex_key);
    bib.push_str(&format!("  title = {{{}}},\n", r.title));
    bib.push_str(&format!("  author = {{{}}},\n", r.authors));
    if let Some(year) = r.year {
        bib.push_str(&format!("  year = {{{}}},\n", year));
    }
    if let Some(journal) = &r.journal {
        bib.push_str(&format!("  journal = {{{}}},\n", journal));
    }
    if let Some(volume) = &r.volume {
        bib.push_str(&format!("  volume = {{{}}},\n", volume));
    }
    if let Some(issue) = &r.issue {
        bib.push_str(&format!("  number = {{{}}},\n", issue));
    }
    if let Some(pages) = &r.pages {
        bib.push_str(&format!("  pages = {{{}}},\n", pages));
    }
    if let Some(doi) = &r.doi {
        bib.push_str(&format!("  doi = {{{}}},\n", doi));
    }
    bib.push('}');
    bib
}
