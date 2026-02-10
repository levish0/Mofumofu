<script lang="ts">
	import Badge from '$lib/components/ui/badge/badge.svelte';

	let {
		title = '',
		summary,
		date,
		comments = 0,
		hashtags = [],
		isSkeleton = false
	}: {
		title?: string;
		summary?: string | null;
		date?: string;
		comments?: number;
		hashtags?: string[];
		isSkeleton?: boolean;
	} = $props();

	const MAX_TAGS = 4;

	const formattedDate = $derived(
		date
			? new Date(date).toLocaleString('en-CA', {
					year: 'numeric',
					month: '2-digit',
					day: '2-digit',
					hour: '2-digit',
					minute: '2-digit',
					hour12: false
				}).replace(',', '')
			: ''
	);

	const visibleTags = $derived(hashtags.slice(0, MAX_TAGS));
	const extraCount = $derived(Math.max(0, hashtags.length - MAX_TAGS));
</script>

<div class="flex flex-1 flex-col gap-2 p-3">
	{#if isSkeleton}
		<div class="shimmer h-5 w-3/4 rounded"></div>
		<div class="flex flex-col gap-1.5">
			<div class="shimmer h-3.5 w-full rounded"></div>
			<div class="shimmer h-3.5 w-full rounded"></div>
			<div class="shimmer h-3.5 w-5/6 rounded"></div>
		</div>
		<div class="mt-auto flex gap-1.5">
			<div class="shimmer h-5 w-12 rounded-full"></div>
			<div class="shimmer h-5 w-14 rounded-full"></div>
		</div>
		<div class="shimmer h-3 w-1/3 rounded"></div>
	{:else}
		<h3 class="line-clamp-1 text-lg font-bold text-mofu-light-100 dark:text-mofu-dark-100">
			{title}
		</h3>

		{#if summary}
			<p class="line-clamp-4 text-sm text-mofu-light-400 dark:text-mofu-dark-300">
				{summary}
			</p>
		{/if}

		<div class="mt-auto py-2">
			{#if visibleTags.length > 0}
				<div class="mb-3 flex flex-wrap gap-1.5">
					{#each visibleTags as tag}
						<Badge variant="secondary" class="text-xs">{tag}</Badge>
					{/each}
					{#if extraCount > 0}
						<Badge variant="outline" class="text-xs">+{extraCount}</Badge>
					{/if}
				</div>
			{/if}

			<p class="text-xs text-mofu-light-500 dark:text-mofu-dark-400">
				{formattedDate}{#if comments > 0}<span class="mx-1">&middot;</span>{comments} comment{comments !== 1 ? 's' : ''}{/if}
			</p>
		</div>
	{/if}
</div>
