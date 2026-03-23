use axum::{
    extract::{Query, State as AxumState},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use rusqlite::Connection;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

use crate::citation::{self, CitationStyle};
use crate::db::{self, Reference};

/// Shared state for the API server — holds its own DB connection
pub struct ApiState {
    pub conn: Mutex<Connection>,
}

pub async fn start_server(db_path: std::path::PathBuf) {
    let conn =
        Connection::open(&db_path).expect("API server: failed to open database");
    let state = Arc::new(ApiState {
        conn: Mutex::new(conn),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // Office add-in pages
        .route("/taskpane.html", get(taskpane_html))
        .route("/taskpane.css", get(taskpane_css))
        .route("/taskpane.js", get(taskpane_js))
        .route("/manifest.xml", get(manifest_xml))
        // REST API
        .route("/api/search", get(api_search))
        .route("/api/references", get(api_references))
        .route("/api/format", post(api_format))
        .route("/api/format-inline", post(api_format_inline))
        .route("/api/format-bibliography", post(api_format_bibliography))
        .route("/api/health", get(api_health))
        .layer(cors)
        .with_state(state);

    let listener = match tokio::net::TcpListener::bind("127.0.0.1:27182").await {
        Ok(l) => l,
        Err(e) => {
            log::warn!("Failed to start Office add-in server on port 27182: {}", e);
            return;
        }
    };

    log::info!("Office add-in server running at http://localhost:27182");

    if let Err(e) = axum::serve(listener, app).await {
        log::warn!("Office add-in server error: {}", e);
    }
}

// ── API Endpoints ──────────────────────────────────────────────────

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
    folder_id: Option<String>,
}

async fn api_search(
    AxumState(state): AxumState<Arc<ApiState>>,
    Query(params): Query<SearchQuery>,
) -> Response {
    let conn = state.conn.lock().unwrap();
    let q = params.q.unwrap_or_default();
    let folder_id = params.folder_id.as_deref();

    let refs = if q.trim().is_empty() {
        db::get_all_refs(&conn, folder_id)
    } else {
        db::search_refs(&conn, &q, folder_id)
    };

    match refs {
        Ok(r) => Json(r).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn api_references(AxumState(state): AxumState<Arc<ApiState>>) -> Response {
    let conn = state.conn.lock().unwrap();
    match db::get_all_refs(&conn, None) {
        Ok(r) => Json(r).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
struct FormatQuery {
    style: String,
}

fn parse_style(s: &str) -> Option<CitationStyle> {
    match s {
        "APA" => Some(CitationStyle::APA),
        "MLA" => Some(CitationStyle::MLA),
        "Chicago" => Some(CitationStyle::Chicago),
        "IEEE" => Some(CitationStyle::IEEE),
        "Harvard" => Some(CitationStyle::Harvard),
        "Vancouver" => Some(CitationStyle::Vancouver),
        "BibTeX" => Some(CitationStyle::BibTeX),
        _ => None,
    }
}

async fn api_format(
    Query(params): Query<FormatQuery>,
    Json(reference): Json<Reference>,
) -> Response {
    let style = match parse_style(&params.style) {
        Some(s) => s,
        None => return (StatusCode::BAD_REQUEST, "Invalid style").into_response(),
    };
    let formatted = citation::format_citation(&reference, style);
    Json(serde_json::json!({ "formatted": formatted })).into_response()
}

#[derive(Deserialize)]
struct InlineQuery {
    style: String,
    number: Option<usize>,
}

async fn api_format_inline(
    Query(params): Query<InlineQuery>,
    Json(reference): Json<Reference>,
) -> Response {
    let style = match parse_style(&params.style) {
        Some(s) => s,
        None => return (StatusCode::BAD_REQUEST, "Invalid style").into_response(),
    };
    let formatted = citation::format_inline(&reference, style, params.number);
    Json(serde_json::json!({ "formatted": formatted })).into_response()
}

#[derive(Deserialize)]
struct BibQuery {
    style: String,
}

async fn api_format_bibliography(
    Query(params): Query<BibQuery>,
    Json(references): Json<Vec<Reference>>,
) -> Response {
    let style = match parse_style(&params.style) {
        Some(s) => s,
        None => return (StatusCode::BAD_REQUEST, "Invalid style").into_response(),
    };
    let formatted = citation::format_bibliography(&references, style);
    Json(serde_json::json!({ "formatted": formatted })).into_response()
}

async fn api_health() -> &'static str {
    "CiteStrike API OK"
}

// ── Static files for the Office Add-in ─────────────────────────────

async fn taskpane_html() -> Html<&'static str> {
    Html(include_str!("addin/taskpane.html"))
}

async fn taskpane_css() -> Response {
    (
        [(axum::http::header::CONTENT_TYPE, "text/css")],
        include_str!("addin/taskpane.css"),
    )
        .into_response()
}

async fn taskpane_js() -> Response {
    (
        [(axum::http::header::CONTENT_TYPE, "application/javascript")],
        include_str!("addin/taskpane.js"),
    )
        .into_response()
}

async fn manifest_xml() -> Response {
    (
        [(axum::http::header::CONTENT_TYPE, "application/xml")],
        include_str!("addin/manifest.xml"),
    )
        .into_response()
}
