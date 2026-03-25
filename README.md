<p align="center">
  <img src="./logo.svg" alt="CiteStrike Logo" width="150" />
</p>

<h1 align="center">CiteStrike</h1>

<p align="center">
  <strong>A blazing-fast, local-first citation manager built with Rust & Tauri — the "Raycast" of reference management.</strong>
</p>

<p align="center">
  <a href="#"><img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" /></a>
  <a href="#"><img src="https://img.shields.io/badge/tauri-%2324C8DB.svg?style=for-the-badge&logo=tauri&logoColor=%23FFFFFF" alt="Tauri" /></a>
  <a href="#"><img src="https://img.shields.io/badge/svelte-%23f1413d.svg?style=for-the-badge&logo=svelte&logoColor=white" alt="Svelte" /></a>
  <a href="#"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge" alt="License" /></a>
</p>

<p align="center">
  <a href="https://github.com/hasnainqau5112/citestrike/actions/workflows/ci.yml"><img src="https://github.com/hasnainqau5112/citestrike/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  <a href="https://github.com/hasnainqau5112/citestrike/releases/latest"><img src="https://img.shields.io/github/v/release/hasnainqau5112/citestrike?style=flat-square&color=2ecc71" alt="Latest Release" /></a>
  <img src="https://img.shields.io/badge/RAM-<_15MB-2ecc71?style=flat-square" alt="RAM < 15MB" />
  <img src="https://img.shields.io/badge/startup-<_200ms-2ecc71?style=flat-square" alt="Startup < 200ms" />
  <img src="https://img.shields.io/badge/platform-Windows-blue?style=flat-square" alt="Windows" />
</p>

---

## The Problem

Current citation managers — EndNote, Mendeley, Zotero — are heavy, bloated, and require constant context switching. Every time you need a reference you leave your writing flow, dig through a clunky GUI, copy text, switch back, and paste. Multiply that by hundreds of citations in a thesis and you've lost hours.

## The Solution

**CiteStrike** runs silently in the background consuming **< 15 MB** of RAM. When you need a citation in *any* application — Word, Google Docs, Overleaf, Obsidian, Notion — press:

<p align="center">
  <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>C</kbd>
</p>

A spotlight-style command palette appears instantly. Type the author or title, hit <kbd>Enter</kbd>, and the perfectly formatted citation lands in your clipboard — or directly into your document. No window switching. No friction.

---

## Download

Head over to the [**Releases**](https://github.com/hasnainqau5112/citestrike/releases/latest) page and grab the latest installer:

| File | Description |
|------|-------------|
| `CiteStrike_x.x.x_x64_en-US.msi` | Standard Windows installer (recommended) |
| `CiteStrike_x.x.x_x64-setup.exe` | NSIS installer |

---

## How It Works

CiteStrike is a complete citation workflow in four steps:

### 1. Import Papers

- **Drag & drop PDFs** onto the app window — DOI is extracted automatically and metadata fetched from Crossref
- **Paste a DOI** directly in the "Add by DOI" view
- **Search Crossref** by title, author, or topic and add results with one click
- **Add manually** for papers without a DOI — fill in title, authors, year, journal, and other fields

### 2. Build Your Library

All imported papers appear in a scrollable reference library. Each entry shows authors, year, title, journal, and DOI. You can:

- Search across your entire library with instant fuzzy matching
- Organize references into color-coded collections (folders)
- Edit, move, or delete any reference
- The library persists across app restarts — everything lives in a local SQLite database

### 3. Cite Papers (One Click)

Click the **Cite** button on any reference. Two things happen simultaneously:

1. The **in-text citation** is copied to your clipboard — `(Smith et al., 2023)` for APA, `[1]` for IEEE, etc.
2. The paper is **marked as cited** with a numbered badge showing its citation order

Paste the in-text citation wherever you're writing. Cited papers are tracked so the bibliography knows exactly which papers to include.

### 4. Generate Bibliography

Click **Copy Bibliography** in the toolbar. CiteStrike generates the full formatted bibliography of all cited papers in the selected citation style:

- **APA / MLA / Chicago / Harvard** — sorted alphabetically by first author
- **IEEE / Vancouver** — numbered in order of first citation

The entire bibliography is copied to your clipboard as rich text (with proper italics). Paste it at the end of your document.

### 5. Switch Styles Anytime

Switch between APA, MLA, Chicago, IEEE, Harvard, Vancouver, or BibTeX at any time using the style selector. The in-text citation format and bibliography output update to match the selected style.

---

## Key Features

| Feature | Description |
|---|---|
| **Drag & Drop PDF Import** | Drop PDF files onto the window — DOI extraction and Crossref metadata lookup happen automatically |
| **One-Click Cite** | Click "Cite" on a paper to copy the in-text citation and track it for the bibliography |
| **Copy Bibliography** | Generate and copy a formatted bibliography of all cited papers in one click |
| **Export .bib** | Export your entire library as a BibTeX file via native save dialog |
| **Global Command Palette** | Summon your library over any application with <kbd>Ctrl+Shift+C</kbd> |
| **Word & PowerPoint Add-in** | Built-in Office Web Add-in — search and insert citations directly at your cursor |
| **7 Citation Styles** | APA 7th, MLA 9th, Chicago, IEEE, Harvard, Vancouver, and BibTeX with rich text formatting |
| **Manual Entry** | Add references without a PDF — fill in metadata manually |
| **Inline Editing** | Edit any reference's metadata directly from the library view |
| **Local-First Storage** | No cloud lock-in. Everything lives in a fast SQLite database on your machine |
| **Instant Search** | Full-text search across your entire library with zero perceptible lag |
| **Smart Clipboard** | Citations copied as rich text (preserves italics in Word/Docs) or plain text (LaTeX/Markdown) |
| **Bibliography Builder** | Select specific references, reorder them, preview and export a formatted bibliography |
| **Collection Management** | Organize references into color-coded folders |
| **Dark Mode** | Automatic dark theme that follows your system preference |

---

## Office Add-in

CiteStrike includes a **built-in Office Web Add-in** for Microsoft Word and PowerPoint — no separate install needed.

**How it works:**
1. CiteStrike runs a local API server on `localhost:27182` automatically
2. Sideload the add-in manifest in Word/PowerPoint via **Insert → Get Add-ins → Upload My Add-in**
3. Download the manifest from `http://localhost:27182/manifest.xml` (save as XML file)
4. The CiteStrike panel appears in the Home ribbon — search your library and insert at cursor

The add-in uses the official **Office.js** API to insert text at your cursor position — no clipboard hijacking or simulated keystrokes.

---

## Tech Stack

### Backend — Core Engine (Rust)

| Crate / Tool | Purpose |
|---|---|
| [Rust](https://www.rust-lang.org/) | Memory safety and absolute performance |
| [Tauri v2](https://tauri.app/) | Tiny, cross-platform native binary (~5 MB installed) |
| `rusqlite` | Instantaneous local database queries |
| `lopdf` | PDF text extraction and DOI parsing |
| `reqwest` + `tokio` | Async API calls to the Crossref metadata service |
| `axum` | Local HTTP API server powering the Office add-in |
| `clipboard-win` | Rich text (RTF) clipboard support for formatted citations |
| `tauri-plugin-dialog` | Native file dialogs for BibTeX export |
| Custom citation engine | Hand-tuned formatters for APA, MLA, Chicago, IEEE, Harvard, Vancouver, BibTeX |

### Frontend — UI (Svelte)

| Library | Purpose |
|---|---|
| [SvelteKit](https://kit.svelte.dev/) | Compiles to tiny vanilla JS — zero virtual DOM overhead |
| [Tailwind CSS](https://tailwindcss.com/) | Sleek, modern UI with automatic dark mode |
| [Svelte 5 Runes](https://svelte.dev/docs/svelte/$state) | Fine-grained reactivity with `$state`, `$effect`, `$props` |

---

## Architecture

```
┌──────────────────────────────────────────────────────┐
│                   CiteStrike App                      │
│               (runs in background)                    │
└──────────┬────────────────────────┬──────────────────┘
           │  Ctrl+Shift+C         │  localhost:27182
           ▼                       ▼
┌─────────────────────┐  ┌─────────────────────────────┐
│  Command Palette    │  │  Office Web Add-in          │
│  (Svelte+Tailwind)  │  │  (Word & PowerPoint panel)  │
└──────────┬──────────┘  └──────────┬──────────────────┘
           │  IPC (Tauri)           │  REST API (axum)
           └──────────┬─────────────┘
                      ▼
┌──────────────────────────────────────────────────────┐
│              Rust Core Engine                         │
│                                                      │
│  ┌──────────┐  ┌──────────┐  ┌───────────────────┐  │
│  │  SQLite  │  │ PDF      │  │ Crossref           │  │
│  │  Store   │  │ Parser   │  │ API                │  │
│  └──────────┘  └──────────┘  └───────────────────┘  │
│  ┌────────────────────┐  ┌────────────────────────┐  │
│  │  Citation Engine   │  │ RTF Clipboard          │  │
│  │  (7 styles)        │  │ (rich text copy)       │  │
│  └────────────────────┘  └────────────────────────┘  │
└──────────────────────────────────────────────────────┘
```

### Database Schema

```sql
refs (
    id, title, authors, year, doi, journal, volume, issue, pages,
    abstract_text, url, ref_type, bibtex_key, folder_id, created_at,
    cited,        -- 0 = in library, 1 = cited in current document
    cite_order    -- order of first citation (for IEEE/Vancouver numbering)
)

folders (id, name, color, created_at)
```

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (via `rustup`)
- [Node.js](https://nodejs.org/) v18+
- OS-specific Tauri dependencies — see the [Tauri v2 Prerequisites](https://v2.tauri.app/start/prerequisites/)

### Development

```bash
# Clone the repository
git clone https://github.com/hasnainqau5112/citestrike.git
cd citestrike

# Install frontend dependencies
npm install

# Run in development mode
npx tauri dev
```

### Build for Production

```bash
npx tauri build
```

The installer will be output to `src-tauri/target/release/bundle/`.

---

## Usage

### Quick Start

1. **Add papers** — Drag PDFs onto the window, paste a DOI, search Crossref, or enter metadata manually
2. **Cite** — Click the **Cite** button on a reference to copy the in-text citation to your clipboard
3. **Paste** — Switch to your document and press <kbd>Ctrl+V</kbd>
4. **Repeat** — Cite more papers as you write
5. **Bibliography** — Click **Copy Bibliography** to get the formatted reference list, then paste at the end of your document
6. **Export** — Click **Export .bib** to save your entire library as a BibTeX file

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| <kbd>Ctrl+Shift+C</kbd> | Toggle CiteStrike window (global, works from any app) |
| <kbd>↑</kbd> / <kbd>↓</kbd> | Navigate references in the library |
| <kbd>Enter</kbd> | Submit search / add DOI |

---

## CI/CD

CiteStrike uses automated GitHub Actions pipelines:

| Workflow | Trigger | What it does |
|----------|---------|--------------|
| **CI** (`ci.yml`) | Push / PR to `main` | Validates frontend build, TypeScript checks, Rust compilation |
| **Release** (`release.yml`) | Push to `main` | Auto-bumps patch version, builds Windows installers, publishes GitHub Release |

Versioning is fully automatic — every push to `main` increments the patch version and creates a new release with `.msi` and `.exe` installers attached.

---

## Roadmap

- [x] Tauri v2 project with SQLite CRUD
- [x] PDF drag-and-drop import with DOI extraction
- [x] Crossref API integration
- [x] Command palette UI (Svelte + Tailwind)
- [x] Global hotkey (Ctrl+Shift+C)
- [x] Citation formatting (APA, MLA, Chicago, IEEE, Harvard, Vancouver, BibTeX)
- [x] Rich text clipboard (RTF with italics)
- [x] One-click cite with citation tracking
- [x] Auto-generated bibliography from cited papers
- [x] Manual reference entry
- [x] Inline reference editing
- [x] Full library export to `.bib`
- [x] Batch bibliography builder
- [x] Office Web Add-in for Word & PowerPoint
- [x] Folder/collection management
- [x] Dark mode (system preference)
- [x] CI/CD pipeline with auto-releases
- [ ] CSL (Citation Style Language) file support
- [ ] System tray with background mode
- [ ] Google Docs integration
- [ ] Duplicate detection
- [ ] BibTeX file import

---

## Contributing

Contributions are welcome! If you'd like to help kill bloated reference managers:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## License

Distributed under the **MIT License**. See `LICENSE` for more information.

---

<p align="center">
  <sub>Built with Rust, Tauri, and a deep hatred for slow citation managers.</sub>
</p>
