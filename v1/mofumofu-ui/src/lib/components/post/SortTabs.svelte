<script lang="ts">
	import { postsStore } from '$lib/stores/posts.svelte';
	import { cn } from '$lib/utils';
	import { ArrowTrendingUp, Clock, Icon } from 'svelte-hero-icons';
	import * as m from '../../../paraglide/messages';

	interface Props {
		onSortChange?: (sortBy: string, timeRange?: string) => void;
		isVisible: () => boolean;
		isAtTop: () => boolean;
	}

	const { onSortChange, isVisible, isAtTop }: Props = $props();

	const currentSort = $derived(postsStore.filter.sortBy);
	const currentTimeRange = $derived(postsStore.filter.timeRange);
	const showTimeRange = $derived(currentSort === 'popular');

	const timeRangeOptions = [
		{ value: 'all', label: m.time_range_all() },
		{ value: 'today', label: m.time_range_today() },
		{ value: 'week', label: m.time_range_week() },
		{ value: 'month', label: m.time_range_month() },
		{ value: 'year', label: m.time_range_year() }
	];

	const handleSortClick = (sortBy: string) => {
		postsStore.updateFilter({ sortBy });
		onSortChange?.(sortBy);
	};

	const handleTimeRangeChange = (timeRange: string) => {
		postsStore.updateFilter({ timeRange });
		onSortChange?.(currentSort, timeRange);
	};

	// Calculate the top position based on navbar state (like SearchPanel)
	const topPosition = $derived(isVisible() ? '60px' : '0px');
</script>

<div
	class={cn(
		' sticky z-40 min-h-10 transition-all duration-100 ease-out',
		isAtTop() ? 'dark:bg-mofu-dark-900 bg-white' : 'dark:bg-mofu-dark-800 bg-white'
	)}
	style="top: {topPosition}"
>
	<div class="flex items-center justify-between px-4 py-3">
		<div class="flex items-center gap-6">
			<button
				class="flex items-center gap-2 text-sm font-medium transition-colors {currentSort === 'popular'
					? 'text-foreground'
					: 'text-muted-foreground hover:text-foreground'}"
				onclick={() => handleSortClick('popular')}
			>
				<Icon src={ArrowTrendingUp} class="h-5 w-5" solid />
				{m.post_sort_trending()}
			</button>

			<button
				class="flex items-center gap-2 text-sm font-medium transition-colors {currentSort === 'recent'
					? 'text-foreground'
					: 'text-muted-foreground hover:text-foreground'}"
				onclick={() => handleSortClick('recent')}
			>
				<Icon src={Clock} class="h-5 w-5" solid />
				{m.post_sort_recent()}
			</button>
		</div>

		{#if showTimeRange}
			<div class="relative">
				<select
					class="border-border bg-background text-foreground focus:ring-ring h-8 w-32 rounded-md border px-2 text-sm focus:ring-2 focus:outline-none"
					value={currentTimeRange}
					onchange={(e) => handleTimeRangeChange(e.currentTarget.value)}
				>
					{#each timeRangeOptions as option}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>
			</div>
		{/if}
	</div>
</div>
