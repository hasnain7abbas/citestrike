use reqwest;
use serde::Deserialize;

use crate::db::NewReference;

#[derive(Debug, Deserialize)]
struct CrossrefResponse {
    message: CrossrefMessage,
}

#[derive(Debug, Deserialize)]
struct CrossrefMessage {
    title: Option<Vec<String>>,
    author: Option<Vec<CrossrefAuthor>>,
    #[serde(rename = "container-title")]
    container_title: Option<Vec<String>>,
    volume: Option<String>,
    issue: Option<String>,
    page: Option<String>,
    #[serde(rename = "abstract")]
    abstract_text: Option<String>,
    #[serde(rename = "URL")]
    url: Option<String>,
    #[serde(rename = "DOI")]
    doi: Option<String>,
    #[serde(rename = "type")]
    ref_type: Option<String>,
    published: Option<CrossrefDate>,
}

#[derive(Debug, Deserialize)]
struct CrossrefAuthor {
    given: Option<String>,
    family: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CrossrefDate {
    #[serde(rename = "date-parts")]
    date_parts: Option<Vec<Vec<Option<i32>>>>,
}

pub async fn fetch_by_doi(doi: &str) -> Result<NewReference, String> {
    let url = format!("https://api.crossref.org/works/{}", doi);
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "CiteStrike/0.1.0 (mailto:citestrike@example.com)")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Crossref returned status {}", resp.status()));
    }

    let data: CrossrefResponse = resp.json().await.map_err(|e| format!("Parse error: {}", e))?;
    let msg = data.message;

    let title = msg
        .title
        .and_then(|t| t.into_iter().next())
        .unwrap_or_else(|| "Untitled".to_string());

    let authors = msg
        .author
        .map(|authors| {
            authors
                .iter()
                .map(|a| {
                    let family = a.family.as_deref().unwrap_or("");
                    let given = a.given.as_deref().unwrap_or("");
                    format!("{}, {}", family, given).trim_matches(|c| c == ',' || c == ' ').to_string()
                })
                .collect::<Vec<_>>()
                .join("; ")
        })
        .unwrap_or_else(|| "Unknown".to_string());

    let year = msg
        .published
        .and_then(|d| d.date_parts)
        .and_then(|parts| parts.into_iter().next())
        .and_then(|part| part.into_iter().next())
        .flatten();

    Ok(NewReference {
        title,
        authors,
        year,
        doi: msg.doi,
        journal: msg.container_title.and_then(|t| t.into_iter().next()),
        volume: msg.volume,
        issue: msg.issue,
        pages: msg.page,
        abstract_text: msg.abstract_text,
        url: msg.url,
        ref_type: msg.ref_type.unwrap_or_else(|| "article".to_string()),
    })
}

pub async fn search_crossref(query: &str) -> Result<Vec<NewReference>, String> {
    let url = format!(
        "https://api.crossref.org/works?query={}&rows=10",
        urlencoded(query)
    );
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "CiteStrike/0.1.0 (mailto:citestrike@example.com)")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Crossref returned status {}", resp.status()));
    }

    let data: serde_json::Value = resp.json().await.map_err(|e| format!("Parse error: {}", e))?;
    let items = data["message"]["items"]
        .as_array()
        .ok_or("No items found")?;

    let results: Vec<NewReference> = items
        .iter()
        .filter_map(|item| {
            let title = item["title"]
                .as_array()
                .and_then(|t| t.first())
                .and_then(|t| t.as_str())
                .unwrap_or("Untitled")
                .to_string();

            let authors = item["author"]
                .as_array()
                .map(|authors| {
                    authors
                        .iter()
                        .filter_map(|a| {
                            let family = a["family"].as_str().unwrap_or("");
                            let given = a["given"].as_str().unwrap_or("");
                            if family.is_empty() && given.is_empty() {
                                None
                            } else {
                                Some(format!("{}, {}", family, given))
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("; ")
                })
                .unwrap_or_else(|| "Unknown".to_string());

            let year = item["published"]["date-parts"]
                .as_array()
                .and_then(|d| d.first())
                .and_then(|parts| parts.as_array())
                .and_then(|parts| parts.first())
                .and_then(|y| y.as_i64())
                .map(|y| y as i32);

            Some(NewReference {
                title,
                authors,
                year,
                doi: item["DOI"].as_str().map(String::from),
                journal: item["container-title"]
                    .as_array()
                    .and_then(|t| t.first())
                    .and_then(|t| t.as_str())
                    .map(String::from),
                volume: item["volume"].as_str().map(String::from),
                issue: item["issue"].as_str().map(String::from),
                pages: item["page"].as_str().map(String::from),
                abstract_text: item["abstract"].as_str().map(String::from),
                url: item["URL"].as_str().map(String::from),
                ref_type: item["type"]
                    .as_str()
                    .unwrap_or("article")
                    .to_string(),
            })
        })
        .collect();

    Ok(results)
}

fn urlencoded(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ' ' => "+".to_string(),
            c if c.is_alphanumeric() || "-_.~".contains(c) => c.to_string(),
            c => format!("%{:02X}", c as u32),
        })
        .collect()
}
