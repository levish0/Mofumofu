<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import * as v from 'valibot';
	import { createPostSchema } from '$lib/schemas/post';
	import { page } from '$app/state';

	interface Props {
		value: string;
		onUpdate: (value: string) => void;
		onValidationChange: (error?: string) => void;
	}

	let { value, onUpdate, onValidationChange }: Props = $props();

	let validationError = $state('');
	const slugCount = $derived(value.length);
	const handle = $derived(page.data.user?.handle ?? 'handle');

	function generateSlug(text: string): string {
		return text
			.trim()
			.replace(/[\s\/\?#\[\]@!$&'()*+,;=]+/g, '-')
			.replace(/-+/g, '-')
			.replace(/^-+|-+$/g, '');
	}

	function validate(slugValue: string) {
		const schema = createPostSchema();
		const result = v.safeParse(schema, {
			title: 'temp',
			content: 'temp',
			slug: slugValue
		});

		if (result.success) {
			validationError = '';
			onValidationChange();
		} else {
			const err = result.issues.find((i) => i.path?.[0]?.key === 'slug');
			if (err) {
				validationError = err.message;
				onValidationChange(err.message);
			}
		}
	}

	function handleInput(e: Event) {
		const raw = (e.target as HTMLInputElement).value;
		const newValue = generateSlug(raw);
		onUpdate(newValue);
		validate(newValue);
	}
</script>

<div>
	<label for="publish-slug" class="mb-2 block text-sm font-medium">URL Slug</label>
	<div class="relative">
		<Input
			id="publish-slug"
			{value}
			oninput={handleInput}
			placeholder="url-slug"
			class="pr-12 {validationError ? 'border-destructive' : ''}"
		/>
		<div
			class="absolute top-1/2 right-3 -translate-y-1/2 text-xs {validationError
				? 'text-destructive'
				: 'text-muted-foreground'}"
		>
			{slugCount}/80
		</div>
	</div>
	{#if validationError}
		<p class="mt-1 text-xs text-destructive">{validationError}</p>
	{/if}
	<p class="mt-1 text-xs text-muted-foreground">
		@{handle}/{value || 'slug'}
	</p>
</div>
