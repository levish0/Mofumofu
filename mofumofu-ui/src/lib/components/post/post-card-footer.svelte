<script lang="ts">
	import { Icon, Eye, Heart } from 'svelte-hero-icons';

	let {
		authorHandle = '',
		authorName = '',
		authorAvatar,
		likes = 0,
		views = 0,
		isSkeleton = false
	}: {
		authorHandle?: string;
		authorName?: string;
		authorAvatar?: string | null;
		likes?: number;
		views?: number;
		isSkeleton?: boolean;
	} = $props();
</script>

<div class="flex items-center justify-between px-3 py-2">
	{#if isSkeleton}
		<div class="flex items-center gap-2">
			<div class="shimmer h-6 w-6 rounded-full"></div>
			<div class="shimmer h-3 w-20 rounded"></div>
		</div>
		<div class="flex items-center gap-3">
			<div class="shimmer h-3 w-10 rounded"></div>
			<div class="shimmer h-3 w-10 rounded"></div>
		</div>
	{:else}
		<a href="/@{authorHandle}/profile" class="flex items-center gap-2 overflow-hidden">
			<div class="h-6 w-6 shrink-0 overflow-hidden rounded-full">
				{#if authorAvatar}
					<img
						src={authorAvatar}
						alt="{authorName}'s avatar"
						class="h-full w-full object-cover"
					/>
				{:else}
					<span
						class="flex h-full w-full items-center justify-center bg-mofu-light-700 text-[10px] font-medium text-black dark:bg-mofu-dark-700 dark:text-white"
					>
						{authorHandle.charAt(0).toUpperCase()}
					</span>
				{/if}
			</div>
			<span class="truncate text-xs text-mofu-light-400 dark:text-mofu-dark-300">
				by {authorName}
			</span>
		</a>

		<div class="flex items-center gap-3 text-xs text-mofu-light-500 dark:text-mofu-dark-400">
			<span class="flex items-center gap-1">
				<Icon src={Eye} size="14" />
				{views}
			</span>
			<span class="flex items-center gap-1">
				<Icon src={Heart} size="14" />
				{likes}
			</span>
		</div>
	{/if}
</div>
