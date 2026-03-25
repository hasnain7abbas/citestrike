<script lang="ts">
	import type { Folder } from '$lib/tauri';
	import { createFolder, getFolders, deleteFolder, renameFolder } from '$lib/tauri';

	let { folders = $bindable([]), activeView = $bindable('all'), activeFolderId = $bindable(null) }: {
		folders?: Folder[];
		activeView?: string;
		activeFolderId?: string | null;
	} = $props();

	let showNewFolder = $state(false);
	let newFolderName = $state('');
	let editingId = $state<string | null>(null);
	let editName = $state('');
	let contextFolderId = $state<string | null>(null);

	const FOLDER_COLORS = ['#3B6CF0', '#E5484D', '#30A46C', '#F5A623', '#8B5CF6', '#EC4899', '#14B8A6', '#F97316'];

	async function loadFolders() {
		folders = await getFolders();
	}

	async function handleCreateFolder() {
		if (!newFolderName.trim()) return;
		const color = FOLDER_COLORS[folders.length % FOLDER_COLORS.length];
		await createFolder(newFolderName.trim(), color);
		newFolderName = '';
		showNewFolder = false;
		await loadFolders();
	}

	async function handleRename(id: string) {
		if (!editName.trim()) { editingId = null; return; }
		await renameFolder(id, editName.trim());
		editingId = null;
		await loadFolders();
	}

	async function handleDelete(id: string) {
		await deleteFolder(id);
		contextFolderId = null;
		if (activeFolderId === id) { activeView = 'all'; activeFolderId = null; }
		await loadFolders();
	}

	function selectView(view: string, folderId: string | null = null) {
		activeView = view;
		activeFolderId = folderId;
	}

	loadFolders();
</script>

<div class="w-52 bg-[var(--bg-sidebar)] border-r border-[var(--border-light)] flex flex-col h-full shrink-0 select-none">
	<!-- Nav -->
	<div class="px-2 pt-3 pb-1">
		<p class="px-2 text-[10px] font-semibold text-[var(--text-muted)] uppercase tracking-wider mb-1.5">Library</p>

		<button onclick={() => selectView('all')}
			class="w-full flex items-center gap-2.5 px-2.5 py-[7px] rounded-[var(--radius-sm)] text-[12px] transition-colors
			       {activeView === 'all' ? 'bg-[var(--accent-light)] text-[var(--accent)] font-medium' : 'text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]'}">
			<svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8">
				<path stroke-linecap="round" stroke-linejoin="round" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
			</svg>
			All References
		</button>

		<button onclick={() => selectView('add-doi')}
			class="w-full flex items-center gap-2.5 px-2.5 py-[7px] rounded-[var(--radius-sm)] text-[12px] transition-colors
			       {activeView === 'add-doi' ? 'bg-[var(--accent-light)] text-[var(--accent)] font-medium' : 'text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]'}">
			<svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8">
				<path stroke-linecap="round" stroke-linejoin="round" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"/>
			</svg>
			Add by DOI
		</button>

		<button onclick={() => selectView('online')}
			class="w-full flex items-center gap-2.5 px-2.5 py-[7px] rounded-[var(--radius-sm)] text-[12px] transition-colors
			       {activeView === 'online' ? 'bg-[var(--accent-light)] text-[var(--accent)] font-medium' : 'text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]'}">
			<svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8">
				<path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
			</svg>
			Search Online
		</button>

		<button onclick={() => selectView('add-manual')}
			class="w-full flex items-center gap-2.5 px-2.5 py-[7px] rounded-[var(--radius-sm)] text-[12px] transition-colors
			       {activeView === 'add-manual' ? 'bg-[var(--accent-light)] text-[var(--accent)] font-medium' : 'text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]'}">
			<svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8">
				<path stroke-linecap="round" stroke-linejoin="round" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
			</svg>
			Add Manually
		</button>

		<button onclick={() => selectView('bibliography')}
			class="w-full flex items-center gap-2.5 px-2.5 py-[7px] rounded-[var(--radius-sm)] text-[12px] transition-colors
			       {activeView === 'bibliography' ? 'bg-[var(--accent-light)] text-[var(--accent)] font-medium' : 'text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]'}">
			<svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8">
				<path stroke-linecap="round" stroke-linejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
			</svg>
			Build Bibliography
		</button>

		<button onclick={() => selectView('help')}
			class="w-full flex items-center gap-2.5 px-2.5 py-[7px] rounded-[var(--radius-sm)] text-[12px] transition-colors
			       {activeView === 'help' ? 'bg-[var(--accent-light)] text-[var(--accent)] font-medium' : 'text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]'}">
			<svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.8">
				<path stroke-linecap="round" stroke-linejoin="round" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
			</svg>
			Help Guide
		</button>
	</div>

	<!-- Folders -->
	<div class="px-2 mt-3 flex-1 overflow-y-auto">
		<div class="flex items-center justify-between px-2 mb-1.5">
			<p class="text-[10px] font-semibold text-[var(--text-muted)] uppercase tracking-wider">Collections</p>
			<button onclick={() => showNewFolder = true}
				class="p-0.5 rounded hover:bg-[var(--bg-hover)] transition-colors">
				<svg class="w-3.5 h-3.5 text-[var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
					<path stroke-linecap="round" d="M12 5v14m-7-7h14"/>
				</svg>
			</button>
		</div>

		{#if showNewFolder}
			<div class="px-1 mb-1">
				<input
					bind:value={newFolderName}
					placeholder="Folder name..."
					onkeydown={(e) => { if (e.key === 'Enter') handleCreateFolder(); if (e.key === 'Escape') showNewFolder = false; }}
					class="w-full px-2 py-1.5 text-[11px] border border-[var(--accent)] rounded-[var(--radius-sm)] outline-none bg-[var(--bg-input)]
					       focus:shadow-[0_0_0_3px_var(--accent-light)]"
				/>
			</div>
		{/if}

		{#each folders as folder}
			<div class="relative group">
				{#if editingId === folder.id}
					<input
						bind:value={editName}
						onkeydown={(e) => { if (e.key === 'Enter') handleRename(folder.id); if (e.key === 'Escape') editingId = null; }}
						class="w-full px-2 py-1.5 text-[11px] border border-[var(--accent)] rounded-[var(--radius-sm)] outline-none bg-[var(--bg-input)] ml-1"
					/>
				{:else}
					<button
						onclick={() => selectView('folder', folder.id)}
						class="w-full flex items-center gap-2.5 px-2.5 py-[7px] rounded-[var(--radius-sm)] text-[12px] transition-colors
						       {activeView === 'folder' && activeFolderId === folder.id ? 'bg-[var(--accent-light)] text-[var(--accent)] font-medium' : 'text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]'}"
					>
						<span class="w-2.5 h-2.5 rounded-full shrink-0" style="background:{folder.color}"></span>
						<span class="truncate">{folder.name}</span>
					</button>

					<!-- Folder context actions -->
					<div class="absolute right-1 top-1/2 -translate-y-1/2 flex opacity-0 group-hover:opacity-100 transition-opacity">
						<button onclick={() => { editingId = folder.id; editName = folder.name; }}
							class="p-1 rounded hover:bg-[var(--bg-active)] transition-colors">
							<svg class="w-3 h-3 text-[var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
								<path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
							</svg>
						</button>
						<button onclick={() => handleDelete(folder.id)}
							class="p-1 rounded hover:bg-[var(--danger-light)] transition-colors">
							<svg class="w-3 h-3 text-[var(--text-muted)] hover:text-[var(--danger)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
								<path stroke-linecap="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
							</svg>
						</button>
					</div>
				{/if}
			</div>
		{/each}

		{#if folders.length === 0 && !showNewFolder}
			<p class="px-2.5 text-[11px] text-[var(--text-muted)] italic">No collections yet</p>
		{/if}
	</div>

	<!-- Bottom -->
	<div class="px-4 py-3 border-t border-[var(--border-light)]">
		<div class="flex items-center gap-2 text-[10px] text-[var(--text-muted)]">
			<kbd class="px-1 py-0.5 rounded border border-[var(--border)] bg-[var(--bg-card)] text-[9px] font-mono">Ctrl+Shift+C</kbd>
			<span>Quick access</span>
		</div>
	</div>
</div>
