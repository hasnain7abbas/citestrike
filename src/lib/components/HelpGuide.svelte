<script lang="ts">
	let openSection = $state<string | null>('getting-started');

	function toggle(id: string) {
		openSection = openSection === id ? null : id;
	}

	const sections = [
		{
			id: 'getting-started',
			title: 'Getting Started',
			icon: '🚀',
			content: [
				{ q: 'What is CiteStrike?', a: 'CiteStrike is a fast, local-first citation manager. Your references live in a local SQLite database on your machine — no cloud accounts, no subscriptions, no data leaving your computer.' },
				{ q: 'How do I add my first reference?', a: 'You have three options:\n\n1. **Add by DOI** — Click "Add by DOI" in the sidebar, paste a DOI (e.g. 10.1038/s41586-021-03819-2), and press Enter. CiteStrike fetches all metadata from Crossref automatically.\n\n2. **Search Online** — Click "Search Online", type a paper title or author name, and click "Add" on any result.\n\n3. **Import PDF** — Drag and drop a PDF file. CiteStrike will extract the DOI from the document and auto-fetch the metadata.' },
				{ q: 'Where is my data stored?', a: 'All references are stored in a local SQLite database file (citestrike.db) in your system\'s app data directory. Nothing is sent to external servers except DOI lookups to the free Crossref API.' },
			]
		},
		{
			id: 'library',
			title: 'Managing Your Library',
			icon: '📚',
			content: [
				{ q: 'How do I organize references?', a: 'Use **Collections** (folders) in the sidebar. Click the + button next to "Collections" to create a new folder. You can move references between folders using the ⋮ menu on each reference card.' },
				{ q: 'How do I delete a reference?', a: 'Hover over any reference card to reveal action buttons. Click the ⋮ (three dots) menu and select "Delete reference". This permanently removes it from your library.' },
				{ q: 'How do I search my library?', a: 'Use the search bar at the top. It searches across titles, authors, DOIs, and BibTeX keys. When viewing a specific collection, search is scoped to that collection only.' },
				{ q: 'How do I rename or delete a collection?', a: 'Hover over any collection in the sidebar to reveal edit (pencil) and delete (trash) icons. Deleting a collection moves its references back to "All References" — references are never deleted when you remove a collection.' },
				{ q: 'Can I move references between collections?', a: 'Yes. Click the ⋮ menu on any reference and select "Move to folder...". You can also move references to no folder (All References).' },
			]
		},
		{
			id: 'citations',
			title: 'Copying Citations',
			icon: '📋',
			content: [
				{ q: 'How do I copy a formatted citation?', a: 'Hover over any reference to see format buttons: **APA**, **MLA**, and **BIB**. Click one to instantly copy the formatted citation to your clipboard. For more formats (Chicago, IEEE, Harvard), click the ⋮ menu.' },
				{ q: 'What citation styles are supported?', a: '**Six styles** are built in:\n\n• **APA** (7th edition) — Psychology, social sciences\n• **MLA** (9th edition) — Humanities, literature\n• **Chicago** — History, publishing\n• **IEEE** — Engineering, computer science\n• **Harvard** — General academic\n• **BibTeX** — LaTeX documents' },
				{ q: 'How do I paste into my document?', a: 'After clicking a format button, the citation is on your clipboard. Simply Ctrl+V (or Cmd+V) into Word, Google Docs, Overleaf, Obsidian, Notion, or any other editor.' },
			]
		},
		{
			id: 'shortcuts',
			title: 'Keyboard Shortcuts',
			icon: '⌨️',
			content: [
				{ q: 'Available shortcuts', a: '• **Ctrl+Shift+C** — Toggle CiteStrike window (global, works from any app)\n• **Enter** — Submit DOI / Confirm search\n• **Escape** — Close menus\n• **Arrow Up/Down** — Navigate search results' },
			]
		},
		{
			id: 'online-search',
			title: 'Searching Online',
			icon: '🌐',
			content: [
				{ q: 'Where does online search look?', a: 'CiteStrike searches the **Crossref** database, which indexes over 130 million scholarly works from thousands of publishers worldwide.' },
				{ q: 'Why can\'t I find a paper?', a: 'Some papers may not be indexed in Crossref (e.g., preprints, very recent publications, or papers from certain publishers). Try searching by title keywords instead of full title. You can also add papers manually by DOI.' },
				{ q: 'Is there a rate limit?', a: 'Crossref\'s public API is free and generous, but please avoid rapid-fire searches. CiteStrike makes polite requests with a proper User-Agent header.' },
			]
		},
		{
			id: 'data',
			title: 'Data & Privacy',
			icon: '🔒',
			content: [
				{ q: 'Is my data private?', a: 'Yes. CiteStrike stores everything locally on your machine. The only outbound network requests are to the Crossref API when you explicitly search online or fetch a DOI.' },
				{ q: 'Can I export my library?', a: 'You can copy individual references in BibTeX format using the BIB button. Full library export to .bib files is planned for a future update.' },
				{ q: 'Can I back up my library?', a: 'Your database file (citestrike.db) is in your app data directory. You can copy this file to back up your entire library.' },
			]
		},
	];
</script>

<div class="h-full overflow-y-auto">
	<div class="max-w-2xl mx-auto px-6 py-6">
		<!-- Header -->
		<div class="mb-8">
			<h1 class="text-xl font-bold text-[var(--text)] mb-1">Help Guide</h1>
			<p class="text-[var(--text-secondary)] text-[13px]">Everything you need to know about using CiteStrike</p>
		</div>

		<!-- Quick start banner -->
		<div class="bg-[var(--accent-light)] border border-[var(--accent)]/20 rounded-[var(--radius)] p-4 mb-6">
			<div class="flex gap-3">
				<span class="text-lg">⚡</span>
				<div>
					<p class="font-semibold text-[var(--accent)] text-[13px] mb-1">Quick Start</p>
					<p class="text-[12px] text-[var(--text-secondary)] leading-relaxed">
						Click <strong>"Add by DOI"</strong> in the sidebar, paste any DOI, and press Enter. Your first reference will appear in the library in seconds.
						Or click <strong>"Search Online"</strong> to find papers by title or author.
					</p>
				</div>
			</div>
		</div>

		<!-- Sections -->
		<div class="space-y-2">
			{#each sections as section}
				<div class="border border-[var(--border-light)] rounded-[var(--radius-sm)] overflow-hidden">
					<button
						onclick={() => toggle(section.id)}
						class="w-full flex items-center gap-3 px-4 py-3 text-left hover:bg-[var(--bg-hover)] transition-colors"
					>
						<span class="text-base">{section.icon}</span>
						<span class="flex-1 font-medium text-[13px] text-[var(--text)]">{section.title}</span>
						<svg class="w-4 h-4 text-[var(--text-muted)] transition-transform {openSection === section.id ? 'rotate-180' : ''}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
						</svg>
					</button>

					{#if openSection === section.id}
						<div class="border-t border-[var(--border-light)] bg-[var(--bg-card)]">
							{#each section.content as item, i}
								<div class="px-4 py-3 {i > 0 ? 'border-t border-[var(--border-light)]' : ''}">
									<p class="font-medium text-[12px] text-[var(--text)] mb-1.5">{item.q}</p>
									<div class="text-[12px] text-[var(--text-secondary)] leading-relaxed whitespace-pre-line">
										{@html item.a
											.replace(/\*\*(.*?)\*\*/g, '<strong style="color:var(--text)">$1</strong>')
											.replace(/\n/g, '<br/>')
											.replace(/• /g, '<span style="display:inline-block;width:16px">•</span>')}
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/each}
		</div>

		<!-- Footer -->
		<div class="mt-8 text-center text-[11px] text-[var(--text-muted)]">
			<p>CiteStrike v0.1.0 — Built with Rust, Tauri & Svelte</p>
			<p class="mt-1">Report issues at github.com/yourusername/citestrike</p>
		</div>
	</div>
</div>
