<script lang="ts">
	import PostCardImage from './post-card-image.svelte';
	import PostCardContent from './post-card-content.svelte';
	import PostCardFooter from './post-card-footer.svelte';
</script>

<div class="flex flex-col overflow-hidden rounded-xl bg-mofu-light-800 dark:bg-mofu-dark-800">
	<PostCardImage image={null} title="" isSkeleton />
	<PostCardContent isSkeleton />
	<div class="mx-3 border-t border-mofu-light-700 dark:border-mofu-dark-700"></div>
	<PostCardFooter isSkeleton />
</div>
