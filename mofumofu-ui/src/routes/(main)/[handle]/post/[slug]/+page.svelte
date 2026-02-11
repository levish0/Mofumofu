<script lang="ts">
	import '$lib/styles/markdown.css';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { mode } from 'mode-watcher';
	import { toast } from 'svelte-sonner';
	import { incrementView, deletePost } from '$lib/api/posts';
	import { useNavbarScroll } from '$lib/hooks/useNavbarScroll.svelte';
	import {
		PostHeader,
		PostTOC,
		PostDeleteModal,
		FloatingTOC,
		FloatingNavigation
	} from '$lib/components/post/view';
	import type { TocItem } from '$lib/api/types';

	const post = $derived(page.data.post);
	const user = $derived(page.data.user);
	const isAuthenticated = $derived(!!user);
	const isAuthor = $derived(user?.handle === post.author.handle);

	const htmlContent = $derived(post.render ?? '');
	const tocItems = $derived((post.toc as TocItem[] | null) ?? []);

	const { isVisible } = useNavbarScroll();
	const topPosition = $derived(isVisible() ? '140px' : '80px');

	let isDeleteModalOpen = $state(false);
	let isDeleting = $state(false);

	function handleEdit() {
		goto(`/edit/${post.id}`);
	}

	function handleDelete() {
		isDeleteModalOpen = true;
	}

	function handleReport() {
		// TODO: report dialog
	}

	async function confirmDelete() {
		try {
			isDeleting = true;
			await deletePost(post.id);
			toast.success('Post deleted.');
			goto(`/@${post.author.handle}`);
		} catch (error) {
			console.error('Failed to delete post:', error);
			toast.error('Failed to delete post.');
		} finally {
			isDeleting = false;
		}
	}

	function cancelDelete() {
		isDeleteModalOpen = false;
	}

	function updateHighlightTheme(isDark: boolean) {
		if (typeof document === 'undefined') return;

		const existing = document.querySelector('link[data-highlight-theme]');
		if (existing) existing.remove();

		const link = document.createElement('link');
		link.rel = 'stylesheet';
		link.href = isDark
			? 'https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/styles/night-owl.css'
			: 'https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/styles/atom-one-light.css';
		link.setAttribute('data-highlight-theme', isDark ? 'dark' : 'light');
		document.head.appendChild(link);
	}

	$effect(() => {
		updateHighlightTheme(mode.current === 'dark');
	});

	onMount(() => {
		incrementView(post.id).catch((err) => {
			console.warn('Failed to increment view count:', err);
		});

		updateHighlightTheme(mode.current === 'dark');

		try {
			((window as any).adsbygoogle = (window as any).adsbygoogle || []).push({});
		} catch {
			// AdSense not loaded
		}
	});
</script>

<svelte:head>
	<title>{post.title} - {post.author.display_name} - Mofumofu</title>
	<meta name="description" content={post.summary || post.title} />

	<meta property="og:title" content="{post.title} - {post.author.display_name}" />
	<meta property="og:description" content={post.summary || post.title} />
	<meta property="og:type" content="article" />
	<meta property="og:url" content="https://mofumofu.ink/@{post.author.handle}/post/{post.slug}" />
	<meta
		property="og:image"
		content={post.thumbnail_image || 'https://mofumofu.ink/og-default.png'}
	/>
	<meta property="og:site_name" content="Mofumofu" />
	<meta property="article:author" content={post.author.display_name} />
	<meta property="article:published_time" content={post.created_at} />
	{#each post.hashtags as tag}
		<meta property="article:tag" content={tag} />
	{/each}

	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="{post.title} - {post.author.display_name}" />
	<meta name="twitter:description" content={post.summary || post.title} />
	<meta
		name="twitter:image"
		content={post.thumbnail_image || 'https://mofumofu.ink/og-default.png'}
	/>
</svelte:head>

<div class="min-h-screen pt-2">
	<div class="mx-auto grid max-w-8xl grid-cols-1 px-4 xl:grid-cols-[minmax(0,240px)_minmax(0,56rem)_minmax(0,240px)]">
		<!-- Left column: ad space (xl+ only) -->
		<div class="hidden xl:block" style="contain: layout;">
			<div class="sticky overflow-hidden pt-4 pr-8" style="top: {topPosition}; max-height: calc(100vh - {topPosition});">
				<ins
					class="adsbygoogle"
					style="display:block"
					data-ad-client="ca-pub-7400482974214530"
					data-ad-slot="9983314415"
					data-ad-format="rectangle"
				></ins>
			</div>
		</div>

		<!-- Center column: post content -->
		<div class="mx-auto w-full max-w-4xl pb-40 xl:mx-0 xl:max-w-none">
			<article>
				<PostHeader
					{post}
					{isAuthor}
					{isAuthenticated}
					onEdit={handleEdit}
					onDelete={handleDelete}
					onReport={handleReport}
				/>

				<div
					class="prose prose-lg dark:prose-invert mb-12 max-w-none wrap-break-word break-keep text-mofu-light-200 dark:text-mofu-dark-200"
				>
					{@html htmlContent}
				</div>
			</article>

			<!-- TODO: Comments Section -->
		</div>

		<!-- Right column: TOC (xl+ only) -->
		<PostTOC {tocItems} {topPosition} />
	</div>
</div>

<!-- Mobile floating components -->
<FloatingTOC {tocItems} />
<FloatingNavigation />

<PostDeleteModal
	bind:isOpen={isDeleteModalOpen}
	{isDeleting}
	onConfirm={confirmDelete}
	onCancel={cancelDelete}
/>
