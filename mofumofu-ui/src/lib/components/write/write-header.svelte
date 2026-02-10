<script lang="ts">
	import { TagsInput } from '$lib/components/ui/tags-input';

	interface Props {
		title: string;
		tags: string[];
		showStickyToolbar: boolean;
		onTitleChange: (value: string) => void;
		onTagsChange: (value: string[]) => void;
	}

	const { title, tags, showStickyToolbar, onTitleChange, onTagsChange }: Props = $props();

	const titleCount = $derived(title.length);
	const titleOverLimit = $derived(titleCount > 80);

	function validateTag(val: string, existing: string[]): string | undefined {
		const trimmed = val.trim();
		if (!trimmed) return undefined;
		if (existing.includes(trimmed)) return undefined;
		if (existing.length >= 8) return undefined;
		return trimmed;
	}
</script>

<div
	class="overflow-hidden pt-4 break-all transition-all duration-400 ease-in-out"
	style="max-height: {showStickyToolbar ? '0px' : '480px'}; opacity: {showStickyToolbar ? '0' : '1'};"
>
	<!-- Title -->
	<div class="mb-6 px-6">
		<div class="relative">
			<input
				value={title}
				oninput={(e) => onTitleChange((e.target as HTMLInputElement).value)}
				placeholder="Title"
				class="h-auto w-full border-none bg-transparent p-0 text-4xl font-bold outline-none placeholder:text-muted-foreground {titleOverLimit
					? 'text-destructive'
					: ''}"
				style="font-size: 2.5rem; line-height: 1.2;"
			/>
			<div
				class="absolute right-0 -bottom-6 text-sm {titleOverLimit
					? 'text-destructive'
					: 'text-muted-foreground'}"
			>
				{titleCount}/80
			</div>
		</div>
	</div>

	<!-- Tags -->
	<div class="mb-4 px-6">
		<TagsInput
			value={tags}
			onValueChange={onTagsChange}
			validate={validateTag}
			placeholder={tags.length < 8 ? 'Add tag...' : ''}
			disabled={tags.length >= 8}
			class="border-none bg-transparent px-0"
		/>
		<div class="mt-1 text-xs {tags.length >= 8 ? 'text-destructive' : 'text-muted-foreground'}">
			Tags: {tags.length}/8
		</div>
	</div>
</div>
