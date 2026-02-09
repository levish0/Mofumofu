<script lang="ts">
	import '$lib/styles/markdown.css';
	import { getContext, onMount } from 'svelte';
	import { mode } from 'mode-watcher';
	import type { PageData } from './$types';
	import { incrementPostView, deletePost } from '$lib/api/post/postApi';
	import { goto } from '$app/navigation';
	import { toast } from 'svelte-sonner';
	import FloatingTOC from '$lib/components/post/floating/FloatingTOC.svelte';
	import FloatingNavigation from '$lib/components/post/floating/FloatingNavigation.svelte';
	import CommentList from '$lib/components/comment/CommentList.svelte';
	import PostHeader from '$lib/components/post/view/PostHeader.svelte';
	import PostTOC from '$lib/components/post/view/PostTOC.svelte';
	import PostDeleteModal from '$lib/components/post/view/PostDeleteModal.svelte';
	import * as m from '../../../../../paraglide/messages';

	const { data }: { data: PageData } = $props();

	// 백엔드에서 렌더된 HTML과 TOC 사용
	const htmlContent = data.post.rendered;
	const tocItems = data.post.toc_items;

	type NavbarContext = {
		isVisible: () => boolean;
		isAtTop: () => boolean;
	};

	const navbar = getContext<NavbarContext>('navbar');
	const topPosition = $derived(navbar.isVisible() ? '68px' : '8px');

	// 삭제 모달 상태
	let isDeleteModalOpen = $state(false);
	let isDeleting = $state(false);

	function handleEdit() {
		goto(`/edit/${data.slug}`);
	}

	function handleDelete() {
		isDeleteModalOpen = true;
	}

	async function confirmDelete() {
		try {
			isDeleting = true;
			await deletePost({ slug: data.slug });
			toast.success(m.post_delete_success());
			// 삭제 후 사용자 프로필 페이지로 이동
			goto(`/@${data.author.handle}/profile`);
		} catch (error) {
			console.error('Failed to delete post:', error);
			toast.error(m.post_delete_error());
		} finally {
			isDeleting = false;
		}
	}

	function cancelDelete() {
		isDeleteModalOpen = false;
	}

	function updateHighlightTheme(isDark: boolean) {
		if (typeof document === 'undefined') return;

		// Remove existing highlight.js theme
		const existingLink = document.querySelector('link[data-highlight-theme]');
		if (existingLink) {
			existingLink.remove();
		}

		// Add new theme
		const link = document.createElement('link');
		link.rel = 'stylesheet';
		link.href = isDark
			? 'https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/styles/night-owl.css'
			: 'https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/styles/atom-one-light.css';
		link.setAttribute('data-highlight-theme', isDark ? 'dark' : 'light');

		document.head.appendChild(link);
	}

	// Watch for theme changes
	$effect(() => {
		updateHighlightTheme(mode.current === 'dark');
	});

	onMount(() => {
		incrementPostView({ post_id: data.post.id }).catch((error) => {
			console.warn('Failed to increment view count:', error);
		});

		updateHighlightTheme(mode.current === 'dark');
	});
</script>

<svelte:head>
	<title>{data.post.title} - {data.author.name} - Mofumofu</title>
	<meta name="description" content={data.post.summary || data.post.title} />

	<!-- Open Graph -->
	<meta property="og:title" content="{data.post.title} - {data.author.name}" />
	<meta property="og:description" content={data.post.summary || data.post.title} />
	<meta property="og:type" content="article" />
	<meta property="og:url" content="https://mofumofu.ink/@{data.author.handle}/post/{data.post.slug}" />
	<meta property="og:image" content={data.post.thumbnail_image || 'https://mofumofu.ink/og-default.png'} />
	<meta property="og:site_name" content="Mofumofu" />
	<meta property="article:author" content={data.author.name} />
	<meta property="article:published_time" content={data.post.created_at} />
	{#each data.post.tags as tag}
		<meta property="article:tag" content={tag} />
	{/each}

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="{data.post.title} - {data.author.name}" />
	<meta name="twitter:description" content={data.post.summary || data.post.title} />
	<meta name="twitter:image" content={data.post.thumbnail_image || 'https://mofumofu.ink/og-default.png'} />
</svelte:head>

<div class="relative min-h-screen pt-2">
	<div class="max-w-8xl mx-auto gap-4 px-4">
		<div class="flex gap-4">
			<!-- Left Column: Post Content -->
			<div class=" min-w-0 flex-1 pb-40">
				<article class="min-h-screen">
					<PostHeader post={data.post} author={data.author} onEdit={handleEdit} onDelete={handleDelete} />

					<!-- Post Content -->
					<div
						class="prose prose-invert prose-lg dark:text-mofu-dark-200 text-mofu-light-200 mb-12 max-w-none wrap-break-word break-keep"
					>
						{@html htmlContent}
					</div>
				</article>

				<!-- Comments Section -->
				<section class="dark:border-mofu-dark-700 border-mofu-light-700 mt-16 border-t pt-8">
					<CommentList postId={data.post.id} perPage={5} replyPerPage={4} defaultSort="latest" />
				</section>
			</div>

			<!-- Right Column: TOC (데스크톱에서만 표시) -->
			<PostTOC {tocItems} {topPosition} />
		</div>
	</div>
</div>

<!-- 모바일용 플로팅 컴포넌트들 -->
<FloatingTOC {tocItems} />
<FloatingNavigation />

<PostDeleteModal bind:isOpen={isDeleteModalOpen} {isDeleting} onConfirm={confirmDelete} onCancel={cancelDelete} />
