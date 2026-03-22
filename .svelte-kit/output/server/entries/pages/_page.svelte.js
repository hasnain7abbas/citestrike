import { a1 as attr_class, a2 as ensure_array_like, a3 as attr, a4 as stringify, a5 as attr_style, e as escape_html, a6 as bind_props } from "../../chunks/index.js";
import "clsx";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
function html(value) {
  var html2 = String(value ?? "");
  var open = "<!---->";
  return open + html2 + "<!---->";
}
function TitleBar($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const appWindow = getCurrentWindow();
    let maximized = false;
    appWindow.isMaximized().then((v) => maximized = v);
    $$renderer2.push(`<div class="h-10 flex items-center justify-between bg-[var(--bg-sidebar)] border-b border-[var(--border-light)] select-none shrink-0" data-tauri-drag-region=""><div class="flex items-center gap-2.5 pl-4" data-tauri-drag-region=""><svg class="w-5 h-5 shrink-0" viewBox="0 0 512 512" fill="none"><rect width="512" height="512" rx="115" fill="url(#tb-bg)"></rect><path d="M 190 160 Q 130 256 190 352" stroke="#94A3B8" stroke-width="28" stroke-linecap="round" fill="none" opacity="0.5"></path><path d="M 322 160 Q 382 256 322 352" stroke="#94A3B8" stroke-width="28" stroke-linecap="round" fill="none" opacity="0.5"></path><path d="M 275 140 L 210 270 L 255 270 L 235 380 L 305 245 L 260 245 Z" fill="url(#tb-bolt)"></path><defs><linearGradient id="tb-bg" x1="0%" y1="0%" x2="100%" y2="100%"><stop offset="0%" stop-color="#0F172A"></stop><stop offset="100%" stop-color="#020617"></stop></linearGradient><linearGradient id="tb-bolt" x1="0%" y1="0%" x2="100%" y2="100%"><stop offset="0%" stop-color="#3B6CF0"></stop><stop offset="100%" stop-color="#38BDF8"></stop></linearGradient></defs></svg> <span class="text-[var(--text)] font-semibold text-[13px] tracking-tight" data-tauri-drag-region="">CiteStrike</span> <span class="text-[var(--text-muted)] text-[10px] font-medium tracking-wider uppercase ml-1" data-tauri-drag-region="">v0.1</span></div> <div class="flex items-center h-full"><button class="h-full w-11 flex items-center justify-center hover:bg-[var(--bg-hover)] transition-colors"><svg class="w-3 h-3 text-[var(--text-secondary)]" fill="none" viewBox="0 0 12 12"><path d="M2 6h8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"></path></svg></button> <button class="h-full w-11 flex items-center justify-center hover:bg-[var(--bg-hover)] transition-colors">`);
    if (maximized) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<svg class="w-3 h-3 text-[var(--text-secondary)]" fill="none" viewBox="0 0 12 12"><rect x="3" y="0.5" width="8.5" height="8.5" rx="1" stroke="currentColor" stroke-width="1.2"></rect><rect x="0.5" y="3" width="8.5" height="8.5" rx="1" stroke="currentColor" stroke-width="1.2" fill="var(--bg-sidebar)"></rect></svg>`);
    } else {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<svg class="w-3 h-3 text-[var(--text-secondary)]" fill="none" viewBox="0 0 12 12"><rect x="1" y="1" width="10" height="10" rx="1.5" stroke="currentColor" stroke-width="1.5"></rect></svg>`);
    }
    $$renderer2.push(`<!--]--></button> <button class="h-full w-11 flex items-center justify-center hover:bg-[var(--danger)] hover:text-white transition-colors rounded-tr-none"><svg class="w-3 h-3 text-[var(--text-secondary)]" fill="none" viewBox="0 0 12 12"><path d="M2 2l8 8M10 2l-8 8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"></path></svg></button></div></div>`);
  });
}
async function searchReferences(query, folderId) {
  return invoke("search_references", { query, folderId: null });
}
async function searchOnline(query) {
  return invoke("search_online", { query });
}
async function getFolders() {
  return invoke("get_folders");
}
async function formatBatchBibliography(references, style) {
  return invoke("format_batch_bibliography", { references, style });
}
function Sidebar($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { folders = [], activeView = "all", activeFolderId = null } = $$props;
    let editingId = null;
    let editName = "";
    async function loadFolders() {
      folders = await getFolders();
    }
    loadFolders();
    $$renderer2.push(`<div class="w-52 bg-[var(--bg-sidebar)] border-r border-[var(--border-light)] flex flex-col h-full shrink-0 select-none"><div class="px-2 pt-3 pb-1"><p class="px-2 text-[10px] font-semibold text-[var(--text-muted)] uppercase tracking-wider mb-1.5">Library</p> <button${attr_class(`w-full flex items-center gap-2.5 px-2.5 py-[7px] rounded-[var(--radius-sm)] text-[12px] transition-colors ${stringify(activeView === "all" ? "bg-[var(--accent-light)] text-[var(--accent)] font-medium" : "text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]")}`)}><svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8"><path stroke-linecap="round" stroke-linejoin="round" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path></svg> All References</button> <button${attr_class(`w-full flex items-center gap-2.5 px-2.5 py-[7px] rounded-[var(--radius-sm)] text-[12px] transition-colors ${stringify(activeView === "add-doi" ? "bg-[var(--accent-light)] text-[var(--accent)] font-medium" : "text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]")}`)}><svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8"><path stroke-linecap="round" stroke-linejoin="round" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"></path></svg> Add by DOI</button> <button${attr_class(`w-full flex items-center gap-2.5 px-2.5 py-[7px] rounded-[var(--radius-sm)] text-[12px] transition-colors ${stringify(activeView === "online" ? "bg-[var(--accent-light)] text-[var(--accent)] font-medium" : "text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]")}`)}><svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8"><path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg> Search Online</button> <button${attr_class(`w-full flex items-center gap-2.5 px-2.5 py-[7px] rounded-[var(--radius-sm)] text-[12px] transition-colors ${stringify(activeView === "bibliography" ? "bg-[var(--accent-light)] text-[var(--accent)] font-medium" : "text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]")}`)}><svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg> Build Bibliography</button> <button${attr_class(`w-full flex items-center gap-2.5 px-2.5 py-[7px] rounded-[var(--radius-sm)] text-[12px] transition-colors ${stringify(activeView === "help" ? "bg-[var(--accent-light)] text-[var(--accent)] font-medium" : "text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]")}`)}><svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8"><path stroke-linecap="round" stroke-linejoin="round" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg> Help Guide</button></div> <div class="px-2 mt-3 flex-1 overflow-y-auto"><div class="flex items-center justify-between px-2 mb-1.5"><p class="text-[10px] font-semibold text-[var(--text-muted)] uppercase tracking-wider">Collections</p> <button class="p-0.5 rounded hover:bg-[var(--bg-hover)] transition-colors"><svg class="w-3.5 h-3.5 text-[var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" d="M12 5v14m-7-7h14"></path></svg></button></div> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> <!--[-->`);
    const each_array = ensure_array_like(folders);
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let folder = each_array[$$index];
      $$renderer2.push(`<div class="relative group">`);
      if (editingId === folder.id) {
        $$renderer2.push("<!--[0-->");
        $$renderer2.push(`<input${attr("value", editName)} class="w-full px-2 py-1.5 text-[11px] border border-[var(--accent)] rounded-[var(--radius-sm)] outline-none bg-[var(--bg-input)] ml-1"/>`);
      } else {
        $$renderer2.push("<!--[-1-->");
        $$renderer2.push(`<button${attr_class(`w-full flex items-center gap-2.5 px-2.5 py-[7px] rounded-[var(--radius-sm)] text-[12px] transition-colors ${stringify(activeView === "folder" && activeFolderId === folder.id ? "bg-[var(--accent-light)] text-[var(--accent)] font-medium" : "text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]")}`)}><span class="w-2.5 h-2.5 rounded-full shrink-0"${attr_style(`background:${stringify(folder.color)}`)}></span> <span class="truncate">${escape_html(folder.name)}</span></button> <div class="absolute right-1 top-1/2 -translate-y-1/2 flex opacity-0 group-hover:opacity-100 transition-opacity"><button class="p-1 rounded hover:bg-[var(--bg-active)] transition-colors"><svg class="w-3 h-3 text-[var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"></path><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"></path></svg></button> <button class="p-1 rounded hover:bg-[var(--danger-light)] transition-colors"><svg class="w-3 h-3 text-[var(--text-muted)] hover:text-[var(--danger)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2"><path stroke-linecap="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg></button></div>`);
      }
      $$renderer2.push(`<!--]--></div>`);
    }
    $$renderer2.push(`<!--]--> `);
    if (folders.length === 0 && true) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<p class="px-2.5 text-[11px] text-[var(--text-muted)] italic">No collections yet</p>`);
    } else {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--></div> <div class="px-4 py-3 border-t border-[var(--border-light)]"><div class="flex items-center gap-2 text-[10px] text-[var(--text-muted)]"><kbd class="px-1 py-0.5 rounded border border-[var(--border)] bg-[var(--bg-card)] text-[9px] font-mono">Ctrl+Shift+C</kbd> <span>Quick access</span></div></div></div>`);
    bind_props($$props, { folders, activeView, activeFolderId });
  });
}
function SearchBar($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { value = "", onsubmit, placeholder = "Search references..." } = $$props;
    $$renderer2.push(`<div class="relative"><svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-[var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg> <input${attr("value", value)}${attr("placeholder", placeholder)} class="w-full bg-[var(--bg-input)] text-[var(--text)] placeholder-[var(--text-muted)] pl-9 pr-3 py-2 text-[13px] rounded-[var(--radius-sm)] border border-[var(--border)] outline-none focus:border-[var(--accent)] focus:shadow-[0_0_0_3px_var(--accent-light)] transition-all"/></div>`);
    bind_props($$props, { value });
  });
}
function CitationItem($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { reference, selected = false } = $$props;
    let copied = "";
    $$renderer2.push(`<div${attr_class(`group relative px-4 py-3 border-b border-[var(--border-light)] hover:bg-[var(--bg-hover)] transition-colors ${stringify(selected ? "bg-[var(--accent-light)] border-l-2 border-l-[var(--accent)]" : "")}`)}><div class="flex items-start justify-between gap-3"><div class="min-w-0 flex-1"><h3 class="text-[var(--text)] font-medium text-[13px] leading-snug line-clamp-2">${escape_html(reference.title)}</h3> <p class="text-[var(--text-secondary)] text-[11px] mt-1 truncate">${escape_html(reference.authors)}</p> <div class="flex items-center gap-2 mt-1.5 flex-wrap">`);
    if (reference.year) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<span class="inline-flex items-center px-1.5 py-0.5 rounded text-[10px] font-medium bg-[var(--bg-active)] text-[var(--text-secondary)]">${escape_html(reference.year)}</span>`);
    } else {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> `);
    if (reference.journal) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<span class="text-[10px] text-[var(--text-muted)] italic truncate max-w-[200px]">${escape_html(reference.journal)}</span>`);
    } else {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> `);
    if (reference.doi) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<span class="text-[10px] text-[var(--accent)] truncate max-w-[150px]">${escape_html(reference.doi)}</span>`);
    } else {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--></div> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--></div> <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity shrink-0 mt-0.5"><!--[-->`);
    const each_array = ensure_array_like(["APA", "MLA", "BibTeX"]);
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let style = each_array[$$index];
      $$renderer2.push(`<button${attr_class(`px-2 py-1 text-[10px] font-medium rounded-[var(--radius-sm)] ${stringify(copied === style ? "bg-[var(--success-light)] text-[var(--success)]" : "bg-[var(--bg-active)] text-[var(--text-secondary)] hover:bg-[var(--accent-light)] hover:text-[var(--accent)]")} transition-colors`)}${attr("title", `Copy ${stringify(style)} (rich text)`)}>${escape_html(copied === style ? "✓" : style === "BibTeX" ? "BIB" : style)}</button>`);
    }
    $$renderer2.push(`<!--]--> <div class="relative"><button class="p-1 rounded-[var(--radius-sm)] hover:bg-[var(--bg-active)] transition-colors" title="More options"><svg class="w-3.5 h-3.5 text-[var(--text-muted)]" fill="currentColor" viewBox="0 0 16 16"><circle cx="8" cy="3" r="1.5"></circle><circle cx="8" cy="8" r="1.5"></circle><circle cx="8" cy="13" r="1.5"></circle></svg></button> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--></div></div></div></div>`);
  });
}
function HelpGuide($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let openSection = "getting-started";
    const sections = [
      {
        id: "getting-started",
        title: "Getting Started",
        icon: "🚀",
        content: [
          {
            q: "What is CiteStrike?",
            a: "CiteStrike is a fast, local-first citation manager. Your references live in a local SQLite database on your machine — no cloud accounts, no subscriptions, no data leaving your computer."
          },
          {
            q: "How do I add my first reference?",
            a: 'You have three options:\n\n1. **Add by DOI** — Click "Add by DOI" in the sidebar, paste a DOI (e.g. 10.1038/s41586-021-03819-2), and press Enter. CiteStrike fetches all metadata from Crossref automatically.\n\n2. **Search Online** — Click "Search Online", type a paper title or author name, and click "Add" on any result.\n\n3. **Import PDF** — Drag and drop a PDF file. CiteStrike will extract the DOI from the document and auto-fetch the metadata.'
          },
          {
            q: "Where is my data stored?",
            a: "All references are stored in a local SQLite database file (citestrike.db) in your system's app data directory. Nothing is sent to external servers except DOI lookups to the free Crossref API."
          }
        ]
      },
      {
        id: "library",
        title: "Managing Your Library",
        icon: "📚",
        content: [
          {
            q: "How do I organize references?",
            a: 'Use **Collections** (folders) in the sidebar. Click the + button next to "Collections" to create a new folder. You can move references between folders using the ⋮ menu on each reference card.'
          },
          {
            q: "How do I delete a reference?",
            a: 'Hover over any reference card to reveal action buttons. Click the ⋮ (three dots) menu and select "Delete reference". This permanently removes it from your library.'
          },
          {
            q: "How do I search my library?",
            a: "Use the search bar at the top. It searches across titles, authors, DOIs, and BibTeX keys. When viewing a specific collection, search is scoped to that collection only."
          },
          {
            q: "How do I rename or delete a collection?",
            a: 'Hover over any collection in the sidebar to reveal edit (pencil) and delete (trash) icons. Deleting a collection moves its references back to "All References" — references are never deleted when you remove a collection.'
          },
          {
            q: "Can I move references between collections?",
            a: 'Yes. Click the ⋮ menu on any reference and select "Move to folder...". You can also move references to no folder (All References).'
          }
        ]
      },
      {
        id: "citations",
        title: "Copying Citations",
        icon: "📋",
        content: [
          {
            q: "How do I copy a formatted citation?",
            a: "Hover over any reference to see format buttons: **APA**, **MLA**, and **BIB**. Click one to instantly copy the formatted citation to your clipboard. For more formats (Chicago, IEEE, Harvard), click the ⋮ menu."
          },
          {
            q: "What citation styles are supported?",
            a: "**Six styles** are built in:\n\n• **APA** (7th edition) — Psychology, social sciences\n• **MLA** (9th edition) — Humanities, literature\n• **Chicago** — History, publishing\n• **IEEE** — Engineering, computer science\n• **Harvard** — General academic\n• **BibTeX** — LaTeX documents"
          },
          {
            q: "How do I paste into my document?",
            a: "After clicking a format button, the citation is on your clipboard. Simply Ctrl+V (or Cmd+V) into Word, Google Docs, Overleaf, Obsidian, Notion, or any other editor."
          }
        ]
      },
      {
        id: "shortcuts",
        title: "Keyboard Shortcuts",
        icon: "⌨️",
        content: [
          {
            q: "Available shortcuts",
            a: "• **Ctrl+Shift+C** — Toggle CiteStrike window (global, works from any app)\n• **Enter** — Submit DOI / Confirm search\n• **Escape** — Close menus\n• **Arrow Up/Down** — Navigate search results"
          }
        ]
      },
      {
        id: "online-search",
        title: "Searching Online",
        icon: "🌐",
        content: [
          {
            q: "Where does online search look?",
            a: "CiteStrike searches the **Crossref** database, which indexes over 130 million scholarly works from thousands of publishers worldwide."
          },
          {
            q: "Why can't I find a paper?",
            a: "Some papers may not be indexed in Crossref (e.g., preprints, very recent publications, or papers from certain publishers). Try searching by title keywords instead of full title. You can also add papers manually by DOI."
          },
          {
            q: "Is there a rate limit?",
            a: "Crossref's public API is free and generous, but please avoid rapid-fire searches. CiteStrike makes polite requests with a proper User-Agent header."
          }
        ]
      },
      {
        id: "data",
        title: "Data & Privacy",
        icon: "🔒",
        content: [
          {
            q: "Is my data private?",
            a: "Yes. CiteStrike stores everything locally on your machine. The only outbound network requests are to the Crossref API when you explicitly search online or fetch a DOI."
          },
          {
            q: "Can I export my library?",
            a: "You can copy individual references in BibTeX format using the BIB button. Full library export to .bib files is planned for a future update."
          },
          {
            q: "Can I back up my library?",
            a: "Your database file (citestrike.db) is in your app data directory. You can copy this file to back up your entire library."
          }
        ]
      }
    ];
    $$renderer2.push(`<div class="h-full overflow-y-auto"><div class="max-w-2xl mx-auto px-6 py-6"><div class="mb-8"><h1 class="text-xl font-bold text-[var(--text)] mb-1">Help Guide</h1> <p class="text-[var(--text-secondary)] text-[13px]">Everything you need to know about using CiteStrike</p></div> <div class="bg-[var(--accent-light)] border border-[var(--accent)]/20 rounded-[var(--radius)] p-4 mb-6"><div class="flex gap-3"><span class="text-lg">⚡</span> <div><p class="font-semibold text-[var(--accent)] text-[13px] mb-1">Quick Start</p> <p class="text-[12px] text-[var(--text-secondary)] leading-relaxed">Click <strong>"Add by DOI"</strong> in the sidebar, paste any DOI, and press Enter. Your first reference will appear in the library in seconds.
						Or click <strong>"Search Online"</strong> to find papers by title or author.</p></div></div></div> <div class="space-y-2"><!--[-->`);
    const each_array = ensure_array_like(sections);
    for (let $$index_1 = 0, $$length = each_array.length; $$index_1 < $$length; $$index_1++) {
      let section = each_array[$$index_1];
      $$renderer2.push(`<div class="border border-[var(--border-light)] rounded-[var(--radius-sm)] overflow-hidden"><button class="w-full flex items-center gap-3 px-4 py-3 text-left hover:bg-[var(--bg-hover)] transition-colors"><span class="text-base">${escape_html(section.icon)}</span> <span class="flex-1 font-medium text-[13px] text-[var(--text)]">${escape_html(section.title)}</span> <svg${attr_class(`w-4 h-4 text-[var(--text-muted)] transition-transform ${stringify(openSection === section.id ? "rotate-180" : "")}`)} fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg></button> `);
      if (openSection === section.id) {
        $$renderer2.push("<!--[0-->");
        $$renderer2.push(`<div class="border-t border-[var(--border-light)] bg-[var(--bg-card)]"><!--[-->`);
        const each_array_1 = ensure_array_like(section.content);
        for (let i = 0, $$length2 = each_array_1.length; i < $$length2; i++) {
          let item = each_array_1[i];
          $$renderer2.push(`<div${attr_class(`px-4 py-3 ${stringify(i > 0 ? "border-t border-[var(--border-light)]" : "")}`)}><p class="font-medium text-[12px] text-[var(--text)] mb-1.5">${escape_html(item.q)}</p> <div class="text-[12px] text-[var(--text-secondary)] leading-relaxed whitespace-pre-line">${html(item.a.replace(/\*\*(.*?)\*\*/g, '<strong style="color:var(--text)">$1</strong>').replace(/\n/g, "<br/>").replace(/• /g, '<span style="display:inline-block;width:16px">•</span>'))}</div></div>`);
        }
        $$renderer2.push(`<!--]--></div>`);
      } else {
        $$renderer2.push("<!--[-1-->");
      }
      $$renderer2.push(`<!--]--></div>`);
    }
    $$renderer2.push(`<!--]--></div> <div class="mt-8 text-center text-[11px] text-[var(--text-muted)]"><p>CiteStrike v0.1.0 — Built with Rust, Tauri &amp; Svelte</p> <p class="mt-1">Report issues at github.com/yourusername/citestrike</p></div></div></div>`);
  });
}
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let query = "";
    let results = [];
    let selectedIndex = 0;
    let loading = false;
    let activeView = "all";
    let activeFolderId = null;
    let folders = [];
    let doiInput = "";
    let onlineQuery = "";
    let onlineResults = [];
    let bibRefs = [];
    let bibStyle = "APA";
    let bibSearch = "";
    let bibSearchResults = [];
    let bibPreview = "";
    async function handleOnlineSearch() {
      if (!onlineQuery.trim()) return;
      loading = true;
      try {
        onlineResults = await searchOnline(onlineQuery);
      } catch {
        onlineResults = [];
      }
      loading = false;
    }
    let $$settled = true;
    let $$inner_renderer;
    function $$render_inner($$renderer3) {
      $$renderer3.push(`<div class="h-screen w-screen flex flex-col bg-[var(--bg)] overflow-hidden">`);
      TitleBar($$renderer3);
      $$renderer3.push(`<!----> <div class="flex flex-1 min-h-0">`);
      Sidebar($$renderer3, {
        get folders() {
          return folders;
        },
        set folders($$value) {
          folders = $$value;
          $$settled = false;
        },
        get activeView() {
          return activeView;
        },
        set activeView($$value) {
          activeView = $$value;
          $$settled = false;
        },
        get activeFolderId() {
          return activeFolderId;
        },
        set activeFolderId($$value) {
          activeFolderId = $$value;
          $$settled = false;
        }
      });
      $$renderer3.push(`<!----> <div class="flex-1 flex flex-col min-w-0">`);
      if (activeView === "help") {
        $$renderer3.push("<!--[0-->");
        HelpGuide($$renderer3);
      } else if (activeView === "add-doi") {
        $$renderer3.push("<!--[1-->");
        $$renderer3.push(`<div class="flex-1 flex flex-col items-center justify-center px-6"><div class="w-full max-w-md"><div class="text-center mb-6"><div class="w-12 h-12 mx-auto mb-3 rounded-[var(--radius)] bg-[var(--accent-light)] flex items-center justify-center"><svg class="w-6 h-6 text-[var(--accent)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8"><path stroke-linecap="round" stroke-linejoin="round" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"></path></svg></div> <h2 class="text-base font-semibold text-[var(--text)]">Add Reference by DOI</h2> <p class="text-[12px] text-[var(--text-secondary)] mt-1">Paste a DOI and press Enter to auto-fetch metadata</p></div> <div class="relative"><input${attr("value", doiInput)} placeholder="e.g. 10.1038/s41586-021-03819-2" class="w-full bg-[var(--bg-input)] text-[var(--text)] placeholder-[var(--text-muted)] px-4 py-3 text-[13px] rounded-[var(--radius)] border border-[var(--border)] outline-none focus:border-[var(--accent)] focus:shadow-[0_0_0_3px_var(--accent-light)] transition-all"/> `);
        if (loading) {
          $$renderer3.push("<!--[0-->");
          $$renderer3.push(`<div class="absolute right-3 top-1/2 -translate-y-1/2"><div class="w-4 h-4 border-2 border-[var(--accent)] border-t-transparent rounded-full animate-spin"></div></div>`);
        } else {
          $$renderer3.push("<!--[-1-->");
        }
        $$renderer3.push(`<!--]--></div> <p class="text-[11px] text-[var(--text-muted)] mt-2 text-center">Metadata fetched from Crossref — supports journal articles, books, conference papers</p></div></div>`);
      } else if (activeView === "online") {
        $$renderer3.push("<!--[2-->");
        $$renderer3.push(`<div class="p-4 border-b border-[var(--border-light)]"><div class="flex gap-2"><div class="flex-1">`);
        SearchBar($$renderer3, {
          placeholder: "Search Crossref for papers...",
          onsubmit: handleOnlineSearch,
          get value() {
            return onlineQuery;
          },
          set value($$value) {
            onlineQuery = $$value;
            $$settled = false;
          }
        });
        $$renderer3.push(`<!----></div> <button class="px-4 py-2 bg-[var(--accent)] text-white text-[12px] font-medium rounded-[var(--radius-sm)] hover:bg-[var(--accent-hover)] transition-colors disabled:opacity-50"${attr("disabled", loading || !onlineQuery.trim(), true)}>${escape_html(loading ? "Searching..." : "Search")}</button></div></div> <div class="flex-1 overflow-y-auto">`);
        if (onlineResults.length > 0) {
          $$renderer3.push("<!--[0-->");
          $$renderer3.push(`<!--[-->`);
          const each_array = ensure_array_like(onlineResults);
          for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
            let ref = each_array[$$index];
            $$renderer3.push(`<div class="group px-4 py-3 border-b border-[var(--border-light)] hover:bg-[var(--bg-hover)] transition-colors"><div class="flex items-start justify-between gap-3"><div class="min-w-0 flex-1"><h3 class="text-[var(--text)] font-medium text-[13px] leading-snug line-clamp-2">${escape_html(ref.title)}</h3> <p class="text-[var(--text-secondary)] text-[11px] mt-1 truncate">${escape_html(ref.authors)}</p> <div class="flex items-center gap-2 mt-1">`);
            if (ref.year) {
              $$renderer3.push("<!--[0-->");
              $$renderer3.push(`<span class="text-[10px] px-1.5 py-0.5 rounded bg-[var(--bg-active)] text-[var(--text-secondary)] font-medium">${escape_html(ref.year)}</span>`);
            } else {
              $$renderer3.push("<!--[-1-->");
            }
            $$renderer3.push(`<!--]--> `);
            if (ref.journal) {
              $$renderer3.push("<!--[0-->");
              $$renderer3.push(`<span class="text-[10px] text-[var(--text-muted)] italic truncate">${escape_html(ref.journal)}</span>`);
            } else {
              $$renderer3.push("<!--[-1-->");
            }
            $$renderer3.push(`<!--]--></div></div> <button class="shrink-0 px-3 py-1.5 text-[11px] font-medium rounded-[var(--radius-sm)] bg-[var(--accent)] text-white hover:bg-[var(--accent-hover)] opacity-0 group-hover:opacity-100 transition-all">+ Add</button></div></div>`);
          }
          $$renderer3.push(`<!--]-->`);
        } else if (!loading) {
          $$renderer3.push("<!--[1-->");
          $$renderer3.push(`<div class="flex flex-col items-center justify-center h-full text-[var(--text-muted)]"><svg class="w-10 h-10 mb-3 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg> <p class="text-[13px]">Search Crossref to find papers</p> <p class="text-[11px] mt-1 opacity-60">Type a title, author, or topic and press Enter</p></div>`);
        } else {
          $$renderer3.push("<!--[-1-->");
        }
        $$renderer3.push(`<!--]--></div>`);
      } else if (activeView === "bibliography") {
        $$renderer3.push("<!--[3-->");
        $$renderer3.push(`<div class="flex-1 flex flex-col min-h-0"><div class="p-4 border-b border-[var(--border-light)]"><div class="flex items-center justify-between mb-3"><div><h2 class="text-sm font-semibold text-[var(--text)]">Bibliography Builder</h2> <p class="text-[11px] text-[var(--text-muted)] mt-0.5">Add references, reorder them, then export as a formatted bibliography. `);
        {
          $$renderer3.push("<!--[-1-->");
        }
        $$renderer3.push(`<!--]--></p></div> `);
        $$renderer3.select(
          {
            value: bibStyle,
            onchange: async () => {
              if (bibRefs.length) bibPreview = await formatBatchBibliography(bibRefs, bibStyle);
            },
            class: "text-[11px] px-2 py-1.5 border border-[var(--border)] rounded-[var(--radius-sm)] bg-[var(--bg-input)] text-[var(--text)] outline-none"
          },
          ($$renderer4) => {
            $$renderer4.option({ value: "APA" }, ($$renderer5) => {
              $$renderer5.push(`APA`);
            });
            $$renderer4.option({ value: "MLA" }, ($$renderer5) => {
              $$renderer5.push(`MLA`);
            });
            $$renderer4.option({ value: "Chicago" }, ($$renderer5) => {
              $$renderer5.push(`Chicago`);
            });
            $$renderer4.option({ value: "IEEE" }, ($$renderer5) => {
              $$renderer5.push(`IEEE`);
            });
            $$renderer4.option({ value: "Harvard" }, ($$renderer5) => {
              $$renderer5.push(`Harvard`);
            });
            $$renderer4.option({ value: "BibTeX" }, ($$renderer5) => {
              $$renderer5.push(`BibTeX`);
            });
          }
        );
        $$renderer3.push(`</div> <div class="flex gap-2"><div class="flex-1">`);
        SearchBar($$renderer3, {
          placeholder: "Search library to add references...",
          onsubmit: async () => {
            bibSearchResults = await searchReferences(bibSearch);
          },
          get value() {
            return bibSearch;
          },
          set value($$value) {
            bibSearch = $$value;
            $$settled = false;
          }
        });
        $$renderer3.push(`<!----></div></div></div> <div class="flex-1 flex min-h-0"><div class="w-1/2 border-r border-[var(--border-light)] overflow-y-auto">`);
        if (bibSearchResults.length > 0) {
          $$renderer3.push("<!--[0-->");
          $$renderer3.push(`<!--[-->`);
          const each_array_1 = ensure_array_like(bibSearchResults);
          for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
            let ref = each_array_1[$$index_1];
            const alreadyAdded = bibRefs.some((r) => r.id === ref.id);
            $$renderer3.push(`<div class="px-4 py-2.5 border-b border-[var(--border-light)] hover:bg-[var(--bg-hover)] transition-colors flex items-center gap-3"><div class="min-w-0 flex-1"><p class="text-[12px] text-[var(--text)] font-medium truncate">${escape_html(ref.title)}</p> <p class="text-[10px] text-[var(--text-muted)] truncate">${escape_html(ref.authors)}${escape_html(ref.year ? ` (${ref.year})` : "")}</p></div> <button${attr("disabled", alreadyAdded, true)}${attr_class(`shrink-0 px-2.5 py-1 text-[10px] font-medium rounded-[var(--radius-sm)] transition-colors ${stringify(alreadyAdded ? "bg-[var(--bg-active)] text-[var(--text-muted)]" : "bg-[var(--accent)] text-white hover:bg-[var(--accent-hover)]")}`)}>${escape_html(alreadyAdded ? "Added" : "+ Add")}</button></div>`);
          }
          $$renderer3.push(`<!--]-->`);
        } else {
          $$renderer3.push("<!--[-1-->");
          $$renderer3.push(`<div class="flex flex-col items-center justify-center h-full text-[var(--text-muted)] px-4"><p class="text-[12px]">Search your library to add references</p> <p class="text-[10px] mt-1 opacity-60">They'll appear in the bibliography on the right</p></div>`);
        }
        $$renderer3.push(`<!--]--></div> <div class="w-1/2 flex flex-col"><div class="px-4 py-2 border-b border-[var(--border-light)] flex items-center justify-between"><p class="text-[11px] font-semibold text-[var(--text-secondary)]">${escape_html(bibRefs.length)} ref${escape_html(bibRefs.length !== 1 ? "s" : "")}</p> <div class="flex gap-1.5"><button class="px-2.5 py-1 text-[10px] font-medium rounded-[var(--radius-sm)] bg-[var(--bg-active)] text-[var(--text-secondary)] hover:bg-[var(--accent-light)] hover:text-[var(--accent)] transition-colors"${attr("disabled", !bibRefs.length, true)}>Copy RTF</button> <button class="px-2.5 py-1 text-[10px] font-medium rounded-[var(--radius-sm)] bg-[var(--accent)] text-white hover:bg-[var(--accent-hover)] transition-colors"${attr("disabled", !bibRefs.length, true)}>Insert into Word</button></div></div> <div class="flex-1 overflow-y-auto">`);
        if (bibRefs.length > 0) {
          $$renderer3.push("<!--[0-->");
          $$renderer3.push(`<!--[-->`);
          const each_array_2 = ensure_array_like(bibRefs);
          for (let i = 0, $$length = each_array_2.length; i < $$length; i++) {
            let ref = each_array_2[i];
            $$renderer3.push(`<div class="flex items-center gap-2 px-4 py-2 border-b border-[var(--border-light)] group hover:bg-[var(--bg-hover)]"><span class="text-[11px] font-mono text-[var(--accent)] w-6 shrink-0">`);
            {
              $$renderer3.push("<!--[-1-->");
              $$renderer3.push(`${escape_html(i + 1)}.`);
            }
            $$renderer3.push(`<!--]--></span> <div class="min-w-0 flex-1"><p class="text-[11px] text-[var(--text)] truncate">${escape_html(ref.title)}</p> <p class="text-[9px] text-[var(--text-muted)] truncate">${escape_html(ref.authors)}</p></div> <div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">`);
            if (i > 0) {
              $$renderer3.push("<!--[0-->");
              $$renderer3.push(`<button class="p-0.5 rounded hover:bg-[var(--bg-active)]" title="Move up"><svg class="w-3 h-3 text-[var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-width="2" d="M5 15l7-7 7 7"></path></svg></button>`);
            } else {
              $$renderer3.push("<!--[-1-->");
            }
            $$renderer3.push(`<!--]--> `);
            if (i < bibRefs.length - 1) {
              $$renderer3.push("<!--[0-->");
              $$renderer3.push(`<button class="p-0.5 rounded hover:bg-[var(--bg-active)]" title="Move down"><svg class="w-3 h-3 text-[var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg></button>`);
            } else {
              $$renderer3.push("<!--[-1-->");
            }
            $$renderer3.push(`<!--]--> <button class="p-0.5 rounded hover:bg-[var(--danger-light)]" title="Remove"><svg class="w-3 h-3 text-[var(--text-muted)] hover:text-[var(--danger)]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg></button></div></div>`);
          }
          $$renderer3.push(`<!--]--> `);
          if (bibPreview) {
            $$renderer3.push("<!--[0-->");
            $$renderer3.push(`<div class="px-4 py-3 border-t border-[var(--border)]"><p class="text-[9px] font-semibold text-[var(--text-muted)] uppercase tracking-wider mb-2">Preview</p> <pre class="text-[11px] text-[var(--text-secondary)] whitespace-pre-wrap font-[inherit] leading-relaxed">${escape_html(bibPreview)}</pre></div>`);
          } else {
            $$renderer3.push("<!--[-1-->");
          }
          $$renderer3.push(`<!--]-->`);
        } else {
          $$renderer3.push("<!--[-1-->");
          $$renderer3.push(`<div class="flex flex-col items-center justify-center h-full text-[var(--text-muted)]"><p class="text-[12px]">No references added yet</p> <p class="text-[10px] mt-1 opacity-60">Search and add from the left panel</p></div>`);
        }
        $$renderer3.push(`<!--]--></div></div></div></div>`);
      } else {
        $$renderer3.push("<!--[-1-->");
        $$renderer3.push(`<div class="p-4 border-b border-[var(--border-light)]"><div class="flex items-center justify-between mb-3"><div><h2 class="text-sm font-semibold text-[var(--text)]">`);
        if (activeView === "folder") {
          $$renderer3.push("<!--[0-->");
          $$renderer3.push(`${escape_html(folders.find((f) => f.id === activeFolderId)?.name ?? "Collection")}`);
        } else {
          $$renderer3.push("<!--[-1-->");
          $$renderer3.push(`All References`);
        }
        $$renderer3.push(`<!--]--></h2> <p class="text-[11px] text-[var(--text-muted)] mt-0.5">${escape_html(results.length)} reference${escape_html(results.length !== 1 ? "s" : "")}</p></div></div> `);
        SearchBar($$renderer3, {
          placeholder: activeView === "folder" ? "Search this collection..." : "Search all references...",
          get value() {
            return query;
          },
          set value($$value) {
            query = $$value;
            $$settled = false;
          }
        });
        $$renderer3.push(`<!----></div> <div class="flex-1 overflow-y-auto">`);
        if (results.length > 0) {
          $$renderer3.push("<!--[0-->");
          $$renderer3.push(`<!--[-->`);
          const each_array_3 = ensure_array_like(results);
          for (let i = 0, $$length = each_array_3.length; i < $$length; i++) {
            let ref = each_array_3[i];
            CitationItem($$renderer3, {
              reference: ref,
              selected: i === selectedIndex
            });
          }
          $$renderer3.push(`<!--]-->`);
        } else {
          $$renderer3.push("<!--[-1-->");
          $$renderer3.push(`<div class="flex flex-col items-center justify-center h-full text-[var(--text-muted)]"><svg class="w-12 h-12 mb-3 opacity-20" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1"><path stroke-linecap="round" stroke-linejoin="round" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path></svg> `);
          if (query) {
            $$renderer3.push("<!--[0-->");
            $$renderer3.push(`<p class="text-[13px]">No matching references</p> <p class="text-[11px] mt-1 opacity-60">Try different keywords</p>`);
          } else {
            $$renderer3.push("<!--[-1-->");
            $$renderer3.push(`<p class="text-[13px]">No references yet</p> <p class="text-[11px] mt-1 opacity-60">Add references using "Add by DOI" or "Search Online"</p>`);
          }
          $$renderer3.push(`<!--]--></div>`);
        }
        $$renderer3.push(`<!--]--></div>`);
      }
      $$renderer3.push(`<!--]--></div></div> `);
      {
        $$renderer3.push("<!--[-1-->");
      }
      $$renderer3.push(`<!--]--> `);
      {
        $$renderer3.push("<!--[-1-->");
      }
      $$renderer3.push(`<!--]--></div>`);
    }
    do {
      $$settled = true;
      $$inner_renderer = $$renderer2.copy();
      $$render_inner($$inner_renderer);
    } while (!$$settled);
    $$renderer2.subsume($$inner_renderer);
  });
}
export {
  _page as default
};
