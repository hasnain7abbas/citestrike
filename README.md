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

## Key Features

| Feature | Description |
|---|---|
| **Global Command Palette** | Summon your library over any application with <kbd>Ctrl+Shift+C</kbd>. Never leave your writing flow. |
| **Word & PowerPoint Add-in** | A built-in Office Web Add-in — search and insert citations directly at your cursor, just like EndNote. |
| **7 Citation Styles** | APA 7th, MLA 9th, Chicago, IEEE, Harvard, Vancouver, and BibTeX — with rich text formatting. |
| **Auto PDF Import** | Drag & drop a PDF — CiteStrike extracts the DOI via Rust, queries Crossref, and saves perfect metadata. |
| **Local-First Storage** | No cloud lock-in. Everything lives in a fast SQLite database on your machine. |
| **Instant Search** | Full-text fuzzy search across your entire library with zero perceptible lag. |
| **Smart Clipboard** | Citations copied as rich text (preserves italics in Word/Docs) or plain text (for LaTeX/Markdown). |
| **Bibliography Builder** | Select multiple references, reorder them, preview and export a formatted bibliography. |
| **Collection Management** | Organize references into color-coded folders. Rename, move, and delete with ease. |
| **Dark Mode** | Automatic dark theme that follows your system preference. |

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
| Custom citation engine | Hand-tuned formatters for APA, MLA, Chicago, IEEE, Harvard, Vancouver, BibTeX |

### Frontend — UI (Svelte)

| Library | Purpose |
|---|---|
| [SvelteKit](https://kit.svelte.dev/) | Compiles to tiny vanilla JS — zero virtual DOM overhead |
| [Tailwind CSS](https://tailwindcss.com/) | Sleek, modern UI with automatic dark mode |

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
- [x] PDF import with DOI extraction
- [x] Crossref API integration
- [x] Command palette UI (Svelte + Tailwind)
- [x] Global hotkey (Ctrl+Shift+C)
- [x] Citation formatting (APA, MLA, Chicago, IEEE, Harvard, Vancouver, BibTeX)
- [x] Rich text clipboard (RTF with italics)
- [x] Batch bibliography builder
- [x] Office Web Add-in for Word & PowerPoint
- [x] Folder/collection management
- [x] Dark mode (system preference)
- [x] CI/CD pipeline with auto-releases
- [ ] Auto `.bib` file sync
- [ ] System tray with background mode
- [ ] Full library export to `.bib`
- [ ] Google Docs integration

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
