<script lang="ts">
	import { Icon, ClipboardDocumentList } from 'svelte-hero-icons';
	import { Button } from '$lib/components/ui/button';
	import { ArrowLeft } from '@lucide/svelte';
	import { PublishDialog } from './publish-dialog';

	interface Props {
		title: string;
		content: string;
		tags: string[];
		onExit: () => void;
		onSaveDraft: () => void;
		onPublished: () => void;
		isEditMode?: boolean;
		editPostId?: string;
		summary?: string;
		existingThumbnail?: string | null;
	}

	const {
		title,
		content,
		tags,
		onExit,
		onSaveDraft,
		onPublished,
		isEditMode = false,
		editPostId,
		summary,
		existingThumbnail
	}: Props = $props();
</script>

<div class="border-t border-border p-4">
	<div class="flex items-center justify-between">
		<Button variant="ghost" onclick={onExit} class="flex items-center gap-2 text-lg">
			<ArrowLeft class="h-5 w-5" />
			Back
		</Button>

		<div class="flex items-center gap-3">
			{#if !isEditMode}
				<Button variant="ghost" onclick={onSaveDraft} class="flex items-center gap-2 text-lg">
					<Icon src={ClipboardDocumentList} class="h-5 w-5" solid />
					Save Draft
				</Button>
			{/if}

			<PublishDialog
				{title}
				{content}
				{tags}
				{onPublished}
				{isEditMode}
				{editPostId}
				{summary}
				{existingThumbnail}
			/>
		</div>
	</div>
</div>
