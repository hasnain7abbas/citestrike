<script lang="ts">
	import type { Reference, CitationStyle } from '$lib/tauri';
	import { formatRef } from '$lib/tauri';

	let { reference, selected = false, onclick }: {
		reference: Reference;
		selected?: boolean;
		onclick?: () => void;
	} = $props();

	let copied = $state(false);

	async function copyFormatted(style: CitationStyle, e: MouseEvent) {
		e.stopPropagation();
		try {
			const text = await formatRef(reference, style);
			await navigator.clipboard.writeText(text);
			copied = true;
			setTimeout(() => copied = false, 1500);
		} catch {
			// fallback
		}
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	onclick={onclick}
	onkeydown={() => {}}
	role="option"
	aria-selected={selected}
	class="w-full text-left px-4 py-3 rounded-lg transition-all cursor-pointer
	       {selected ? 'bg-[var(--accent)]/10 border border-[var(--accent)]/30' : 'hover:bg-[var(--bg-elevated)] border border-transparent'}
	       group"
>
	<div class="flex items-start justify-between gap-3">
		<div class="min-w-0 flex-1">
			<h3 class="text-[var(--text-primary)] font-medium truncate text-sm">
				{reference.title}
			</h3>
			<p class="text-[var(--text-secondary)] text-xs mt-0.5 truncate">
				{reference.authors}{reference.year ? ` (${reference.year})` : ''}
			</p>
			{#if reference.journal}
				<p class="text-[var(--text-secondary)] text-xs mt-0.5 italic truncate opacity-60">
					{reference.journal}
				</p>
			{/if}
		</div>
		<div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity shrink-0">
			<button onclick={(e) => copyFormatted('APA', e)}
				class="px-2 py-1 text-[10px] rounded bg-[var(--bg-elevated)] text-[var(--text-secondary)]
				       hover:text-[var(--accent)] hover:bg-[var(--accent)]/10 transition-colors">
				APA
			</button>
			<button onclick={(e) => copyFormatted('MLA', e)}
				class="px-2 py-1 text-[10px] rounded bg-[var(--bg-elevated)] text-[var(--text-secondary)]
				       hover:text-[var(--accent)] hover:bg-[var(--accent)]/10 transition-colors">
				MLA
			</button>
			<button onclick={(e) => copyFormatted('BibTeX', e)}
				class="px-2 py-1 text-[10px] rounded bg-[var(--bg-elevated)] text-[var(--text-secondary)]
				       hover:text-[var(--accent)] hover:bg-[var(--accent)]/10 transition-colors">
				BIB
			</button>
		</div>
	</div>
	{#if copied}
		<div class="text-[var(--success)] text-xs mt-1">Copied to clipboard!</div>
	{/if}
</div>
