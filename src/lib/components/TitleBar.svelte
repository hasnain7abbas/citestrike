<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';

	const appWindow = getCurrentWindow();

	let maximized = $state(false);

	async function toggleMaximize() {
		await appWindow.toggleMaximize();
		maximized = await appWindow.isMaximized();
	}

	appWindow.isMaximized().then(v => maximized = v);
</script>

<div
	class="h-10 flex items-center justify-between bg-[var(--bg-sidebar)] border-b border-[var(--border-light)] select-none shrink-0"
	data-tauri-drag-region
>
	<div class="flex items-center gap-2.5 pl-4" data-tauri-drag-region>
		<img src="/logo.svg" alt="CiteStrike" class="w-5 h-5 shrink-0 rounded" />
		<span class="text-[var(--text)] font-semibold text-[13px] tracking-tight" data-tauri-drag-region>CiteStrike</span>
		<span class="text-[var(--text-muted)] text-[10px] font-medium tracking-wider uppercase ml-1" data-tauri-drag-region>v0.2</span>
	</div>

	<div class="flex items-center h-full">
		<button
			onclick={() => appWindow.minimize()}
			class="h-full w-11 flex items-center justify-center hover:bg-[var(--bg-hover)] transition-colors"
		>
			<svg class="w-3 h-3 text-[var(--text-secondary)]" fill="none" viewBox="0 0 12 12">
				<path d="M2 6h8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
			</svg>
		</button>
		<button
			onclick={toggleMaximize}
			class="h-full w-11 flex items-center justify-center hover:bg-[var(--bg-hover)] transition-colors"
		>
			{#if maximized}
				<svg class="w-3 h-3 text-[var(--text-secondary)]" fill="none" viewBox="0 0 12 12">
					<rect x="3" y="0.5" width="8.5" height="8.5" rx="1" stroke="currentColor" stroke-width="1.2"/>
					<rect x="0.5" y="3" width="8.5" height="8.5" rx="1" stroke="currentColor" stroke-width="1.2" fill="var(--bg-sidebar)"/>
				</svg>
			{:else}
				<svg class="w-3 h-3 text-[var(--text-secondary)]" fill="none" viewBox="0 0 12 12">
					<rect x="1" y="1" width="10" height="10" rx="1.5" stroke="currentColor" stroke-width="1.5"/>
				</svg>
			{/if}
		</button>
		<button
			onclick={() => appWindow.close()}
			class="h-full w-11 flex items-center justify-center hover:bg-[var(--danger)] hover:text-white transition-colors rounded-tr-none"
		>
			<svg class="w-3 h-3 text-[var(--text-secondary)]" fill="none" viewBox="0 0 12 12">
				<path d="M2 2l8 8M10 2l-8 8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
			</svg>
		</button>
	</div>
</div>
