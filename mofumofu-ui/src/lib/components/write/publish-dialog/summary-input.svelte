<script lang="ts">
	import { Textarea } from '$lib/components/ui/textarea';

	interface Props {
		value: string;
		onUpdate: (value: string) => void;
		onValidationChange: (error?: string) => void;
	}

	let { value, onUpdate, onValidationChange }: Props = $props();

	let validationError = $state('');
	const summaryCount = $derived(value.length);

	function handleInput(e: Event) {
		const newValue = (e.target as HTMLTextAreaElement).value;
		onUpdate(newValue);

		if (newValue.length > 500) {
			validationError = 'Summary must be 500 characters or less.';
			onValidationChange(validationError);
		} else {
			validationError = '';
			onValidationChange();
		}
	}
</script>

<div>
	<label for="publish-summary" class="mb-2 block text-sm font-medium">Summary (optional)</label>
	<div class="relative">
		<Textarea
			id="publish-summary"
			{value}
			oninput={handleInput}
			placeholder="A brief summary of your post"
			class="min-h-[80px] {validationError ? 'border-destructive' : ''}"
		/>
		<div
			class="absolute right-2 bottom-2 text-xs {validationError
				? 'text-destructive'
				: 'text-muted-foreground'}"
		>
			{summaryCount}/500
		</div>
	</div>
	{#if validationError}
		<p class="mt-1 text-xs text-destructive">{validationError}</p>
	{/if}
</div>
