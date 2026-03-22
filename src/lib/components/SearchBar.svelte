<script lang="ts">
	let { value = $bindable(''), onsubmit, placeholder = 'Search your library...' }: {
		value?: string;
		onsubmit?: () => void;
		placeholder?: string;
	} = $props();

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			// Hide window via Tauri
			import('@tauri-apps/api/core').then(({ invoke }) => {
				invoke('plugin:window|hide');
			}).catch(() => {});
		}
		if (e.key === 'Enter' && onsubmit) {
			onsubmit();
		}
	}
</script>

<div class="relative">
	<svg class="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-[var(--text-secondary)]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
		<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
	</svg>
	<input
		bind:value
		onkeydown={handleKeydown}
		{placeholder}
		class="w-full bg-[var(--bg-secondary)] text-[var(--text-primary)] placeholder-[var(--text-secondary)]
		       pl-12 pr-4 py-4 text-lg rounded-xl border border-[var(--border)] outline-none
		       focus:border-[var(--accent)] focus:ring-1 focus:ring-[var(--accent)] transition-all"
		autofocus
	/>
</div>
