<p align="center">
  <img src="./logo.svg" alt="CiteStrike Logo" width="150" />
</p>

<h1 align="center">CiteStrike</h1>

<p align="center">
  <strong>A blazing-fast, local-first, invisible citation manager built with Rust & Tauri — the "Raycast" of reference management.</strong>
</p>

<p align="center">
  <a href="#"><img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" /></a>
  <a href="#"><img src="https://img.shields.io/badge/tauri-%2324C8DB.svg?style=for-the-badge&logo=tauri&logoColor=%23FFFFFF" alt="Tauri" /></a>
  <a href="#"><img src="https://img.shields.io/badge/svelte-%23f1413d.svg?style=for-the-badge&logo=svelte&logoColor=white" alt="Svelte" /></a>
  <a href="#"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge" alt="License" /></a>
</p>

<br />

<p align="center">
  <img src="https://img.shields.io/badge/RAM-<_15MB-2ecc71?style=flat-square" alt="RAM < 15MB" />
  <img src="https://img.shields.io/badge/startup-<_200ms-2ecc71?style=flat-square" alt="Startup < 200ms" />
  <img src="https://img.shields.io/badge/platform-Windows_|_macOS_|_Linux-lightgrey?style=flat-square" alt="Cross-platform" />
</p>

---

## The Problem

Current citation managers — EndNote, Mendeley, Zotero — are heavy, bloated, and require constant context switching. Every time you need a reference you leave your writing flow, dig through a clunky GUI, copy text, switch back, and paste. Multiply that by hundreds of citations in a thesis and you've lost hours.

## The Solution

**CiteStrike** runs silently in the background consuming **< 15 MB** of RAM. When you need a citation in *any* application — Word, Google Docs, Overleaf, Obsidian, Notion — press:

<p align="center">
  <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>C</kbd> &nbsp;/&nbsp; <kbd>Cmd</kbd> + <kbd>Shift</kbd> + <kbd>C</kbd>
</p>

A spotlight-style command palette appears instantly. Type the author or title, hit <kbd>Enter</kbd>, and the perfectly formatted citation lands in your clipboard — or directly into your document. No window switching. No friction.

---

## Key Features

| Feature | Description |
|---|---|
| **Global Command Palette** | Summon your library over any application. Never leave your writing flow. |
| **Auto-Magic PDF Import** | Drag & drop a PDF — CiteStrike extracts the DOI via Rust, queries Crossref, and saves perfect metadata in milliseconds. |
| **Local-First Storage** | No cloud lock-in. Everything lives in a fast SQLite database with automatic `.bib` (BibTeX) file sync. |
| **Instant Search** | Full-text fuzzy search across 10,000+ papers with zero perceptible lag. |
| **Universal Citation Styles** | APA 7th, MLA 9th, Chicago, IEEE, Harvard, Vancouver, and BibTeX. |
| **Smart Clipboard** | Formatted citations copied as rich text (for Word/Docs) or plain text (for LaTeX/Markdown) based on context. |
| **Batch Operations** | Select multiple references and generate a full bibliography section in one action. |

---

## Tech Stack

CiteStrike is designed to be as lightweight and performant as possible.

### Backend — Core Engine (Rust)

| Crate / Tool | Purpose |
|---|---|
| [Rust](https://www.rust-lang.org/) | Memory safety and absolute performance |
| [Tauri v2](https://tauri.app/) | Tiny, cross-platform native binary (~5 MB installed) |
| `rusqlite` | Instantaneous local database queries |
| `lopdf` | PDF text extraction and DOI parsing |
| `reqwest` + `tokio` | Async API calls to the Crossref metadata service |
| `global-hotkey` | OS-level shortcut interception for the command palette |
| Custom citation engine | Hand-tuned formatters for APA, MLA, Chicago, IEEE, Harvard, Vancouver, BibTeX |

### Frontend — Command Palette (Svelte)

| Library | Purpose |
|---|---|
| [SvelteKit](https://kit.svelte.dev/) | Compiles to tiny vanilla JS — zero virtual DOM overhead |
| [Tailwind CSS](https://tailwindcss.com/) | Sleek, modern UI with built-in dark mode |

---

## Architecture

```
┌──────────────────────────────────────────────────────┐
│                   System Tray                        │
│               (runs in background)                   │
└──────────────┬───────────────────────────────────────┘
               │  Ctrl+Shift+C
               ▼
┌──────────────────────────────────────────────────────┐
│            Command Palette (Svelte + Tailwind)       │
│  ┌────────────────────────────────────────────────┐  │
│  │  Search papers...                              │  │
│  └────────────────────────────────────────────────┘  │
│    Smith et al. (2024) — Deep Learning Survey        │
│    Johnson (2023) — Neural Architecture Search       │
└──────────────┬───────────────────────────────────────┘
               │  IPC (Tauri Commands)
               ▼
┌──────────────────────────────────────────────────────┐
│              Rust Core Engine                         │
│                                                      │
│  ┌──────────┐  ┌──────────┐  ┌───────────────────┐  │
│  │  SQLite  │  │ PDF      │  │ Crossref           │  │
│  │  Store   │  │ Parser   │  │ API                │  │
│  └──────────┘  └──────────┘  └───────────────────┘  │
│  ┌──────────┐  ┌──────────────────────────────────┐  │
│  │  CSL     │  │ BibTeX Sync (.bib auto-export)  │  │
│  │  Engine  │  │                                  │  │
│  └──────────┘  └──────────────────────────────────┘  │
└──────────────────────────────────────────────────────┘
```

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (via `rustup`)
- [Node.js](https://nodejs.org/) v18+
- OS-specific Tauri dependencies — see the [Tauri v2 Prerequisites](https://v2.tauri.app/start/prerequisites/)

### Installation

```bash
# Clone the repository
git clone https://github.com/hasnainqau5112/citestrike.git
cd citestrike

# Install frontend dependencies
npm install

# Run in development mode
cargo tauri dev
```

### Build for Production

```bash
cargo tauri build
```

The installer will be output to `src-tauri/target/release/bundle/`.

---

## Roadmap

- [x] Project scaffolding & README
- [x] Tauri v2 project initialization
- [x] SQLite database schema & CRUD operations
- [x] PDF import with DOI extraction (`lopdf`)
- [x] Crossref API integration for metadata fetching
- [x] Command palette UI (Svelte + Tailwind)
- [x] Global hotkey registration
- [x] Citation formatting engine (APA / MLA / Chicago / IEEE / Harvard / Vancouver / BibTeX)
- [x] Clipboard injection (rich text + plain text)
- [x] Batch bibliography generation
- [x] Office Web Add-in for Word & PowerPoint
- [x] Folder/collection management
- [ ] Auto `.bib` file sync
- [ ] System tray with background mode
- [ ] Full library export to `.bib`

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
