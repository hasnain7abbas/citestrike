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

/// Full bibliography entry
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

/// In-text citation like (Smith et al., 2024) or [1]
pub fn format_inline(reference: &Reference, style: CitationStyle, number: Option<usize>) -> String {
    let first_author = get_first_last_name(&reference.authors);
    let author_count = reference.authors.split(';').count();
    let year = reference.year.map(|y| y.to_string()).unwrap_or("n.d.".to_string());

    match style {
        CitationStyle::APA => {
            let author_part = match author_count {
                1 => first_author,
                2 => {
                    let second = get_nth_last_name(&reference.authors, 1);
                    format!("{} & {}", first_author, second)
                }
                _ => format!("{} et al.", first_author),
            };
            format!("({}, {})", author_part, year)
        }
        CitationStyle::MLA => {
            let author_part = match author_count {
                1 | 2 => {
                    if author_count == 2 {
                        let second = get_nth_last_name(&reference.authors, 1);
                        format!("{} and {}", first_author, second)
                    } else {
                        first_author
                    }
                }
                _ => format!("{} et al.", first_author),
            };
            // MLA uses page numbers in-text; we include if available
            if let Some(pages) = &reference.pages {
                format!("({} {})", author_part, pages)
            } else {
                format!("({})", author_part)
            }
        }
        CitationStyle::Chicago => {
            let author_part = match author_count {
                1 => first_author,
                2 | 3 => {
                    let names: Vec<String> = (0..author_count)
                        .map(|i| get_nth_last_name(&reference.authors, i))
                        .collect();
                    if names.len() == 2 {
                        format!("{} and {}", names[0], names[1])
                    } else {
                        format!("{}, {}, and {}", names[0], names[1], names[2])
                    }
                }
                _ => format!("{} et al.", first_author),
            };
            format!("({} {})", author_part, year)
        }
        CitationStyle::IEEE => {
            format!("[{}]", number.unwrap_or(1))
        }
        CitationStyle::Harvard => {
            let author_part = match author_count {
                1 => first_author,
                2 => {
                    let second = get_nth_last_name(&reference.authors, 1);
                    format!("{} and {}", first_author, second)
                }
                _ => format!("{} et al.", first_author),
            };
            format!("({} {})", author_part, year)
        }
        CitationStyle::BibTeX => {
            format!("\\cite{{{}}}", reference.bibtex_key)
        }
    }
}

/// Generate a full bibliography from multiple references
pub fn format_bibliography(references: &[Reference], style: CitationStyle) -> String {
    let mut entries: Vec<String> = Vec::new();

    for (i, reference) in references.iter().enumerate() {
        let entry = match style {
            CitationStyle::IEEE => {
                format!("[{}] {}", i + 1, format_citation(reference, style))
            }
            _ => format_citation(reference, style),
        };
        entries.push(entry);
    }

    entries.join("\n\n")
}

fn get_first_last_name(authors: &str) -> String {
    authors
        .split(';')
        .next()
        .unwrap_or("Unknown")
        .split(',')
        .next()
        .unwrap_or("Unknown")
        .trim()
        .to_string()
}

fn get_nth_last_name(authors: &str, n: usize) -> String {
    authors
        .split(';')
        .nth(n)
        .unwrap_or("Unknown")
        .split(',')
        .next()
        .unwrap_or("Unknown")
        .trim()
        .to_string()
}

// --- Full bibliography formatters ---

fn format_apa(r: &Reference) -> String {
    let year = r.year.map(|y| format!("({})", y)).unwrap_or("(n.d.)".to_string());
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
        2 => format!("{}, & {}", parts[0], parts[1]),
        _ if parts.len() <= 20 => {
            let init = parts[..parts.len() - 1].join(", ");
            format!("{}, & {}", init, parts.last().unwrap())
        }
        _ => {
            // APA 7th: list first 19, then ... then last
            let first19 = parts[..19].join(", ");
            format!("{}, ... {}", first19, parts.last().unwrap())
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
        citation.push_str(&format!(". {}", journal));
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
    if let Some(doi) = &r.doi {
        citation.push_str(&format!(" https://doi.org/{}", doi));
    }
    citation
}

fn mla_authors(authors: &str) -> String {
    let parts: Vec<&str> = authors.split(';').map(|a| a.trim()).collect();
    match parts.len() {
        0 => "Unknown".to_string(),
        1 => parts[0].to_string(),
        2 => format!("{}, and {}", parts[0], parts[1]),
        _ => format!("{}, et al.", parts[0]),
    }
}

fn format_chicago(r: &Reference) -> String {
    let year = r.year.map(|y| y.to_string()).unwrap_or_default();
    let journal = r.journal.as_deref().unwrap_or("");
    let volume = r.volume.as_deref().unwrap_or("");
    let pages = r.pages.as_deref().unwrap_or("");

    let mut citation = format!("{}. \"{}\"", apa_authors(&r.authors), r.title);
    if !journal.is_empty() {
        citation.push_str(&format!(". {}", journal));
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
    if let Some(doi) = &r.doi {
        citation.push_str(&format!(" https://doi.org/{}", doi));
    }
    citation
}

fn format_ieee(r: &Reference) -> String {
    let year = r.year.map(|y| y.to_string()).unwrap_or_default();
    let journal = r.journal.as_deref().unwrap_or("");
    let volume = r.volume.as_deref().unwrap_or("");
    let pages = r.pages.as_deref().unwrap_or("");

    let authors = ieee_authors(&r.authors);
    let mut citation = format!("{}, \"{}\"", authors, r.title);
    if !journal.is_empty() {
        citation.push_str(&format!(", {}", journal));
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
    if let Some(doi) = &r.doi {
        citation.push_str(&format!(" doi: {}", doi));
    }
    citation
}

fn ieee_authors(authors: &str) -> String {
    let parts: Vec<&str> = authors.split(';').map(|a| a.trim()).collect();
    // IEEE uses initials first: "J. Smith" format
    parts
        .iter()
        .map(|a| {
            let segs: Vec<&str> = a.split(',').map(|s| s.trim()).collect();
            if segs.len() >= 2 {
                // "Smith, John" -> "J. Smith"
                let initials: String = segs[1]
                    .split_whitespace()
                    .map(|w| format!("{}.", w.chars().next().unwrap_or(' ')))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("{} {}", initials, segs[0])
            } else {
                a.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_harvard(r: &Reference) -> String {
    let year = r.year.map(|y| y.to_string()).unwrap_or("n.d.".to_string());
    let journal = r.journal.as_deref().unwrap_or("");
    let volume = r.volume.as_deref().unwrap_or("");
    let pages = r.pages.as_deref().unwrap_or("");

    let mut citation = format!("{} ({}). '{}'", apa_authors(&r.authors), year, r.title);
    if !journal.is_empty() {
        citation.push_str(&format!(", {}", journal));
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
    if let Some(doi) = &r.doi {
        citation.push_str(&format!(" Available at: https://doi.org/{}", doi));
    }
    citation
}

fn format_bibtex(r: &Reference) -> String {
    let ref_type = match r.ref_type.as_str() {
        "journal-article" => "article",
        "book" | "monograph" => "book",
        "proceedings-article" | "posted-content" => "inproceedings",
        "book-chapter" => "incollection",
        "dissertation" | "thesis" => "phdthesis",
        other => other,
    };
    let mut bib = format!("@{}{{{},\n", ref_type, r.bibtex_key);
    bib.push_str(&format!("  title = {{{{{}}}}},\n", r.title));
    bib.push_str(&format!("  author = {{{}}},\n", r.authors.replace(';', " and")));
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
    if let Some(url) = &r.url {
        bib.push_str(&format!("  url = {{{}}},\n", url));
    }
    bib.push('}');
    bib
}
