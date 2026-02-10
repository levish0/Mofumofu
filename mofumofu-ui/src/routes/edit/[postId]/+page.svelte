<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { toast } from 'svelte-sonner';
	import { WriteEditor, WritePreview } from '$lib/components/write';
	import { processMarkdown } from '$lib/utils/markdown';
	import { useResizable } from '$lib/hooks/use-resizable.svelte';

	const post = $derived(page.data.post);

	let title = $state(post.title);
	let tags = $state<string[]>(post.hashtags);
	let content = $state(post.content);
	let htmlOutput = $state('');
	let isPreviewMode = $state(false);

	const { leftRatio, isDragging, handleMouseDown } = useResizable({
		initialRatio: 0.5,
		minRatio: 0.3,
		maxRatio: 0.7
	});

	$effect(() => {
		if (content.trim()) {
			processMarkdown(content).then((result) => {
				htmlOutput = result.htmlContent;
			});
		} else {
			htmlOutput = '';
		}
	});

	function handleExit() {
		history.back();
	}

	function handlePublished() {
		toast.success('Post updated.');
		goto(`/@${post.author.handle}/post/${post.slug}`);
	}
</script>

<svelte:head>
	<title>Edit - {post.title} - Mofumofu</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div id="resizable-container" class="flex h-full">
	<div
		class="flex h-full w-full flex-col lg:w-auto"
		style="flex: 0 0 {leftRatio() * 100}%;"
		class:hidden={isPreviewMode}
		class:lg:flex={isPreviewMode}
	>
		<WriteEditor
			{title}
			{tags}
			{content}
			onTitleChange={(v) => (title = v)}
			onTagsChange={(v) => (tags = v)}
			onContentChange={(v) => (content = v)}
			onExit={handleExit}
			onSaveDraft={() => {}}
			onPublished={handlePublished}
			isEditMode={true}
			editPostId={post.id}
			{isPreviewMode}
			onTogglePreviewMode={(v) => (isPreviewMode = v)}
			{htmlOutput}
			summary={post.summary ?? undefined}
			existingThumbnail={post.thumbnail_image}
		/>
	</div>

	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="hidden w-1 cursor-col-resize bg-border transition-colors hover:bg-primary/50 lg:block {isDragging()
			? 'bg-primary'
			: ''}"
		onmousedown={handleMouseDown}
		role="separator"
		aria-orientation="vertical"
		tabindex="-1"
	></div>

	<div
		class="hidden min-h-0 flex-1 overflow-y-auto lg:block"
		class:block={isPreviewMode}
		class:lg:block={true}
	>
		<WritePreview {title} {htmlOutput} />
	</div>
</div>
