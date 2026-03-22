<script lang="ts">
	import SearchBar from '$lib/components/SearchBar.svelte';
	import CitationItem from '$lib/components/CitationItem.svelte';
	import { searchReferences, fetchDoi, addReference, searchOnline } from '$lib/tauri';
	import type { Reference, NewReference } from '$lib/tauri';

	let query = $state('');
	let results = $state<Reference[]>([]);
	let selectedIndex = $state(0);
	let loading = $state(false);
	let view = $state<'search' | 'add-doi' | 'online-search'>('search');
	let doiInput = $state('');
	let statusMessage = $state('');
	let onlineResults = $state<NewReference[]>([]);

	let debounceTimer: ReturnType<typeof setTimeout>;

	$effect(() => {
		if (view === 'search') {
			clearTimeout(debounceTimer);
			debounceTimer = setTimeout(async () => {
				try {
					results = await searchReferences(query);
					selectedIndex = 0;
				} catch {
					results = [];
				}
			}, 150);
		}
	});

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'ArrowDown') {
			e.preventDefault();
			selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			selectedIndex = Math.max(selectedIndex - 1, 0);
		} else if (e.key === 'Escape') {
			if (view !== 'search') {
				view = 'search';
			}
		}
	}

	async function handleAddDoi() {
		if (!doiInput.trim()) return;
		loading = true;
		statusMessage = '';
		try {
			const ref = await fetchDoi(doiInput.trim());
			await addReference(ref);
			statusMessage = 'Added successfully!';
			doiInput = '';
			view = 'search';
			results = await searchReferences('');
		} catch (e) {
			statusMessage = `Error: ${e}`;
		}
		loading = false;
	}

	async function handleOnlineSearch() {
		if (!query.trim()) return;
		loading = true;
		try {
			onlineResults = await searchOnline(query);
		} catch {
			onlineResults = [];
		}
		loading = false;
	}

	async function addFromOnline(ref: NewReference) {
		try {
			await addReference(ref);
			statusMessage = 'Added to library!';
			setTimeout(() => statusMessage = '', 2000);
		} catch (e) {
			statusMessage = `Error: ${e}`;
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="w-full h-screen bg-[var(--bg-primary)] flex flex-col overflow-hidden rounded-2xl border border-[var(--border)]/50">
	<!-- Header -->
	<div class="px-4 pt-4 pb-2" data-tauri-drag-region>
		<div class="flex items-center gap-2 mb-3">
			<div class="w-6 h-6 rounded-md bg-gradient-to-br from-[var(--accent)] to-[var(--accent-alt)] flex items-center justify-center">
				<svg class="w-3.5 h-3.5 text-white" viewBox="0 0 24 24" fill="currentColor">
					<path d="M13 3L4 14h7l-2 7 9-11h-7l2-7z"/>
				</svg>
			</div>
			<span class="text-[var(--text-primary)] font-semibold text-sm tracking-wide">CiteStrike</span>

			<!-- Tab buttons -->
			<div class="flex gap-1 ml-auto">
				<button onclick={() => view = 'search'}
					class="px-2 py-1 text-xs rounded-md transition-colors
					       {view === 'search' ? 'bg-[var(--accent)]/15 text-[var(--accent)]' : 'text-[var(--text-secondary)] hover:text-[var(--text-primary)]'}">
					Library
				</button>
				<button onclick={() => view = 'add-doi'}
					class="px-2 py-1 text-xs rounded-md transition-colors
					       {view === 'add-doi' ? 'bg-[var(--accent)]/15 text-[var(--accent)]' : 'text-[var(--text-secondary)] hover:text-[var(--text-primary)]'}">
					+ DOI
				</button>
				<button onclick={() => { view = 'online-search'; }}
					class="px-2 py-1 text-xs rounded-md transition-colors
					       {view === 'online-search' ? 'bg-[var(--accent)]/15 text-[var(--accent)]' : 'text-[var(--text-secondary)] hover:text-[var(--text-primary)]'}">
					Search Online
				</button>
			</div>
		</div>

		{#if view === 'search'}
			<SearchBar bind:value={query} />
		{:else if view === 'add-doi'}
			<div class="relative">
				<input
					bind:value={doiInput}
					placeholder="Enter DOI (e.g. 10.1038/s41586-021-03819-2)"
					class="w-full bg-[var(--bg-secondary)] text-[var(--text-primary)] placeholder-[var(--text-secondary)]
					       px-4 py-4 text-lg rounded-xl border border-[var(--border)] outline-none
					       focus:border-[var(--accent)] focus:ring-1 focus:ring-[var(--accent)] transition-all"
					onkeydown={(e) => e.key === 'Enter' && handleAddDoi()}
					autofocus
				/>
			</div>
		{:else}
			<SearchBar bind:value={query} placeholder="Search Crossref..." onsubmit={handleOnlineSearch} />
		{/if}
	</div>

	<!-- Results -->
	<div class="flex-1 overflow-y-auto px-4 pb-4 scrollbar-thin">
		{#if loading}
			<div class="flex items-center justify-center py-12">
				<div class="w-6 h-6 border-2 border-[var(--accent)] border-t-transparent rounded-full animate-spin"></div>
			</div>
		{:else if view === 'search'}
			{#if results.length > 0}
				<div class="space-y-1 mt-2">
					{#each results as ref, i}
						<CitationItem
							reference={ref}
							selected={i === selectedIndex}
						/>
					{/each}
				</div>
			{:else}
				<div class="flex flex-col items-center justify-center py-12 text-[var(--text-secondary)]">
					<svg class="w-10 h-10 mb-3 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
							d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
					</svg>
					<p class="text-sm">
						{query ? 'No references found' : 'Your library is empty'}
					</p>
					<p class="text-xs mt-1 opacity-60">
						{query ? 'Try a different search term' : 'Add a DOI or search online to get started'}
					</p>
				</div>
			{/if}
		{:else if view === 'add-doi'}
			<div class="py-8 text-center">
				{#if statusMessage}
					<p class="text-sm {statusMessage.startsWith('Error') ? 'text-[var(--danger)]' : 'text-[var(--success)]'}">
						{statusMessage}
					</p>
				{:else}
					<p class="text-[var(--text-secondary)] text-sm">
						Paste a DOI and press Enter to auto-fetch metadata from Crossref
					</p>
				{/if}
			</div>
		{:else}
			{#if onlineResults.length > 0}
				<div class="space-y-2 mt-2">
					{#each onlineResults as ref}
						<div class="px-4 py-3 rounded-lg hover:bg-[var(--bg-elevated)] border border-transparent
						            hover:border-[var(--border)] transition-all group">
							<div class="flex items-start justify-between gap-3">
								<div class="min-w-0 flex-1">
									<h3 class="text-[var(--text-primary)] font-medium text-sm truncate">{ref.title}</h3>
									<p class="text-[var(--text-secondary)] text-xs mt-0.5">{ref.authors}{ref.year ? ` (${ref.year})` : ''}</p>
									{#if ref.journal}
										<p class="text-[var(--text-secondary)] text-xs italic opacity-60">{ref.journal}</p>
									{/if}
								</div>
								<button onclick={() => addFromOnline(ref)}
									class="shrink-0 px-3 py-1.5 text-xs rounded-md bg-[var(--accent)] text-white
									       hover:bg-[var(--accent-alt)] transition-colors opacity-0 group-hover:opacity-100">
									Add
								</button>
							</div>
						</div>
					{/each}
				</div>
			{:else}
				<div class="py-8 text-center text-[var(--text-secondary)] text-sm">
					Search Crossref for papers and add them to your library
				</div>
			{/if}
		{/if}
	</div>

	<!-- Status bar -->
	{#if statusMessage && view !== 'add-doi'}
		<div class="px-4 py-2 text-xs text-center {statusMessage.startsWith('Error') ? 'text-[var(--danger)]' : 'text-[var(--success)]'}">
			{statusMessage}
		</div>
	{/if}

	<!-- Footer -->
	<div class="px-4 py-2 border-t border-[var(--border)]/30 flex items-center justify-between text-[10px] text-[var(--text-secondary)]">
		<div class="flex gap-3">
			<span><kbd class="px-1 py-0.5 rounded bg-[var(--bg-elevated)] text-[var(--text-secondary)]">↑↓</kbd> Navigate</span>
			<span><kbd class="px-1 py-0.5 rounded bg-[var(--bg-elevated)] text-[var(--text-secondary)]">Enter</kbd> Copy</span>
			<span><kbd class="px-1 py-0.5 rounded bg-[var(--bg-elevated)] text-[var(--text-secondary)]">Esc</kbd> Close</span>
		</div>
		<span>{results.length} references</span>
	</div>
</div>
