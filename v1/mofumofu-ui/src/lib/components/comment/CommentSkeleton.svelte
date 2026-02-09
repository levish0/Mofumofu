<script lang="ts">
	import Self from './CommentSkeleton.svelte';

	interface Props {
		depth?: number;
		showReplies?: boolean;
		replyCount?: number;
	}

	let { depth = 0, showReplies = false, replyCount = 2 }: Props = $props();
</script>

<!-- Comment Skeleton -->
<div class="relative py-1.5 pl-4" class:-ml-4={depth === 0} class:ml-4={depth > 0}>
	<!-- Thread line for nested comments -->
	{#if depth > 0}
		<div class="border-mofu-light-300 dark:border-mofu-dark-600 absolute top-0 left-2 h-full w-px border-l"></div>
	{/if}

	<!-- Comment main content -->
	<div
		class="hover:bg-mofu-light-900 dark:hover:bg-mofu-dark-800/50 flex items-start gap-3 rounded-lg p-2 pb-1.5 transition-colors"
	>
		<!-- Avatar skeleton -->
		<div class="shimmer h-8 w-8 rounded-full"></div>

		<!-- Comment content -->
		<div class="min-w-0 flex-1">
			<div class="flex items-start justify-between">
				<!-- User info and timestamp skeleton -->
				<div class="flex items-center gap-2">
					<div class="shimmer h-4 w-20 rounded"></div>
					<div class="shimmer h-3 w-16 rounded"></div>
				</div>

				<!-- Actions menu skeleton -->
				<div class="flex items-center gap-1">
					<!-- Like button skeleton -->
					<div class="flex items-center gap-2 px-4 py-2">
						<div class="shimmer h-4 w-4 rounded"></div>
						<div class="shimmer h-3 w-6 rounded"></div>
					</div>
					<!-- Menu button skeleton -->
					<div class="shimmer h-6 w-6 rounded"></div>
				</div>
			</div>

			<!-- Comment text skeleton -->
			<div class="mt-2 space-y-2">
				<div class="shimmer h-3 w-full rounded"></div>
				<div class="shimmer h-3 w-4/5 rounded"></div>
				<div class="shimmer h-3 w-3/5 rounded"></div>
			</div>
		</div>
	</div>

	<!-- Replies skeleton -->
	{#if showReplies}
		{#each Array(replyCount) as _, i}
			<Self depth={depth + 1} />
		{/each}
	{/if}
</div>
