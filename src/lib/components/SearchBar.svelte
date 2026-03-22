<script lang="ts">
	let { value = $bindable(''), onsubmit, placeholder = 'Search references...' }: {
		value?: string;
		onsubmit?: () => void;
		placeholder?: string;
	} = $props();
</script>

<div class="relative">
	<svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-[var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
		<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
	</svg>
	<input
		bind:value
		onkeydown={(e) => { if (e.key === 'Enter' && onsubmit) onsubmit(); }}
		{placeholder}
		class="w-full bg-[var(--bg-input)] text-[var(--text)] placeholder-[var(--text-muted)]
		       pl-9 pr-3 py-2 text-[13px] rounded-[var(--radius-sm)] border border-[var(--border)]
		       outline-none focus:border-[var(--accent)] focus:shadow-[0_0_0_3px_var(--accent-light)]
		       transition-all"
	/>
</div>
