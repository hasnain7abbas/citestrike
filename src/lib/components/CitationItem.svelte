<script lang="ts">
	import type { Reference, CitationStyle } from '$lib/tauri';
	import { formatRef, deleteReference, formatInlineCitation, copyRichCitation, copyRichInline, insertCitationIntoWord, insertInlineIntoWord, insertCitationIntoPpt, citeReference, unciteReference, updateReference } from '$lib/tauri';

	let { reference, selected = false, citationStyle = 'APA' as CitationStyle, ondelete, onmove, oncite, onupdate }: {
		reference: Reference;
		selected?: boolean;
		citationStyle?: CitationStyle;
		ondelete?: (id: string) => void;
		onmove?: (id: string) => void;
		oncite?: () => void;
		onupdate?: (id: string) => void;
	} = $props();

	let copied = $state('');
	let showMenu = $state(false);
	let statusMsg = $state('');
	let editing = $state(false);
	let editTitle = $state('');
	let editAuthors = $state('');
	let editYear = $state('');
	let editJournal = $state('');
	let editDoi = $state('');
	let editVolume = $state('');
	let editIssue = $state('');
	let editPages = $state('');

	async function copyRich(style: CitationStyle) {
		try {
			await copyRichCitation(reference, style);
			copied = style;
			setTimeout(() => copied = '', 1500);
		} catch {
			// Fallback to plain text
			const text = await formatRef(reference, style);
			await navigator.clipboard.writeText(text);
			copied = style;
			setTimeout(() => copied = '', 1500);
		}
		showMenu = false;
	}

	async function copyInline(style: CitationStyle) {
		try {
			await copyRichInline(reference, style);
			copied = 'inline';
			setTimeout(() => copied = '', 1500);
		} catch {
			const text = await formatInlineCitation(reference, style);
			await navigator.clipboard.writeText(text);
			copied = 'inline';
			setTimeout(() => copied = '', 1500);
		}
		showMenu = false;
	}

	async function insertWord(mode: 'full' | 'inline', style: CitationStyle) {
		try {
			const title = mode === 'full'
				? await insertCitationIntoWord(reference, style)
				: await insertInlineIntoWord(reference, style);
			statusMsg = `Inserted into ${title.substring(0, 30)}...`;
			setTimeout(() => statusMsg = '', 2500);
		} catch (e) {
			statusMsg = `${e}`;
			setTimeout(() => statusMsg = '', 3000);
		}
		showMenu = false;
	}

	async function insertPpt(style: CitationStyle) {
		try {
			const title = await insertCitationIntoPpt(reference, style);
			statusMsg = `Inserted into ${title.substring(0, 30)}...`;
			setTimeout(() => statusMsg = '', 2500);
		} catch (e) {
			statusMsg = `${e}`;
			setTimeout(() => statusMsg = '', 3000);
		}
		showMenu = false;
	}

	async function handleCite() {
		try {
			const inline = await citeReference(reference.id, citationStyle);
			statusMsg = `Copied: ${inline}`;
			setTimeout(() => statusMsg = '', 2500);
			oncite?.();
		} catch (e) {
			statusMsg = `${e}`;
			setTimeout(() => statusMsg = '', 3000);
		}
	}

	async function handleUncite() {
		try {
			await unciteReference(reference.id);
			oncite?.();
		} catch { /* */ }
		showMenu = false;
	}

	async function handleDelete() {
		try {
			await deleteReference(reference.id);
			ondelete?.(reference.id);
		} catch { /* */ }
		showMenu = false;
	}

	function startEdit() {
		editTitle = reference.title;
		editAuthors = reference.authors;
		editYear = reference.year?.toString() ?? '';
		editJournal = reference.journal ?? '';
		editDoi = reference.doi ?? '';
		editVolume = reference.volume ?? '';
		editIssue = reference.issue ?? '';
		editPages = reference.pages ?? '';
		editing = true;
		showMenu = false;
	}

	async function saveEdit() {
		try {
			await updateReference(reference.id, {
				title: editTitle,
				authors: editAuthors,
				year: editYear ? parseInt(editYear) : null,
				doi: editDoi || null,
				journal: editJournal || null,
				volume: editVolume || null,
				issue: editIssue || null,
				pages: editPages || null,
				abstract_text: reference.abstract_text,
				url: reference.url,
				ref_type: reference.ref_type,
			});
			editing = false;
			onupdate?.(reference.id);
		} catch (e) {
			statusMsg = `${e}`;
			setTimeout(() => statusMsg = '', 3000);
		}
	}
</script>

{#if editing}
	<!-- Edit form -->
	<div class="px-4 py-3 border-b border-[var(--border-light)] bg-[var(--bg-card)]">
		<div class="space-y-2">
			<input bind:value={editTitle} placeholder="Title *" class="w-full px-2.5 py-1.5 text-[12px] bg-[var(--bg-input)] text-[var(--text)] border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)]" />
			<input bind:value={editAuthors} placeholder="Authors (LastName, First; LastName2, First2)" class="w-full px-2.5 py-1.5 text-[12px] bg-[var(--bg-input)] text-[var(--text)] border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)]" />
			<div class="flex gap-2">
				<input bind:value={editYear} placeholder="Year" type="number" class="w-20 px-2.5 py-1.5 text-[12px] bg-[var(--bg-input)] text-[var(--text)] border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)]" />
				<input bind:value={editJournal} placeholder="Journal" class="flex-1 px-2.5 py-1.5 text-[12px] bg-[var(--bg-input)] text-[var(--text)] border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)]" />
			</div>
			<div class="flex gap-2">
				<input bind:value={editDoi} placeholder="DOI" class="flex-1 px-2.5 py-1.5 text-[12px] bg-[var(--bg-input)] text-[var(--text)] border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)]" />
				<input bind:value={editVolume} placeholder="Vol" class="w-16 px-2.5 py-1.5 text-[12px] bg-[var(--bg-input)] text-[var(--text)] border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)]" />
				<input bind:value={editIssue} placeholder="Issue" class="w-16 px-2.5 py-1.5 text-[12px] bg-[var(--bg-input)] text-[var(--text)] border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)]" />
				<input bind:value={editPages} placeholder="Pages" class="w-20 px-2.5 py-1.5 text-[12px] bg-[var(--bg-input)] text-[var(--text)] border border-[var(--border)] rounded-[var(--radius-sm)] outline-none focus:border-[var(--accent)]" />
			</div>
			<div class="flex gap-2 justify-end">
				<button onclick={() => editing = false} class="px-3 py-1.5 text-[11px] text-[var(--text-muted)] hover:text-[var(--text)] rounded-[var(--radius-sm)] transition-colors">Cancel</button>
				<button onclick={saveEdit} class="px-3 py-1.5 text-[11px] font-medium bg-[var(--accent)] text-white rounded-[var(--radius-sm)] hover:bg-[var(--accent-hover)] transition-colors">Save</button>
			</div>
		</div>
	</div>
{:else}
<div
	class="group relative px-4 py-3 border-b border-[var(--border-light)] hover:bg-[var(--bg-hover)] transition-colors
	       {selected ? 'bg-[var(--accent-light)] border-l-2 border-l-[var(--accent)]' : ''}"
>
	<div class="flex items-start justify-between gap-3">
		<div class="min-w-0 flex-1">
			<div class="flex items-center gap-2">
				{#if reference.cited}
					<span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-[9px] font-bold bg-[var(--accent)] text-white shrink-0" title="Cited #{reference.cite_order}">
						{reference.cite_order ?? ''}
					</span>
				{/if}
				<h3 class="text-[var(--text)] font-medium text-[13px] leading-snug line-clamp-2">
					{reference.title}
				</h3>
			</div>
			<p class="text-[var(--text-secondary)] text-[11px] mt-1 truncate">
				{reference.authors}
			</p>
			<div class="flex items-center gap-2 mt-1.5 flex-wrap">
				{#if reference.year}
					<span class="inline-flex items-center px-1.5 py-0.5 rounded text-[10px] font-medium bg-[var(--bg-active)] text-[var(--text-secondary)]">
						{reference.year}
					</span>
				{/if}
				{#if reference.journal}
					<span class="text-[10px] text-[var(--text-muted)] italic truncate max-w-[200px]">
						{reference.journal}
					</span>
				{/if}
				{#if reference.doi}
					<span class="text-[10px] text-[var(--accent)] truncate max-w-[150px]">
						{reference.doi}
					</span>
				{/if}
			</div>
			{#if statusMsg}
				<p class="text-[10px] mt-1 {statusMsg.includes('No Microsoft') ? 'text-[var(--danger)]' : 'text-[var(--success)]'}">{statusMsg}</p>
			{/if}
		</div>

		<!-- Actions -->
		<div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity shrink-0 mt-0.5">
			<!-- Cite button (prominent) -->
			<button onclick={handleCite}
				class="px-2.5 py-1 text-[10px] font-semibold rounded-[var(--radius-sm)] transition-colors
				       {reference.cited
				         ? 'bg-[var(--accent)] text-white'
				         : 'bg-[var(--success-light)] text-[var(--success)] hover:bg-[var(--success)] hover:text-white'}"
				title={reference.cited ? 'Re-copy in-text citation' : 'Cite — copies in-text citation to clipboard'}>
				{reference.cited ? `Cited [${reference.cite_order}]` : 'Cite'}
			</button>

			<!-- Quick copy buttons (rich text) -->
			{#each (['APA', 'MLA', 'BibTeX'] as const) as style}
				<button onclick={() => copyRich(style)}
					class="px-2 py-1 text-[10px] font-medium rounded-[var(--radius-sm)]
					       {copied === style ? 'bg-[var(--success-light)] text-[var(--success)]' : 'bg-[var(--bg-active)] text-[var(--text-secondary)] hover:bg-[var(--accent-light)] hover:text-[var(--accent)]'}
					       transition-colors" title="Copy {style} (rich text)">
					{copied === style ? '✓' : style === 'BibTeX' ? 'BIB' : style}
				</button>
			{/each}

			<!-- More menu -->
			<div class="relative">
				<button onclick={() => showMenu = !showMenu}
					class="p-1 rounded-[var(--radius-sm)] hover:bg-[var(--bg-active)] transition-colors" title="More options">
					<svg class="w-3.5 h-3.5 text-[var(--text-muted)]" fill="currentColor" viewBox="0 0 16 16">
						<circle cx="8" cy="3" r="1.5"/><circle cx="8" cy="8" r="1.5"/><circle cx="8" cy="13" r="1.5"/>
					</svg>
				</button>

				{#if showMenu}
					<!-- svelte-ignore a11y_no_static_element_interactions -->
					<div class="fixed inset-0 z-40" onclick={() => showMenu = false} onkeydown={() => {}}></div>
					<div class="absolute right-0 top-7 z-50 w-52 bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius-sm)] shadow-[var(--shadow-lg)] py-1 max-h-[400px] overflow-y-auto">

						<!-- Cite/Uncite -->
						{#if reference.cited}
							<button onclick={handleUncite}
								class="w-full text-left px-3 py-1.5 text-[11px] text-[var(--warning)] hover:bg-[var(--bg-hover)] transition-colors">
								Uncite this reference
							</button>
							<div class="h-px bg-[var(--border-light)] my-1"></div>
						{/if}

						<!-- Insert into Word -->
						<p class="px-3 py-1 text-[9px] font-semibold text-[var(--text-muted)] uppercase tracking-wider flex items-center gap-1.5">
							<svg class="w-3 h-3" viewBox="0 0 24 24" fill="#2B579A"><path d="M23 1.5q.41 0 .7.3.3.29.3.7v19q0 .41-.3.7-.29.3-.7.3H7q-.41 0-.7-.3-.3-.29-.3-.7V18H1q-.41 0-.7-.3-.3-.29-.3-.7V7q0-.41.3-.7Q.58 6 1 6h5V2.5q0-.41.3-.7.29-.3.7-.3zM6 13.28l1.42 2.66h2.14l-2.38-3.87 2.34-3.8H7.46l-1.3 2.4-.05.08-.04.09-.64-1.28-.66-1.29H2.59l2.27 3.82-2.48 3.85h2.16zM14 2.5H7V6h1.5q.41 0 .7.3.3.29.3.7v10q0 .41-.3.7-.29.3-.7.3H7v3.5h7V18h-1v-1h1v-2h-1v-1h1v-2h-1v-1h1V9h-1V8h1V6h-1V5h1zM21 20V17h-4v3zm0-4V13h-4v3zm0-4V9h-4v3zm0-4V5h-4v3z"/></svg>
							Insert into Word
						</p>
						{#each (['APA', 'MLA', 'IEEE', 'Chicago', 'Harvard', 'Vancouver'] as const) as style}
							<button onclick={() => insertWord('full', style)}
								class="w-full text-left px-3 py-1.5 text-[11px] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] hover:text-[var(--text)] transition-colors pl-6">
								{style} full citation
							</button>
						{/each}
						{#each (['APA', 'MLA', 'IEEE'] as const) as style}
							<button onclick={() => insertWord('inline', style)}
								class="w-full text-left px-3 py-1.5 text-[11px] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] hover:text-[var(--text)] transition-colors pl-6">
								{style} in-text
							</button>
						{/each}

						<div class="h-px bg-[var(--border-light)] my-1"></div>

						<!-- Insert into PowerPoint -->
						<p class="px-3 py-1 text-[9px] font-semibold text-[var(--text-muted)] uppercase tracking-wider flex items-center gap-1.5">
							<svg class="w-3 h-3" viewBox="0 0 24 24" fill="#D24726"><path d="M23 1.5q.41 0 .7.3.3.29.3.7v19q0 .41-.3.7-.29.3-.7.3H7q-.41 0-.7-.3-.3-.29-.3-.7V18H1q-.41 0-.7-.3-.3-.29-.3-.7V7q0-.41.3-.7Q.58 6 1 6h5V2.5q0-.41.3-.7.29-.3.7-.3zM6 13.25v3.63h1.84V7.13H4.87q-.58 0-1.05.13t-.82.4-.55.65-.2.88q0 .56.2.93.19.38.52.64t.72.39.82.22zm1.84-1.62H6V8.8h1.4q.57 0 .86.3.28.28.28.73 0 .5-.28.79-.29.29-.86.29zM14 2.5H7V6h1.5q.41 0 .7.3.3.29.3.7v10q0 .41-.3.7-.29.3-.7.3H7v3.5h7zm9 17.5V5h-7v15z"/></svg>
							Insert into PowerPoint
						</p>
						{#each (['APA', 'MLA', 'IEEE'] as const) as style}
							<button onclick={() => insertPpt(style)}
								class="w-full text-left px-3 py-1.5 text-[11px] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] hover:text-[var(--text)] transition-colors pl-6">
								{style} citation
							</button>
						{/each}

						<div class="h-px bg-[var(--border-light)] my-1"></div>

						<!-- Copy in-text -->
						<p class="px-3 py-1 text-[9px] font-semibold text-[var(--text-muted)] uppercase tracking-wider">Copy in-text</p>
						{#each (['APA', 'MLA', 'Chicago', 'IEEE', 'Harvard', 'Vancouver', 'BibTeX'] as const) as style}
							<button onclick={() => copyInline(style)}
								class="w-full text-left px-3 py-1.5 text-[11px] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] hover:text-[var(--text)] transition-colors pl-6">
								{style}
							</button>
						{/each}

						<div class="h-px bg-[var(--border-light)] my-1"></div>

						<!-- Copy full (rich text) -->
						<p class="px-3 py-1 text-[9px] font-semibold text-[var(--text-muted)] uppercase tracking-wider">Copy full (rich text)</p>
						{#each (['Chicago', 'IEEE', 'Harvard', 'Vancouver'] as const) as style}
							<button onclick={() => copyRich(style)}
								class="w-full text-left px-3 py-1.5 text-[11px] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] hover:text-[var(--text)] transition-colors pl-6">
								{style}
							</button>
						{/each}

						<div class="h-px bg-[var(--border-light)] my-1"></div>

						<!-- Actions -->
						<button onclick={startEdit}
							class="w-full text-left px-3 py-1.5 text-[11px] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] hover:text-[var(--text)] transition-colors">
							Edit reference
						</button>
						{#if onmove}
							<button onclick={() => { onmove?.(reference.id); showMenu = false; }}
								class="w-full text-left px-3 py-1.5 text-[11px] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] hover:text-[var(--text)] transition-colors">
								Move to folder...
							</button>
						{/if}
						<button onclick={handleDelete}
							class="w-full text-left px-3 py-1.5 text-[11px] text-[var(--danger)] hover:bg-[var(--danger-light)] transition-colors">
							Delete reference
						</button>
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
{/if}
