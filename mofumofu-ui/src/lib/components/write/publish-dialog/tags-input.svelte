<script lang="ts">
	import { TagsInput } from '$lib/components/ui/tags-input';

	interface Props {
		value: string[];
		onUpdate: (value: string[]) => void;
		onValidationChange: (error?: string) => void;
	}

	let { value, onUpdate, onValidationChange }: Props = $props();

	let validationError = $state('');

	function validateTag(val: string, existing: string[]): string | undefined {
		const trimmed = val.trim();
		if (!trimmed) return undefined;
		if (existing.includes(trimmed)) return undefined;
		if (existing.length >= 8) {
			validationError = 'You can add up to 8 tags.';
			onValidationChange(validationError);
			return undefined;
		}
		return trimmed;
	}

	function handleChange(newTags: string[]) {
		onUpdate(newTags);
		if (newTags.length > 8) {
			validationError = 'You can add up to 8 tags.';
			onValidationChange(validationError);
		} else {
			validationError = '';
			onValidationChange();
		}
	}
</script>

<div>
	<!-- svelte-ignore a11y_label_has_associated_control -->
	<label class="mb-2 block text-sm font-medium">Tags</label>
	<TagsInput
		{value}
		onValueChange={handleChange}
		validate={validateTag}
		placeholder={value.length < 8 ? 'Add tag and press Enter...' : ''}
	/>
	<div class="mt-1 flex items-center justify-between">
		{#if validationError}
			<p class="text-xs text-destructive">{validationError}</p>
		{:else}
			<p class="text-xs text-muted-foreground">Click a tag to remove it.</p>
		{/if}
		<span class="text-xs {value.length >= 8 ? 'text-destructive' : 'text-muted-foreground'}">
			{value.length}/8
		</span>
	</div>
</div>
