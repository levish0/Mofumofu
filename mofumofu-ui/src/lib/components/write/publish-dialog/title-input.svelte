<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import * as v from 'valibot';
	import { createPostSchema } from '$lib/schemas/post';

	interface Props {
		value: string;
		onUpdate: (value: string) => void;
		onValidationChange: (error?: string) => void;
	}

	let { value, onUpdate, onValidationChange }: Props = $props();

	let validationError = $state('');
	const titleCount = $derived(value.length);

	function validate(titleValue: string) {
		const schema = createPostSchema();
		const result = v.safeParse(schema, {
			title: titleValue,
			content: 'temp',
			slug: 'temp'
		});

		if (result.success) {
			validationError = '';
			onValidationChange();
		} else {
			const err = result.issues.find((i) => i.path?.[0]?.key === 'title');
			if (err) {
				validationError = err.message;
				onValidationChange(err.message);
			}
		}
	}

	function handleInput(e: Event) {
		const newValue = (e.target as HTMLInputElement).value;
		onUpdate(newValue);
		validate(newValue);
	}
</script>

<div>
	<label for="publish-title" class="mb-2 block text-sm font-medium">Title</label>
	<div class="relative">
		<Input
			id="publish-title"
			{value}
			oninput={handleInput}
			placeholder="Post title"
			class="pr-12 {validationError ? 'border-destructive' : ''}"
		/>
		<div
			class="absolute top-1/2 right-3 -translate-y-1/2 text-xs {validationError
				? 'text-destructive'
				: 'text-muted-foreground'}"
		>
			{titleCount}/80
		</div>
	</div>
	{#if validationError}
		<p class="mt-1 text-xs text-destructive">{validationError}</p>
	{/if}
</div>
