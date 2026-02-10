<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { toast } from 'svelte-sonner';
	import { WriteEditor, WritePreview } from '$lib/components/write';
	import { processMarkdown } from '$lib/utils/markdown';
	import { useResizable } from '$lib/hooks/use-resizable.svelte';
	import { createDraft, getDraft, updateDraft } from '$lib/api/drafts/draftsApi';

	const user = $derived(page.data.user);

	// Redirect if not authenticated
	$effect(() => {
		if (!user) {
			goto('/account/signin');
		}
	});

	// Editor state
	let title = $state('');
	let tags = $state<string[]>([]);
	let content = $state('');
	let htmlOutput = $state('');
	let isPreviewMode = $state(false);

	// Draft state
	let draftId = $state<string | null>(null);
	let autoSaveTimer: ReturnType<typeof setInterval> | null = null;
	let lastSavedContent = '';
	let lastSavedTitle = '';

	// Resizable split-pane
	const { leftRatio, isDragging, handleMouseDown } = useResizable({
		initialRatio: 0.5,
		minRatio: 0.3,
		maxRatio: 0.7
	});

	// Process markdown on content change
	$effect(() => {
		if (content.trim()) {
			processMarkdown(content).then((result) => {
				htmlOutput = result.htmlContent;
			});
		} else {
			htmlOutput = '';
		}
	});

	// Load draft from URL param
	$effect(() => {
		const urlDraftId = page.url.searchParams.get('draft');
		if (urlDraftId && !draftId) {
			loadDraft(urlDraftId);
		}
	});

	async function loadDraft(id: string) {
		try {
			const draft = await getDraft(id);
			draftId = draft.id;
			title = draft.title ?? '';
			content = draft.content ?? '';
			lastSavedTitle = title;
			lastSavedContent = content;
		} catch (error) {
			console.error('Failed to load draft:', error);
			toast.error('Failed to load draft.');
		}
	}

	// Auto-save every 5 minutes
	$effect(() => {
		if (!user) return;

		autoSaveTimer = setInterval(
			() => {
				if (hasUnsavedChanges()) {
					saveDraft();
				}
			},
			5 * 60 * 1000
		);

		return () => {
			if (autoSaveTimer) clearInterval(autoSaveTimer);
		};
	});

	function hasUnsavedChanges(): boolean {
		return (
			(title.trim() !== '' || content.trim() !== '') &&
			(title !== lastSavedTitle || content !== lastSavedContent)
		);
	}

	async function saveDraft() {
		if (!title.trim() && !content.trim()) return;

		try {
			if (draftId) {
				await updateDraft(draftId, {
					title: title.trim() || null,
					content: content.trim() || null
				});
			} else {
				const draft = await createDraft();
				draftId = draft.id;
				await updateDraft(draftId, {
					title: title.trim() || null,
					content: content.trim() || null
				});
			}
			lastSavedTitle = title;
			lastSavedContent = content;
			toast.success('Draft saved.');
		} catch (error) {
			console.error('Failed to save draft:', error);
			toast.error('Failed to save draft.');
		}
	}

	function handleExit() {
		if (hasUnsavedChanges()) {
			saveDraft();
		}
		history.back();
	}

	function handlePublished() {
		// Delete draft after publishing if one exists
		// (handled silently, don't block the redirect)
	}
</script>

{#if user}
	<div id="resizable-container" class="flex h-full">
		<!-- Editor (left pane on desktop, full on mobile) -->
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
				onSaveDraft={saveDraft}
				onPublished={handlePublished}
				{isPreviewMode}
				onTogglePreviewMode={(v) => (isPreviewMode = v)}
				{htmlOutput}
			/>
		</div>

		<!-- Resize handle (desktop only) -->
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

		<!-- Preview (right pane on desktop, full on mobile when toggled) -->
		<div
			class="hidden min-h-0 flex-1 overflow-y-auto lg:block"
			class:block={isPreviewMode}
			class:lg:block={true}
		>
			<WritePreview {title} {htmlOutput} />
		</div>
	</div>
{/if}
