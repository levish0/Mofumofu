<script lang="ts">
	import WriteHeader from './WriteHeader.svelte';
	import WriteToolbar from './WriteToolbar.svelte';
	import WriteActions from './WriteActions.svelte';
	import * as m from '../../../paraglide/messages';

	interface Props {
		title: string;
		tags: string;
		content: string;
		onTitleChange: (value: string) => void;
		onTagsChange: (value: string) => void;
		onContentChange: (value: string) => void;
		onExit: () => void;
		onSaveDraft: () => void;
		onPublished: () => void;
		isEditMode?: boolean;
		editSlug?: string;
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
		editSlug,
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
		const newText = content.substring(0, start) + before + selectedText + after + content.substring(end);

		onContentChange(newText);

		setTimeout(() => {
			if (contentTextarea) {
				contentTextarea.focus();
				contentTextarea.setSelectionRange(start + before.length, start + before.length + selectedText.length);
			}
		}, 0);
	}
</script>

<div
	class="dark:bg-mofu-dark-900 bg-mofu-light-900 text-mofu-light-200 dark:text-mofu-dark-200 flex h-full flex-col overflow-hidden"
>
	<!-- 헤더 영역 (sticky) -->
	<div class="bg-mofu-dark-900 sticky top-0 z-10 overflow-hidden">
		<!-- 제목/태그 영역 -->
		<WriteHeader {title} {tags} {showStickyToolbar} {onTitleChange} {onTagsChange} />

		<!-- 툴바 (항상 보임) -->
		<WriteToolbar
			onInsertText={insertText}
			{showStickyToolbar}
			onToggleHeader={() => (showStickyToolbar = !showStickyToolbar)}
			{isPreviewMode}
			{onTogglePreviewMode}
		/>
	</div>

	<!-- 본문 입력 또는 프리뷰 -->
	<div class="flex min-h-0 flex-1 flex-col overflow-hidden">
		{#if isPreviewMode && onTogglePreviewMode}
			<!-- 모바일 프리뷰 모드 -->
			<div class="min-h-0 flex-1 overflow-y-auto px-6 py-4">
				<div class="prose prose-invert prose-lg text-mofu-dark-200 max-w-none break-all">
					{@html htmlOutput}
				</div>
			</div>
		{:else}
			<!-- 에디터 모드 -->
			<textarea
				bind:this={contentTextarea}
				value={content}
				oninput={(e) => onContentChange((e.target as HTMLInputElement).value)}
				placeholder={m.write_editor_placeholder()}
				class="placeholder:text-mofu-dark-600 w-full flex-1 resize-none border-none bg-transparent px-6 py-0 text-lg leading-relaxed outline-none"
				spellcheck="false"
			></textarea>
		{/if}
	</div>

	<!-- 에디터 영역 하단 버튼들 -->
	<WriteActions
		{title}
		{content}
		{tags}
		{onExit}
		{onSaveDraft}
		{onPublished}
		{isEditMode}
		{editSlug}
		{editPostId}
		{summary}
		{existingThumbnail}
	/>
</div>
