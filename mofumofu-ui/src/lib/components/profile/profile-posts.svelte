<script lang="ts">
	import type { PostSearchItem } from '$lib/api/types';
	import { Badge } from '$lib/components/ui/badge';
	import { PostCard } from '$lib/components/post';
	import { useNavbarScroll } from '$lib/hooks/useNavbarScroll.svelte';

	interface Props {
		posts: PostSearchItem[];
		profileName: string;
	}

	const { posts, profileName }: Props = $props();

	const { isVisible } = useNavbarScroll();
	const stickyTop = $derived(isVisible() ? '60px' : '0px');

	let selectedTags = $state<string[]>([]);

	const availableHashtags = $derived(() => {
		const counts = new Map<string, number>();
		for (const post of posts) {
			for (const tag of post.hashtags) {
				const t = tag.trim();
				if (t) counts.set(t, (counts.get(t) || 0) + 1);
			}
		}
		return Array.from(counts.entries())
			.map(([tag, count]) => ({ tag, count }))
			.sort((a, b) => b.count - a.count);
	});

	const filteredPosts = $derived(
		selectedTags.length === 0
			? posts
			: posts.filter((post) =>
					selectedTags.some((st) =>
						post.hashtags.some((pt) => pt.toLowerCase().includes(st.toLowerCase()))
					)
				)
	);

	function toggleTag(tag: string) {
		if (selectedTags.includes(tag)) {
			selectedTags = selectedTags.filter((t) => t !== tag);
		} else {
			selectedTags = [...selectedTags, tag];
		}
	}
</script>

<div class="space-y-2">
	<!-- Sticky hashtag filter -->
	<div
		class="sticky z-20 bg-mofu-light-900 pt-2 pb-2 transition-all duration-100 ease-out dark:bg-mofu-dark-900"
		style="top: {stickyTop}"
	>
		{#if availableHashtags().length > 0}
			<div class="flex flex-wrap gap-2">
				{#each availableHashtags() as { tag, count }}
					<Badge
						variant="secondary"
						class="cursor-pointer transition-colors {selectedTags.includes(tag)
							? 'bg-primary text-primary-foreground'
							: ''}"
						onclick={() => toggleTag(tag)}
					>
						#{tag}({count})
					</Badge>
				{/each}
			</div>
		{:else}
			<div class="text-sm text-mofu-light-400 dark:text-mofu-dark-400">No tags to filter.</div>
		{/if}
	</div>

	<!-- Posts grid -->
	<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
		{#each filteredPosts as post (post.id)}
			<PostCard {post} />
		{/each}
	</div>

	<!-- Empty state -->
	{#if filteredPosts.length === 0}
		<div class="flex flex-col items-center justify-center py-12 text-center">
			{#if selectedTags.length > 0}
				<div class="mb-2 text-lg text-mofu-light-400 dark:text-mofu-dark-400">
					No posts matching selected tags.
				</div>
				<button
					onclick={() => (selectedTags = [])}
					class="text-sm text-primary underline hover:opacity-80"
				>
					Clear filters
				</button>
			{:else if posts.length === 0}
				<div class="mb-2 text-lg text-mofu-light-400 dark:text-mofu-dark-400">
					No posts yet.
				</div>
				<div class="text-sm text-mofu-light-500 dark:text-mofu-dark-500">
					{profileName} hasn't published any posts.
				</div>
			{/if}
		</div>
	{/if}
</div>
