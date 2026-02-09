<script lang="ts">
	import { BookmarkSquare, PaperAirplane, Icon, ClipboardDocumentList } from 'svelte-hero-icons';
	import { Button } from '../ui/button';
	import { ArrowLeft, Save, Send } from '@lucide/svelte';
	import { PublishDialog } from './PublishDialog';
	import * as m from '../../../paraglide/messages';

	interface Props {
		title: string;
		content: string;
		tags: string;
		onExit: () => void;
		onSaveDraft: () => void;
		onPublished: () => void;
		isEditMode?: boolean;
		editSlug?: string;
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
		editSlug,
		editPostId,
		summary,
		existingThumbnail
	}: Props = $props();
</script>

<div class="dark:bg-mofu-dark-950 bg-mofu-light-950 p-4">
	<div class="flex items-center justify-between">
		<Button
			variant="ghost"
			onclick={onExit}
			class="dark:text-mofu-dark-200 flex items-center gap-2 rounded px-4 py-2 text-lg"
		>
			<ArrowLeft class="h-5 w-5" />
			{m.write_back()}
		</Button>

		<div class="flex items-center gap-3">
			{#if !isEditMode}
				<Button
					variant="ghost"
					onclick={onSaveDraft}
					class=" dark:text-mofu-dark-200 flex items-center gap-2 rounded px-4 py-2 text-lg"
				>
					<Icon src={ClipboardDocumentList} class="h-5 w-5" solid />
					{m.write_save_draft()}
				</Button>
			{/if}

			<PublishDialog
				{title}
				{content}
				{tags}
				{onPublished}
				{isEditMode}
				{editSlug}
				{editPostId}
				{summary}
				{existingThumbnail}
			/>
		</div>
	</div>
</div>
