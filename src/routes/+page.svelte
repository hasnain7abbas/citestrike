<script lang="ts">
	import TitleBar from '$lib/components/TitleBar.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import CitationItem from '$lib/components/CitationItem.svelte';
	import HelpGuide from '$lib/components/HelpGuide.svelte';
	import {
		searchReferences, fetchDoi, addReference, searchOnline, moveReference, getFolders,
		copyRichBibliography, insertBibliographyIntoWord, formatBatchBibliography,
		importPdf, copyCitedBibliography, getCitedReferences, resetCitations, writeBibtexFile,
		type CitationStyle
	} from '$lib/tauri';
	import type { Reference, NewReference, Folder } from '$lib/tauri';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { save } from '@tauri-apps/plugin-dialog';
	import { onMount } from 'svelte';

	// State
	let query = $state('');
	let results = $state<Reference[]>([]);
	let selectedIndex = $state(0);
	let loading = $state(false);
	let activeView = $state('all');
	let activeFolderId = $state<string | null>(null);
	let folders = $state<Folder[]>([]);
	let doiInput = $state('');
	let statusMessage = $state('');
	let statusType = $state<'success' | 'error'>('success');
	let onlineQuery = $state('');
	let onlineResults = $state<NewReference[]>([]);
	let showMoveDialog = $state(false);
	let moveRefId = $state<string | null>(null);
	// Bibliography builder
	let bibRefs = $state<Reference[]>([]);
	let bibStyle = $state<CitationStyle>('APA');
	let bibSearch = $state('');
	let bibSearchResults = $state<Reference[]>([]);
	let bibPreview = $state('');
	// Global citation style
	let citationStyle = $state<CitationStyle>('APA');
	// Drag and drop
	let dragOver = $state(false);
	let importing = $state(false);
	// Manual entry
	let manualTitle = $state('');
	let manualAuthors = $state('');
	let manualYear = $state('');
	let manualJournal = $state('');
	let manualDoi = $state('');
	let manualVolume = $state('');
	let manualIssue = $state('');
	let manualPages = $state('');
	let manualType = $state('article');
	// Cited count
	let citedCount = $state(0);

	let debounceTimer: ReturnType<typeof setTimeout>;

	// Load refs when view or query changes
	$effect(() => {
		if (activeView === 'all' || activeView === 'folder') {
			clearTimeout(debounceTimer);
			debounceTimer = setTimeout(async () => {
				try {
					results = await searchReferences(query, activeView === 'folder' ? activeFolderId : null);
					selectedIndex = 0;
				} catch { results = []; }
			}, 120);
		}
	});

	// Reload on view switch
	$effect(() => {
		if (activeView === 'all' || activeView === 'folder') {
			searchReferences('', activeView === 'folder' ? activeFolderId : null)
				.then(r => { results = r; query = ''; })
				.catch(() => { results = []; });
		}
	});

	// Track cited count
	$effect(() => {
		if (activeView === 'all' || activeView === 'folder') {
			getCitedReferences().then(cited => { citedCount = cited.length; }).catch(() => {});
		}
	});

	function showStatus(msg: string, type: 'success' | 'error' = 'success') {
		statusMessage = msg;
		statusType = type;
		setTimeout(() => statusMessage = '', 3000);
	}

	async function handleAddDoi() {
		if (!doiInput.trim()) return;
		loading = true;
		try {
			const ref = await fetchDoi(doiInput.trim());
			await addReference(ref, activeFolderId);
			showStatus('Reference added to library');
			doiInput = '';
			activeView = 'all';
		} catch (e) {
			showStatus(`Failed: ${e}`, 'error');
		}
		loading = false;
	}

	async function handleOnlineSearch() {
		if (!onlineQuery.trim()) return;
		loading = true;
		try { onlineResults = await searchOnline(onlineQuery); }
		catch { onlineResults = []; }
		loading = false;
	}

	async function addFromOnline(ref: NewReference) {
		try {
			await addReference(ref, activeFolderId);
			showStatus('Added to library');
		} catch (e) {
			showStatus(`Error: ${e}`, 'error');
		}
	}

	function handleRefDelete(id: string) {
		results = results.filter(r => r.id !== id);
		showStatus('Reference deleted');
	}

	function openMoveDialog(refId: string) {
		moveRefId = refId;
		showMoveDialog = true;
		getFolders().then(f => folders = f);
	}

	async function handleMove(folderId: string | null) {
		if (moveRefId) {
			await moveReference(moveRefId, folderId);
			showMoveDialog = false;
			moveRefId = null;
			results = await searchReferences(query, activeView === 'folder' ? activeFolderId : null);
			showStatus('Reference moved');
		}
	}

	async function refreshResults() {
		try {
			results = await searchReferences(query, activeView === 'folder' ? activeFolderId : null);
			const cited = await getCitedReferences();
			citedCount = cited.length;
		} catch { /* */ }
	}

	// --- Drag and Drop ---
	onMount(() => {
		let unlisten: (() => void) | undefined;
		getCurrentWindow().onDragDropEvent((event) => {
			if (event.payload.type === 'over' || event.payload.type === 'enter') {
				dragOver = true;
			} else if (event.payload.type === 'drop') {
				dragOver = false;
				handleFileDrop(event.payload.paths);
			} else {
				dragOver = false;
			}
		}).then(fn => { unlisten = fn; });

		return () => { unlisten?.(); };
	});

	async function handleFileDrop(paths: string[]) {
		const pdfPaths = paths.filter(p => p.toLowerCase().endsWith('.pdf'));
		if (pdfPaths.length === 0) {
			showStatus('Only PDF files are accepted', 'error');
			return;
		}

		importing = true;
		let successCount = 0;
		let noDoi = 0;

		for (const path of pdfPaths) {
			try {
				const doi = await importPdf(path);
				const ref = await fetchDoi(doi);
				await addReference(ref, activeFolderId);
				successCount++;
			} catch (e) {
				const err = String(e);
				if (err.includes('No DOI')) {
					noDoi++;
				} else {
					showStatus(`Import error: ${err}`, 'error');
				}
			}
		}

		importing = false;

		if (successCount > 0) {
			showStatus(`Imported ${successCount} reference${successCount > 1 ? 's' : ''}`);
			activeView = 'all';
			await refreshResults();
		}
		if (noDoi > 0) {
			showStatus(`${noDoi} PDF(s) had no DOI — use "Add Manually"`, 'error');
			activeView = 'add-manual';
		}
	}

	// --- Manual Entry ---
	async function handleManualAdd() {
		if (!manualTitle.trim() || !manualAuthors.trim()) {
			showStatus('Title and Authors are required', 'error');
			return;
		}
		loading = true;
		try {
			const newRef: NewReference = {
				title: manualTitle.trim(),
				authors: manualAuthors.trim(),
				year: manualYear ? parseInt(manualYear) : null,
				doi: manualDoi.trim() || null,
				journal: manualJournal.trim() || null,
				volume: manualVolume.trim() || null,
				issue: manualIssue.trim() || null,
				pages: manualPages.trim() || null,
				abstract_text: null,
				url: null,
				ref_type: manualType,
			};
			await addReference(newRef, activeFolderId);
			showStatus('Reference added to library');
			manualTitle = ''; manualAuthors = ''; manualYear = ''; manualJournal = '';
			manualDoi = ''; manualVolume = ''; manualIssue = ''; manualPages = '';
			activeView = 'all';
		} catch (e) {
			showStatus(`Failed: ${e}`, 'error');
		}
		loading = false;
	}

	// --- Export BibTeX ---
	async function handleExportBibtex() {
		try {
			const path = await save({
				filters: [{ name: 'BibTeX', extensions: ['bib'] }],
				defaultPath: 'references.bib',
			});
			if (path) {
				await writeBibtexFile(path);
				showStatus('Exported BibTeX file');
			}
		} catch (e) {
			showStatus(`Export failed: ${e}`, 'error');
		}
	}

	// --- Copy Bibliography of cited papers ---
	async function handleCopyBibliography() {
		try {
			await copyCitedBibliography(citationStyle);
			showStatus('Bibliography copied to clipboard');
		} catch (e) {
			showStatus(`${e}`, 'error');
		}
	}

	// --- Reset citations ---
	async function handleResetCitations() {
		try {
			await resetCitations();
			citedCount = 0;
			await refreshResults();
			showStatus('All citations reset');
		} catch (e) {
			showStatus(`${e}`, 'error');
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (activeView === 'all' || activeView === 'folder') {
			if (e.key === 'ArrowDown') { e.preventDefault(); selectedIndex = Math.min(selectedIndex + 1, results.length - 1); }
			if (e.key === 'ArrowUp') { e.preventDefault(); selectedIndex = Math.max(selectedIndex - 1, 0); }
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="h-screen w-screen flex flex-col bg-[var(--bg)] overflow-hidden">
	<TitleBar />

	<div class="flex flex-1 min-h-0">
		<Sidebar bind:folders bind:activeView bind:activeFolderId />

		<!-- Main content -->
		<div class="flex-1 flex flex-col min-w-0">

			{#if activeView === 'help'}
				<HelpGuide />

			{:else if activeView === 'add-doi'}
				<div class="flex-1 flex flex-col items-center justify-center px-6">
					<div class="w-full max-w-md">
						<div class="text-center mb-6">
							<div class="w-12 h-12 mx-auto mb-3 rounded-[var(--radius)] bg-[var(--accent-light)] flex items-center justify-center">
								<svg class="w-6 h-6 text-[var(--accent)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8">
									<path stroke-linecap="round" stroke-linejoin="round" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"/>
								</svg>
							</div>
							<h2 class="text-base font-semibold text-[var(--text)]">Add Reference by DOI</h2>
							<p class="text-[12px] text-[var(--text-secondary)] mt-1">Paste a DOI and press Enter to auto-fetch metadata</p>
						</div>
						<div class="relative">
							<input
								bind:value={doiInput}
								placeholder="e.g. 10.1038/s41586-021-03819-2"
								onkeydown={(e) => e.key === 'Enter' && handleAddDoi()}
								class="w-full bg-[var(--bg-input)] text-[var(--text)] placeholder-[var(--text-muted)]
								       px-4 py-3 text-[13px] rounded-[var(--radius)] border border-[var(--border)]
								       outline-none focus:border-[var(--accent)] focus:shadow-[0_0_0_3px_var(--accent-light)] transition-all"
							/>
							{#if loading}
								<div class="absolute right-3 top-1/2 -translate-y-1/2">
									<div class="w-4 h-4 border-2 border-[var(--accent)] border-t-transparent rounded-full animate-spin"></div>
								</div>
							{/if}
						</div>
						<p class="text-[11px] text-[var(--text-muted)] mt-2 text-center">
							Metadata fetched from Crossref — supports journal articles, books, conference papers
						</p>
					</div>
				</div>

			{:else if activeView === 'add-manual'}
				<!-- Manual Entry Form -->
				<div class="flex-1 flex flex-col items-center justify-center px-6 overflow-y-auto">
					<div class="w-full max-w-lg py-8">
						<div class="text-center mb-6">
							<div class="w-12 h-12 mx-auto mb-3 rounded-[var(--radius)] bg-[var(--accent-light)] flex items-center justify-center">
								<svg class="w-6 h-6 text-[var(--accent)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8">
									<path stroke-linecap="round" stroke-linejoin="round" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
								</svg>
							</div>
							<h2 class="text-base font-semibold text-[var(--text)]">Add Reference Manually</h2>
							<p class="text-[12px] text-[var(--text-secondary)] mt-1">Enter the paper details directly</p>
						</div>
						<div class="space-y-3">
							<div>
								<label class="block text-[11px] font-medium text-[var(--text-secondary)] mb-1">Title *</label>
								<input bind:value={manualTitle} placeholder="Paper title"
									class="w-full px-3 py-2.5 text-[13px] bg-[var(--bg-input)] text-[var(--text)] placeholder-[var(--text-muted)]
									       border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)] focus:shadow-[0_0_0_3px_var(--accent-light)] transition-all" />
							</div>
							<div>
								<label class="block text-[11px] font-medium text-[var(--text-secondary)] mb-1">Authors *</label>
								<input bind:value={manualAuthors} placeholder="Smith, John; Doe, Jane"
									class="w-full px-3 py-2.5 text-[13px] bg-[var(--bg-input)] text-[var(--text)] placeholder-[var(--text-muted)]
									       border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)] focus:shadow-[0_0_0_3px_var(--accent-light)] transition-all" />
								<p class="text-[10px] text-[var(--text-muted)] mt-1">Format: LastName, FirstName; LastName2, FirstName2</p>
							</div>
							<div class="grid grid-cols-3 gap-3">
								<div>
									<label class="block text-[11px] font-medium text-[var(--text-secondary)] mb-1">Year</label>
									<input bind:value={manualYear} type="number" placeholder="2024"
										class="w-full px-3 py-2.5 text-[13px] bg-[var(--bg-input)] text-[var(--text)] placeholder-[var(--text-muted)]
										       border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)] transition-all" />
								</div>
								<div class="col-span-2">
									<label class="block text-[11px] font-medium text-[var(--text-secondary)] mb-1">Journal</label>
									<input bind:value={manualJournal} placeholder="Journal name"
										class="w-full px-3 py-2.5 text-[13px] bg-[var(--bg-input)] text-[var(--text)] placeholder-[var(--text-muted)]
										       border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)] transition-all" />
								</div>
							</div>
							<div>
								<label class="block text-[11px] font-medium text-[var(--text-secondary)] mb-1">DOI</label>
								<input bind:value={manualDoi} placeholder="10.xxxx/xxxxx"
									class="w-full px-3 py-2.5 text-[13px] bg-[var(--bg-input)] text-[var(--text)] placeholder-[var(--text-muted)]
									       border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)] transition-all" />
							</div>
							<div class="grid grid-cols-4 gap-3">
								<div>
									<label class="block text-[11px] font-medium text-[var(--text-secondary)] mb-1">Volume</label>
									<input bind:value={manualVolume} placeholder="12"
										class="w-full px-3 py-2.5 text-[13px] bg-[var(--bg-input)] text-[var(--text)] placeholder-[var(--text-muted)]
										       border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)] transition-all" />
								</div>
								<div>
									<label class="block text-[11px] font-medium text-[var(--text-secondary)] mb-1">Issue</label>
									<input bind:value={manualIssue} placeholder="3"
										class="w-full px-3 py-2.5 text-[13px] bg-[var(--bg-input)] text-[var(--text)] placeholder-[var(--text-muted)]
										       border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)] transition-all" />
								</div>
								<div>
									<label class="block text-[11px] font-medium text-[var(--text-secondary)] mb-1">Pages</label>
									<input bind:value={manualPages} placeholder="45-67"
										class="w-full px-3 py-2.5 text-[13px] bg-[var(--bg-input)] text-[var(--text)] placeholder-[var(--text-muted)]
										       border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)] transition-all" />
								</div>
								<div>
									<label class="block text-[11px] font-medium text-[var(--text-secondary)] mb-1">Type</label>
									<select bind:value={manualType}
										class="w-full px-2 py-2.5 text-[13px] bg-[var(--bg-input)] text-[var(--text)]
										       border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)] transition-all">
										<option value="article">Article</option>
										<option value="book">Book</option>
										<option value="proceedings-article">Conference</option>
										<option value="book-chapter">Chapter</option>
										<option value="dissertation">Thesis</option>
									</select>
								</div>
							</div>
							<button onclick={handleManualAdd}
								disabled={loading || !manualTitle.trim() || !manualAuthors.trim()}
								class="w-full mt-2 px-4 py-2.5 bg-[var(--accent)] text-white text-[13px] font-medium rounded-[var(--radius-sm)]
								       hover:bg-[var(--accent-hover)] transition-colors disabled:opacity-50">
								{loading ? 'Adding...' : 'Add to Library'}
							</button>
						</div>
					</div>
				</div>

			{:else if activeView === 'online'}
				<!-- Online search -->
				<div class="p-4 border-b border-[var(--border-light)]">
					<div class="flex gap-2">
						<div class="flex-1">
							<SearchBar bind:value={onlineQuery} placeholder="Search Crossref for papers..." onsubmit={handleOnlineSearch} />
						</div>
						<button onclick={handleOnlineSearch}
							class="px-4 py-2 bg-[var(--accent)] text-white text-[12px] font-medium rounded-[var(--radius-sm)]
							       hover:bg-[var(--accent-hover)] transition-colors disabled:opacity-50"
							disabled={loading || !onlineQuery.trim()}>
							{loading ? 'Searching...' : 'Search'}
						</button>
					</div>
				</div>
				<div class="flex-1 overflow-y-auto">
					{#if onlineResults.length > 0}
						{#each onlineResults as ref}
							<div class="group px-4 py-3 border-b border-[var(--border-light)] hover:bg-[var(--bg-hover)] transition-colors">
								<div class="flex items-start justify-between gap-3">
									<div class="min-w-0 flex-1">
										<h3 class="text-[var(--text)] font-medium text-[13px] leading-snug line-clamp-2">{ref.title}</h3>
										<p class="text-[var(--text-secondary)] text-[11px] mt-1 truncate">{ref.authors}</p>
										<div class="flex items-center gap-2 mt-1">
											{#if ref.year}
												<span class="text-[10px] px-1.5 py-0.5 rounded bg-[var(--bg-active)] text-[var(--text-secondary)] font-medium">{ref.year}</span>
											{/if}
											{#if ref.journal}
												<span class="text-[10px] text-[var(--text-muted)] italic truncate">{ref.journal}</span>
											{/if}
										</div>
									</div>
									<button onclick={() => addFromOnline(ref)}
										class="shrink-0 px-3 py-1.5 text-[11px] font-medium rounded-[var(--radius-sm)]
										       bg-[var(--accent)] text-white hover:bg-[var(--accent-hover)]
										       opacity-0 group-hover:opacity-100 transition-all">
										+ Add
									</button>
								</div>
							</div>
						{/each}
					{:else if !loading}
						<div class="flex flex-col items-center justify-center h-full text-[var(--text-muted)]">
							<svg class="w-10 h-10 mb-3 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
							</svg>
							<p class="text-[13px]">Search Crossref to find papers</p>
							<p class="text-[11px] mt-1 opacity-60">Type a title, author, or topic and press Enter</p>
						</div>
					{/if}
				</div>

			{:else if activeView === 'bibliography'}
				<!-- Bibliography Builder -->
				<div class="flex-1 flex flex-col min-h-0">
					<div class="p-4 border-b border-[var(--border-light)]">
						<div class="flex items-center justify-between mb-3">
							<div>
								<h2 class="text-sm font-semibold text-[var(--text)]">Bibliography Builder</h2>
								<p class="text-[11px] text-[var(--text-muted)] mt-0.5">
									Add references, reorder them, then export as a formatted bibliography.
									{#if bibStyle === 'IEEE'} IEEE references are auto-numbered [1], [2], etc.{:else if bibStyle === 'Vancouver'} Vancouver references are auto-numbered 1., 2., etc.{/if}
								</p>
							</div>
							<select bind:value={bibStyle}
								onchange={async () => { if (bibRefs.length) bibPreview = await formatBatchBibliography(bibRefs, bibStyle); }}
								class="text-[11px] px-2 py-1.5 border border-[var(--border)] rounded-[var(--radius-sm)] bg-[var(--bg-input)] text-[var(--text)] outline-none">
								<option value="APA">APA</option>
								<option value="MLA">MLA</option>
								<option value="Chicago">Chicago</option>
								<option value="IEEE">IEEE</option>
								<option value="Harvard">Harvard</option>
								<option value="Vancouver">Vancouver</option>
								<option value="BibTeX">BibTeX</option>
							</select>
						</div>
						<div class="flex gap-2">
							<div class="flex-1">
								<SearchBar bind:value={bibSearch} placeholder="Search library to add references..." onsubmit={async () => {
									bibSearchResults = await searchReferences(bibSearch);
								}} />
							</div>
						</div>
					</div>
					<div class="flex-1 flex min-h-0">
						<!-- Left: search results -->
						<div class="w-1/2 border-r border-[var(--border-light)] overflow-y-auto">
							{#if bibSearchResults.length > 0}
								{#each bibSearchResults as ref}
									{@const alreadyAdded = bibRefs.some(r => r.id === ref.id)}
									<div class="px-4 py-2.5 border-b border-[var(--border-light)] hover:bg-[var(--bg-hover)] transition-colors flex items-center gap-3">
										<div class="min-w-0 flex-1">
											<p class="text-[12px] text-[var(--text)] font-medium truncate">{ref.title}</p>
											<p class="text-[10px] text-[var(--text-muted)] truncate">{ref.authors}{ref.year ? ` (${ref.year})` : ''}</p>
										</div>
										<button
											onclick={async () => { if (!alreadyAdded) { bibRefs = [...bibRefs, ref]; bibPreview = await formatBatchBibliography(bibRefs, bibStyle); } }}
											disabled={alreadyAdded}
											class="shrink-0 px-2.5 py-1 text-[10px] font-medium rounded-[var(--radius-sm)] transition-colors
											       {alreadyAdded ? 'bg-[var(--bg-active)] text-[var(--text-muted)]' : 'bg-[var(--accent)] text-white hover:bg-[var(--accent-hover)]'}">
											{alreadyAdded ? 'Added' : '+ Add'}
										</button>
									</div>
								{/each}
							{:else}
								<div class="flex flex-col items-center justify-center h-full text-[var(--text-muted)] px-4">
									<p class="text-[12px]">Search your library to add references</p>
									<p class="text-[10px] mt-1 opacity-60">They'll appear in the bibliography on the right</p>
								</div>
							{/if}
						</div>
						<!-- Right: ordered bibliography -->
						<div class="w-1/2 flex flex-col">
							<div class="px-4 py-2 border-b border-[var(--border-light)] flex items-center justify-between">
								<p class="text-[11px] font-semibold text-[var(--text-secondary)]">{bibRefs.length} ref{bibRefs.length !== 1 ? 's' : ''}</p>
								<div class="flex gap-1.5">
									<button onclick={async () => {
										if (bibRefs.length) { try { await copyRichBibliography(bibRefs, bibStyle); showStatus('Bibliography copied (rich text)'); } catch { showStatus('Copy failed', 'error'); } }
									}} class="px-2.5 py-1 text-[10px] font-medium rounded-[var(--radius-sm)] bg-[var(--bg-active)] text-[var(--text-secondary)] hover:bg-[var(--accent-light)] hover:text-[var(--accent)] transition-colors" disabled={!bibRefs.length}>
										Copy RTF
									</button>
									<button onclick={async () => {
										if (bibRefs.length) { try { const t = await insertBibliographyIntoWord(bibRefs, bibStyle); showStatus('Inserted into Word'); } catch (e) { showStatus(`${e}`, 'error'); } }
									}} class="px-2.5 py-1 text-[10px] font-medium rounded-[var(--radius-sm)] bg-[var(--accent)] text-white hover:bg-[var(--accent-hover)] transition-colors" disabled={!bibRefs.length}>
										Insert into Word
									</button>
								</div>
							</div>
							<div class="flex-1 overflow-y-auto">
								{#if bibRefs.length > 0}
									{#each bibRefs as ref, i}
										<div class="flex items-center gap-2 px-4 py-2 border-b border-[var(--border-light)] group hover:bg-[var(--bg-hover)]">
											<span class="text-[11px] font-mono text-[var(--accent)] w-6 shrink-0">
												{#if bibStyle === 'IEEE'}[{i+1}]{:else if bibStyle === 'Vancouver'}{i+1}.{:else}{i+1}.{/if}
											</span>
											<div class="min-w-0 flex-1">
												<p class="text-[11px] text-[var(--text)] truncate">{ref.title}</p>
												<p class="text-[9px] text-[var(--text-muted)] truncate">{ref.authors}</p>
											</div>
											<div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
												{#if i > 0}
													<button onclick={() => { const a=[...bibRefs]; [a[i-1],a[i]]=[a[i],a[i-1]]; bibRefs=a; formatBatchBibliography(bibRefs,bibStyle).then(p=>bibPreview=p); }} class="p-0.5 rounded hover:bg-[var(--bg-active)]" title="Move up">
														<svg class="w-3 h-3 text-[var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-width="2" d="M5 15l7-7 7 7"/></svg>
													</button>
												{/if}
												{#if i < bibRefs.length - 1}
													<button onclick={() => { const a=[...bibRefs]; [a[i],a[i+1]]=[a[i+1],a[i]]; bibRefs=a; formatBatchBibliography(bibRefs,bibStyle).then(p=>bibPreview=p); }} class="p-0.5 rounded hover:bg-[var(--bg-active)]" title="Move down">
														<svg class="w-3 h-3 text-[var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
													</button>
												{/if}
												<button onclick={() => { bibRefs=bibRefs.filter((_,idx)=>idx!==i); if(bibRefs.length) formatBatchBibliography(bibRefs,bibStyle).then(p=>bibPreview=p); else bibPreview=''; }} class="p-0.5 rounded hover:bg-[var(--danger-light)]" title="Remove">
													<svg class="w-3 h-3 text-[var(--text-muted)] hover:text-[var(--danger)]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/></svg>
												</button>
											</div>
										</div>
									{/each}
									{#if bibPreview}
										<div class="px-4 py-3 border-t border-[var(--border)]">
											<p class="text-[9px] font-semibold text-[var(--text-muted)] uppercase tracking-wider mb-2">Preview</p>
											<pre class="text-[11px] text-[var(--text-secondary)] whitespace-pre-wrap font-[inherit] leading-relaxed">{bibPreview}</pre>
										</div>
									{/if}
								{:else}
									<div class="flex flex-col items-center justify-center h-full text-[var(--text-muted)]">
										<p class="text-[12px]">No references added yet</p>
										<p class="text-[10px] mt-1 opacity-60">Search and add from the left panel</p>
									</div>
								{/if}
							</div>
						</div>
					</div>
				</div>

			{:else}
				<!-- Library view (all or folder) -->
				<div class="p-4 border-b border-[var(--border-light)]">
					<div class="flex items-center justify-between mb-3">
						<div>
							<h2 class="text-sm font-semibold text-[var(--text)]">
								{#if activeView === 'folder'}
									{folders.find(f => f.id === activeFolderId)?.name ?? 'Collection'}
								{:else}
									All References
								{/if}
							</h2>
							<p class="text-[11px] text-[var(--text-muted)] mt-0.5">
								{results.length} reference{results.length !== 1 ? 's' : ''}
								{#if citedCount > 0}
									<span class="ml-1 text-[var(--accent)]">({citedCount} cited)</span>
								{/if}
							</p>
						</div>
						<!-- Citation style + actions toolbar -->
						<div class="flex items-center gap-2">
							<select bind:value={citationStyle}
								class="text-[11px] px-2 py-1.5 border border-[var(--border)] rounded-[var(--radius-sm)] bg-[var(--bg-input)] text-[var(--text)] outline-none">
								<option value="APA">APA</option>
								<option value="MLA">MLA</option>
								<option value="Chicago">Chicago</option>
								<option value="IEEE">IEEE</option>
								<option value="Harvard">Harvard</option>
								<option value="Vancouver">Vancouver</option>
								<option value="BibTeX">BibTeX</option>
							</select>
							{#if citedCount > 0}
								<button onclick={handleCopyBibliography}
									class="px-2.5 py-1.5 text-[10px] font-medium rounded-[var(--radius-sm)]
									       bg-[var(--accent)] text-white hover:bg-[var(--accent-hover)] transition-colors"
									title="Copy bibliography of all cited papers">
									Copy Bibliography
								</button>
								<button onclick={handleResetCitations}
									class="px-2 py-1.5 text-[10px] font-medium rounded-[var(--radius-sm)]
									       bg-[var(--bg-active)] text-[var(--text-muted)] hover:bg-[var(--danger-light)] hover:text-[var(--danger)] transition-colors"
									title="Clear all citation marks">
									Reset
								</button>
							{/if}
							<button onclick={handleExportBibtex}
								class="px-2.5 py-1.5 text-[10px] font-medium rounded-[var(--radius-sm)]
								       bg-[var(--bg-active)] text-[var(--text-secondary)] hover:bg-[var(--accent-light)] hover:text-[var(--accent)] transition-colors"
								title="Export all references as BibTeX file">
								Export .bib
							</button>
						</div>
					</div>
					<SearchBar bind:value={query} placeholder={activeView === 'folder' ? 'Search this collection...' : 'Search all references...'} />
				</div>
				<div class="flex-1 overflow-y-auto">
					{#if results.length > 0}
						{#each results as ref, i}
							<CitationItem
								reference={ref}
								selected={i === selectedIndex}
								{citationStyle}
								ondelete={handleRefDelete}
								onmove={openMoveDialog}
								oncite={refreshResults}
								onupdate={() => refreshResults()}
							/>
						{/each}
					{:else}
						<div class="flex flex-col items-center justify-center h-full text-[var(--text-muted)]">
							<svg class="w-12 h-12 mb-3 opacity-20" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1">
								<path stroke-linecap="round" stroke-linejoin="round"
									d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
							</svg>
							{#if query}
								<p class="text-[13px]">No matching references</p>
								<p class="text-[11px] mt-1 opacity-60">Try different keywords</p>
							{:else}
								<p class="text-[13px]">No references yet</p>
								<p class="text-[11px] mt-1 opacity-60">Drop PDFs here, or use "Add by DOI" / "Search Online"</p>
							{/if}
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</div>

	<!-- Drag and drop overlay -->
	{#if dragOver}
		<div class="fixed inset-0 z-50 flex items-center justify-center bg-[var(--bg)]/80 backdrop-blur-sm pointer-events-none">
			<div class="flex flex-col items-center gap-4 p-12 border-2 border-dashed border-[var(--accent)] rounded-[var(--radius-lg)] bg-[var(--accent-light)]">
				<svg class="w-16 h-16 text-[var(--accent)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5">
					<path stroke-linecap="round" stroke-linejoin="round" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"/>
				</svg>
				<div class="text-center">
					<p class="text-lg font-semibold text-[var(--accent)]">Drop PDF files here</p>
					<p class="text-[12px] text-[var(--text-secondary)] mt-1">DOI will be extracted and metadata fetched automatically</p>
				</div>
			</div>
		</div>
	{/if}

	<!-- Importing overlay -->
	{#if importing}
		<div class="fixed inset-0 z-50 flex items-center justify-center bg-[var(--bg)]/80 backdrop-blur-sm">
			<div class="flex flex-col items-center gap-3 p-8 bg-[var(--bg-card)] rounded-[var(--radius-lg)] shadow-[var(--shadow-lg)] border border-[var(--border)]">
				<div class="w-8 h-8 border-3 border-[var(--accent)] border-t-transparent rounded-full animate-spin"></div>
				<p class="text-[13px] font-medium text-[var(--text)]">Importing PDFs...</p>
				<p class="text-[11px] text-[var(--text-muted)]">Extracting DOI and fetching metadata</p>
			</div>
		</div>
	{/if}

	<!-- Status toast -->
	{#if statusMessage}
		<div class="fixed bottom-4 right-4 px-4 py-2.5 rounded-[var(--radius-sm)] shadow-[var(--shadow-lg)] text-[12px] font-medium z-50 animate-[slideUp_0.2s_ease-out]
		     {statusType === 'error' ? 'bg-[var(--danger)] text-white' : 'bg-[var(--text)] text-white'}">
			{statusMessage}
		</div>
	{/if}

	<!-- Move to folder dialog -->
	{#if showMoveDialog}
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="fixed inset-0 bg-black/30 z-50 flex items-center justify-center" onclick={() => showMoveDialog = false} onkeydown={() => {}}>
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div class="bg-[var(--bg-card)] rounded-[var(--radius-lg)] shadow-[var(--shadow-lg)] w-72 p-4 border border-[var(--border)]"
				onclick={(e) => e.stopPropagation()} onkeydown={() => {}}>
				<h3 class="text-[13px] font-semibold text-[var(--text)] mb-3">Move to collection</h3>
				<div class="space-y-1">
					<button onclick={() => handleMove(null)}
						class="w-full text-left px-3 py-2 text-[12px] rounded-[var(--radius-sm)] hover:bg-[var(--bg-hover)] text-[var(--text-secondary)] transition-colors">
						No collection (All References)
					</button>
					{#each folders as folder}
						<button onclick={() => handleMove(folder.id)}
							class="w-full text-left px-3 py-2 text-[12px] rounded-[var(--radius-sm)] hover:bg-[var(--bg-hover)] text-[var(--text-secondary)] transition-colors flex items-center gap-2">
							<span class="w-2.5 h-2.5 rounded-full shrink-0" style="background:{folder.color}"></span>
							{folder.name}
						</button>
					{/each}
				</div>
				<button onclick={() => showMoveDialog = false}
					class="w-full mt-3 px-3 py-2 text-[11px] text-[var(--text-muted)] hover:text-[var(--text)] text-center transition-colors">
					Cancel
				</button>
			</div>
		</div>
	{/if}
</div>

<style>
	@keyframes slideUp {
		from { opacity: 0; transform: translateY(8px); }
		to { opacity: 1; transform: translateY(0); }
	}
</style>
