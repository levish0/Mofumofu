<script lang="ts">
	import { Badge } from '../ui/badge';

	const {
		title = '',
		summary = '',
		date = '',
		isSkeleton = false
	}: {
		title: string;
		summary: string;
		date: string;
		isSkeleton?: boolean;
	} = $props();
</script>

<div class="flex flex-1 flex-col px-3 py-2">
	{#if isSkeleton}
		<div class="shimmer mb-3 h-5 w-4/5 rounded"></div>
		<div class="mb-4 space-y-2">
			<div class="shimmer h-3 w-full rounded"></div>
			<div class="shimmer h-3 w-full rounded"></div>
			<div class="shimmer h-3 w-3/4 rounded"></div>
		</div>

		<div class="mt-auto py-2">
			<div class="flex items-center gap-2 text-xs">
				<div class="shimmer h-3 w-20 rounded"></div>
				<div class="shimmer h-3 w-16 rounded"></div>
			</div>
		</div>
	{:else}
		<h3 class="dark:text-mofu-dark-100 text-mofu-light-100 mb-3 line-clamp-1 text-left text-lg font-bold break-all">
			{title}
		</h3>
		<p class="dark:text-mofu-dark-300 text-mofu-light-300 mb-3 line-clamp-4 text-sm break-all">{summary}</p>

		<div class="mt-auto py-2">
			<!-- 날짜만 표시 -->
			<div class="flex items-center text-xs">
				<span class="text-mofu-light-400 dark:text-mofu-dark-400">
					{date}
				</span>
			</div>
		</div>
	{/if}
</div>
