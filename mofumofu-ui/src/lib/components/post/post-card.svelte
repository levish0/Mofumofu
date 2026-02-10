<script lang="ts">
	import type { PostSearchItem } from '$lib/api/types';
	import PostCardImage from './post-card-image.svelte';
	import PostCardContent from './post-card-content.svelte';
	import PostCardFooter from './post-card-footer.svelte';
	import PostCardSkeleton from './post-card-skeleton.svelte';

	let { post }: { post?: PostSearchItem } = $props();
</script>

{#if !post}
	<PostCardSkeleton />
{:else}
	<a
		href="/@{post.author_handle}/post/{post.slug}"
		class="flex flex-col overflow-hidden rounded-xl bg-mofu-light-800 transition-transform duration-200 hover:-translate-y-1 dark:bg-mofu-dark-800"
	>
		<PostCardImage image={post.thumbnail_image} title={post.title} />
		<PostCardContent
			title={post.title}
			summary={post.summary}
			date={post.created_at}
			comments={post.comment_count}
			hashtags={post.hashtags}
		/>
		<div class="mx-3 border-t border-mofu-light-700 dark:border-mofu-dark-700"></div>
		<PostCardFooter
			authorHandle={post.author_handle}
			authorName={post.author_display_name}
			authorAvatar={post.author_profile_image}
			likes={post.like_count}
			views={post.view_count}
		/>
	</a>
{/if}
