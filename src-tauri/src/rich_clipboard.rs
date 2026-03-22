use crate::db::Reference;
use crate::citation::CitationStyle;

/// Set RTF content on the Windows clipboard so Word/PowerPoint receives formatted text.
pub fn copy_rtf_to_clipboard(rtf: &str, plain: &str) -> Result<(), String> {
    use clipboard_win::Clipboard;
    use clipboard_win::raw::{set_without_clear, register_format};

    // Open clipboard and set both plain text and RTF
    let _clip = Clipboard::new_attempts(10).map_err(|e| format!("Clipboard open failed: {}", e))?;

    // Clear first
    clipboard_win::raw::empty().map_err(|e| format!("Clipboard clear failed: {}", e))?;

    // Set plain text fallback
    clipboard_win::raw::set_string(plain).map_err(|e| format!("Set plain text failed: {}", e))?;

    // Register and set RTF format
    let rtf_format = register_format("Rich Text Format").ok_or("Failed to register RTF format")?;
    let rtf_bytes = rtf.as_bytes();
    set_without_clear(rtf_format.get(), rtf_bytes)
        .map_err(|e| format!("Set RTF failed: {}", e))?;

    Ok(())
}

/// Generate RTF string from a citation with proper italic formatting for journal names.
pub fn citation_to_rtf(reference: &Reference, style: CitationStyle) -> String {
    let rtf_header = r"{\rtf1\ansi\deff0{\fonttbl{\f0\fswiss\fcharset0 Calibri;}}{\colortbl;\red0\green0\blue0;}\viewkind4\uc1\pard\f0\fs22 ";
    let rtf_footer = "}";

    let body = match style {
        CitationStyle::APA => rtf_apa(reference),
        CitationStyle::MLA => rtf_mla(reference),
        CitationStyle::Chicago => rtf_chicago(reference),
        CitationStyle::IEEE => rtf_ieee(reference),
        CitationStyle::Harvard => rtf_harvard(reference),
        CitationStyle::BibTeX => {
            // BibTeX is code, no rich formatting needed
            rtf_escape(&crate::citation::format_citation(reference, style))
        }
    };

    format!("{}{}{}", rtf_header, body, rtf_footer)
}

/// Generate RTF for inline citation (no special formatting needed, but keep consistent)
pub fn inline_to_rtf(text: &str) -> String {
    format!(
        r"{{\rtf1\ansi\deff0{{\fonttbl{{\f0\fswiss\fcharset0 Calibri;}}}}{{\colortbl;\red0\green0\blue0;}}\viewkind4\uc1\pard\f0\fs22 {}}}",
        rtf_escape(text)
    )
}

/// Generate RTF for a batch bibliography
pub fn bibliography_to_rtf(references: &[Reference], style: CitationStyle) -> String {
    let rtf_header = r"{\rtf1\ansi\deff0{\fonttbl{\f0\fswiss\fcharset0 Calibri;}}{\colortbl;\red0\green0\blue0;}\viewkind4\uc1\pard\f0\fs22 ";

    let mut body = String::new();
    for (i, reference) in references.iter().enumerate() {
        if i > 0 {
            body.push_str(r"\par\par ");
        }
        let entry = match style {
            CitationStyle::IEEE => {
                format!("[{}] {}", i + 1, match style {
                    _ => rtf_ieee_entry(reference),
                })
            }
            CitationStyle::APA => rtf_apa(reference),
            CitationStyle::MLA => rtf_mla(reference),
            CitationStyle::Chicago => rtf_chicago(reference),
            CitationStyle::Harvard => rtf_harvard(reference),
            CitationStyle::BibTeX => rtf_escape(&crate::citation::format_citation(reference, style)),
        };
        body.push_str(&entry);
    }

    format!("{}{}}}", rtf_header, body)
}

// --- RTF formatters per style ---

fn rtf_apa(r: &Reference) -> String {
    let year = r.year.map(|y| format!("({})", y)).unwrap_or("(n.d.)".to_string());
    let authors = apa_authors_str(&r.authors);

    let mut s = format!("{} {}. {}", rtf_escape(&authors), rtf_escape(&year), rtf_escape(&r.title));

    if let Some(journal) = &r.journal {
        // Journal name in italics
        s.push_str(&format!(". {{\\i {}}}", rtf_escape(journal)));
        if let Some(vol) = &r.volume {
            s.push_str(&format!(", {{\\i {}}}", rtf_escape(vol)));
            if let Some(issue) = &r.issue {
                s.push_str(&format!("({})", rtf_escape(issue)));
            }
        }
        if let Some(pages) = &r.pages {
            s.push_str(&format!(", {}", rtf_escape(pages)));
        }
    }
    s.push('.');
    if let Some(doi) = &r.doi {
        s.push_str(&format!(" https://doi.org/{}", rtf_escape(doi)));
    }
    s
}

fn rtf_mla(r: &Reference) -> String {
    let authors = mla_authors_str(&r.authors);
    let mut s = format!("{}. \\ldblquote {}\\rdblquote ", rtf_escape(&authors), rtf_escape(&r.title));

    if let Some(journal) = &r.journal {
        s.push_str(&format!(". {{\\i {}}}", rtf_escape(journal)));
        if let Some(vol) = &r.volume {
            s.push_str(&format!(", vol. {}", rtf_escape(vol)));
        }
        if let Some(issue) = &r.issue {
            s.push_str(&format!(", no. {}", rtf_escape(issue)));
        }
    }
    if let Some(year) = r.year {
        s.push_str(&format!(", {}", year));
    }
    if let Some(pages) = &r.pages {
        s.push_str(&format!(", pp. {}", rtf_escape(pages)));
    }
    s.push('.');
    if let Some(doi) = &r.doi {
        s.push_str(&format!(" https://doi.org/{}", rtf_escape(doi)));
    }
    s
}

fn rtf_chicago(r: &Reference) -> String {
    let authors = apa_authors_str(&r.authors);
    let mut s = format!("{}. \\ldblquote {}\\rdblquote ", rtf_escape(&authors), rtf_escape(&r.title));

    if let Some(journal) = &r.journal {
        s.push_str(&format!(". {{\\i {}}}", rtf_escape(journal)));
        if let Some(vol) = &r.volume {
            s.push_str(&format!(" {}", rtf_escape(vol)));
        }
        if let Some(issue) = &r.issue {
            s.push_str(&format!(", no. {}", rtf_escape(issue)));
        }
    }
    if let Some(year) = r.year {
        s.push_str(&format!(" ({})", year));
    }
    if let Some(pages) = &r.pages {
        s.push_str(&format!(": {}", rtf_escape(pages)));
    }
    s.push('.');
    s
}

fn rtf_ieee(r: &Reference) -> String {
    rtf_ieee_entry(r)
}

fn rtf_ieee_entry(r: &Reference) -> String {
    let authors = ieee_authors_str(&r.authors);
    let mut s = format!("{}, \\ldblquote {}\\rdblquote ", rtf_escape(&authors), rtf_escape(&r.title));

    if let Some(journal) = &r.journal {
        s.push_str(&format!(", {{\\i {}}}", rtf_escape(journal)));
    }
    if let Some(vol) = &r.volume {
        s.push_str(&format!(", vol. {}", rtf_escape(vol)));
    }
    if let Some(issue) = &r.issue {
        s.push_str(&format!(", no. {}", rtf_escape(issue)));
    }
    if let Some(pages) = &r.pages {
        s.push_str(&format!(", pp. {}", rtf_escape(pages)));
    }
    if let Some(year) = r.year {
        s.push_str(&format!(", {}", year));
    }
    s.push('.');
    if let Some(doi) = &r.doi {
        s.push_str(&format!(" doi: {}", rtf_escape(doi)));
    }
    s
}

fn rtf_harvard(r: &Reference) -> String {
    let authors = apa_authors_str(&r.authors);
    let year = r.year.map(|y| y.to_string()).unwrap_or("n.d.".to_string());

    let mut s = format!("{} ({}). \\lquote {}\\rquote ", rtf_escape(&authors), rtf_escape(&year), rtf_escape(&r.title));

    if let Some(journal) = &r.journal {
        s.push_str(&format!(", {{\\i {}}}", rtf_escape(journal)));
        if let Some(vol) = &r.volume {
            s.push_str(&format!(", {}", rtf_escape(vol)));
        }
        if let Some(issue) = &r.issue {
            s.push_str(&format!("({})", rtf_escape(issue)));
        }
        if let Some(pages) = &r.pages {
            s.push_str(&format!(", pp. {}", rtf_escape(pages)));
        }
    }
    s.push('.');
    if let Some(doi) = &r.doi {
        s.push_str(&format!(" Available at: https://doi.org/{}", rtf_escape(doi)));
    }
    s
}

// --- Helpers ---

fn rtf_escape(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\\' => "\\\\".to_string(),
            '{' => "\\{".to_string(),
            '}' => "\\}".to_string(),
            c if (c as u32) > 127 => format!("\\u{}?", c as i32),
            c => c.to_string(),
        })
        .collect()
}

fn apa_authors_str(authors: &str) -> String {
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
            let first19 = parts[..19].join(", ");
            format!("{}, ... {}", first19, parts.last().unwrap())
        }
    }
}

fn mla_authors_str(authors: &str) -> String {
    let parts: Vec<&str> = authors.split(';').map(|a| a.trim()).collect();
    match parts.len() {
        0 => "Unknown".to_string(),
        1 => parts[0].to_string(),
        2 => format!("{}, and {}", parts[0], parts[1]),
        _ => format!("{}, et al.", parts[0]),
    }
}

fn ieee_authors_str(authors: &str) -> String {
    let parts: Vec<&str> = authors.split(';').map(|a| a.trim()).collect();
    parts
        .iter()
        .map(|a| {
            let segs: Vec<&str> = a.split(',').map(|s| s.trim()).collect();
            if segs.len() >= 2 {
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
