<script lang="ts">
	import TitleBar from '$lib/components/TitleBar.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import CitationItem from '$lib/components/CitationItem.svelte';
	import HelpGuide from '$lib/components/HelpGuide.svelte';
	import {
		searchReferences, fetchDoi, addReference, searchOnline, moveReference, getFolders,
		copyRichBibliography, insertBibliographyIntoWord, formatBatchBibliography,
		type CitationStyle
	} from '$lib/tauri';
	import type { Reference, NewReference, Folder } from '$lib/tauri';

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
							<p class="text-[11px] text-[var(--text-muted)] mt-0.5">{results.length} reference{results.length !== 1 ? 's' : ''}</p>
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
								ondelete={handleRefDelete}
								onmove={openMoveDialog}
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
								<p class="text-[11px] mt-1 opacity-60">Add references using "Add by DOI" or "Search Online"</p>
							{/if}
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</div>

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
