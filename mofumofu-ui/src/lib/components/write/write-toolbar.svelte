<script lang="ts">
	import {
		Heading1,
		Heading2,
		Heading3,
		Heading4,
		Bold,
		Italic,
		Strikethrough,
		Quote,
		Link,
		Image,
		Code,
		Sigma,
		ChevronUp,
		ChevronDown
	} from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { Switch } from '$lib/components/ui/switch';
	import { uploadPostImage } from '$lib/api/posts/postsApi';
	import { toast } from 'svelte-sonner';

	interface Props {
		onInsertText: (before: string, after?: string) => void;
		showStickyToolbar: boolean;
		onToggleHeader: () => void;
		isPreviewMode?: boolean;
		onTogglePreviewMode?: (isPreview: boolean) => void;
	}

	const {
		onInsertText,
		showStickyToolbar,
		onToggleHeader,
		isPreviewMode = false,
		onTogglePreviewMode
	}: Props = $props();

	const btnClass = 'hover:bg-accent text-muted-foreground hover:text-foreground rounded p-2';

	async function handleImageUpload() {
		const input = document.createElement('input');
		input.type = 'file';
		input.accept = 'image/*';

		input.onchange = async (e) => {
			const file = (e.target as HTMLInputElement).files?.[0];
			if (!file) return;

			const fileSizeMB = file.size / (1024 * 1024);
			if (fileSizeMB > 8) {
				toast.error(`File size (${fileSizeMB.toFixed(2)}MB) exceeds 8MB limit.`);
				return;
			}

			const toastId = toast.loading('Uploading image...');

			try {
				const response = await uploadPostImage(file);
				toast.dismiss(toastId);
				toast.success('Image uploaded.');
				onInsertText(`![${file.name}](${response.image_url})`);
			} catch (error) {
				toast.dismiss(toastId);
				toast.error('Image upload failed.');
				console.error('Image upload failed:', error);
			}
		};

		input.click();
	}
</script>

<div class="px-4 pb-4">
	<div class="flex flex-wrap items-center justify-between gap-2">
		<div class="flex flex-wrap items-center gap-1">
			<Button variant="ghost" onclick={() => onInsertText('# ')} class={btnClass} title="Heading 1">
				<Heading1 class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('## ')}
				class={btnClass}
				title="Heading 2"
			>
				<Heading2 class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('### ')}
				class={btnClass}
				title="Heading 3"
			>
				<Heading3 class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('#### ')}
				class={btnClass}
				title="Heading 4"
			>
				<Heading4 class="h-5 w-5" />
			</Button>

			<div class="mx-1 h-6 w-px bg-border"></div>

			<Button
				variant="ghost"
				onclick={() => onInsertText('**', '**')}
				class={btnClass}
				title="Bold"
			>
				<Bold class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('*', '*')}
				class={btnClass}
				title="Italic"
			>
				<Italic class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('~~', '~~')}
				class={btnClass}
				title="Strikethrough"
			>
				<Strikethrough class="h-5 w-5" />
			</Button>

			<div class="mx-1 h-6 w-px bg-border"></div>

			<Button variant="ghost" onclick={() => onInsertText('> ')} class={btnClass} title="Quote">
				<Quote class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('[', '](url)')}
				class={btnClass}
				title="Link"
			>
				<Link class="h-5 w-5" />
			</Button>
			<Button variant="ghost" onclick={handleImageUpload} class={btnClass} title="Image">
				<Image class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('$$\n', '\n$$')}
				class={btnClass}
				title="Math"
			>
				<Sigma class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('```\n', '\n```')}
				class={btnClass}
				title="Code"
			>
				<Code class="h-5 w-5" />
			</Button>
		</div>

		<div class="ml-auto flex items-center gap-3">
			{#if onTogglePreviewMode}
				<div class="flex items-center gap-2 lg:hidden">
					<span class="text-sm text-muted-foreground">Editor</span>
					<Switch checked={isPreviewMode} onCheckedChange={onTogglePreviewMode} />
					<span class="text-sm text-muted-foreground">Preview</span>
				</div>
			{/if}

			<Button
				variant="ghost"
				onclick={onToggleHeader}
				class={btnClass}
				title={showStickyToolbar ? 'Show header' : 'Hide header'}
			>
				{#if showStickyToolbar}
					<ChevronDown class="h-5 w-5" />
				{:else}
					<ChevronUp class="h-5 w-5" />
				{/if}
			</Button>
		</div>
	</div>
</div>
