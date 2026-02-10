<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import PostLikeButton from './post-like-button.svelte';
	import PostActions from './post-actions.svelte';
	import type { PostResponse } from '$lib/api/types';

	interface Props {
		post: PostResponse;
		isAuthor: boolean;
		isAuthenticated: boolean;
		onEdit: () => void;
		onDelete: () => void;
		onReport: () => void;
	}

	const { post, isAuthor, isAuthenticated, onEdit, onDelete, onReport }: Props = $props();

	const formattedDate = $derived(
		new Date(post.created_at).toLocaleString(undefined, {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit',
			timeZoneName: 'short'
		})
	);
</script>

<header>
	<h1 class="mb-4 text-4xl font-bold break-all text-mofu-light-100 dark:text-mofu-dark-100">
		{post.title}
	</h1>

	<div class="mb-4 flex items-center justify-between">
		<a
			href="/@{post.author.handle}"
			class="flex items-center gap-4 transition-opacity hover:opacity-80"
		>
			{#if post.author.profile_image}
				<img
					src={post.author.profile_image}
					alt={post.author.display_name}
					class="h-12 w-12 rounded-full object-cover"
				/>
			{:else}
				<div
					class="flex h-12 w-12 items-center justify-center rounded-full bg-mofu-light-600 dark:bg-mofu-dark-600"
				>
					<span class="text-lg font-medium text-mofu-light-200 dark:text-mofu-dark-200">
						{post.author.display_name.charAt(0).toUpperCase()}
					</span>
				</div>
			{/if}
			<div>
				<p class="font-medium text-black dark:text-white">{post.author.display_name}</p>
				<p class="text-sm text-mofu-light-400 dark:text-mofu-dark-400">
					{formattedDate}
				</p>
			</div>
		</a>

		<div class="flex items-center gap-2">
			<PostLikeButton postId={post.id} initialLikeCount={post.like_count} {isAuthenticated} />
			<PostActions isOwner={isAuthor} {onEdit} {onDelete} {onReport} />
		</div>
	</div>

	{#if post.hashtags.length > 0}
		<div class="flex flex-wrap gap-2">
			{#each post.hashtags as tag}
				<Badge>#{tag}</Badge>
			{/each}
		</div>
	{/if}
</header>
