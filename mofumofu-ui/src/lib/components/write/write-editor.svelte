<script lang="ts">
	import WriteHeader from './write-header.svelte';
	import WriteToolbar from './write-toolbar.svelte';
	import WriteActions from './write-actions.svelte';

	interface Props {
		title: string;
		tags: string[];
		content: string;
		onTitleChange: (value: string) => void;
		onTagsChange: (value: string[]) => void;
		onContentChange: (value: string) => void;
		onExit: () => void;
		onSaveDraft: () => void;
		onPublished: () => void;
		isEditMode?: boolean;
		editPostId?: string;
		isPreviewMode?: boolean;
		onTogglePreviewMode?: (isPreview: boolean) => void;
		htmlOutput?: string;
		summary?: string;
		existingThumbnail?: string | null;
	}

	const {
		title,
		tags,
		content,
		onTitleChange,
		onTagsChange,
		onContentChange,
		onExit,
		onSaveDraft,
		onPublished,
		isEditMode = false,
		editPostId,
		isPreviewMode = false,
		onTogglePreviewMode,
		htmlOutput = '',
		summary,
		existingThumbnail
	}: Props = $props();

	let contentTextarea: HTMLTextAreaElement | undefined = $state();
	let showStickyToolbar = $state(false);

	function insertText(before: string, after: string = '') {
		if (!contentTextarea) return;

		const start = contentTextarea.selectionStart;
		const end = contentTextarea.selectionEnd;
		const selectedText = content.substring(start, end);
		const newText =
			content.substring(0, start) + before + selectedText + after + content.substring(end);

		onContentChange(newText);

		setTimeout(() => {
			if (contentTextarea) {
				contentTextarea.focus();
				contentTextarea.setSelectionRange(
					start + before.length,
					start + before.length + selectedText.length
				);
			}
		}, 0);
	}
</script>

<div class="flex h-full flex-col overflow-hidden bg-background text-foreground">
	<!-- Header area (sticky) -->
	<div class="sticky top-0 z-10 overflow-hidden bg-background">
		<WriteHeader {title} {tags} {showStickyToolbar} {onTitleChange} {onTagsChange} />

		<WriteToolbar
			onInsertText={insertText}
			{showStickyToolbar}
			onToggleHeader={() => (showStickyToolbar = !showStickyToolbar)}
			{isPreviewMode}
			{onTogglePreviewMode}
		/>
	</div>

	<!-- Content area -->
	<div class="flex min-h-0 flex-1 flex-col overflow-hidden">
		{#if isPreviewMode && onTogglePreviewMode}
			<!-- Mobile preview mode -->
			<div class="min-h-0 flex-1 overflow-y-auto px-6 py-4">
				<div class="prose prose-lg dark:prose-invert max-w-none break-all">
					{@html htmlOutput}
				</div>
			</div>
		{:else}
			<textarea
				bind:this={contentTextarea}
				value={content}
				oninput={(e) => onContentChange((e.target as HTMLTextAreaElement).value)}
				placeholder="Write your story..."
				class="w-full flex-1 resize-none border-none bg-transparent px-6 py-0 text-lg leading-relaxed outline-none placeholder:text-muted-foreground"
				spellcheck="false"
			></textarea>
		{/if}
	</div>

	<!-- Action buttons -->
	<WriteActions
		{title}
		{content}
		{tags}
		{onExit}
		{onSaveDraft}
		{onPublished}
		{isEditMode}
		{editPostId}
		{summary}
		{existingThumbnail}
	/>
</div>
