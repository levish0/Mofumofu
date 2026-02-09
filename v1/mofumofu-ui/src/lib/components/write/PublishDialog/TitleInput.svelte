<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import * as v from 'valibot';
	import { createPostSchema } from '$lib/schemas/post';
	import * as m from '../../../../paraglide/messages';

	interface Props {
		value: string;
		onUpdate: (value: string) => void;
		onValidationChange: (error?: string) => void;
	}

	let { value, onUpdate, onValidationChange }: Props = $props();

	let validationError = $state<string>('');
	const titleCount = $derived(value.length);

	function validateTitle(titleValue: string) {
		const schema = createPostSchema();
		const dataToValidate = {
			title: titleValue,
			content: 'temp',
			slug: 'temp',
			summary: '',
			tags: ''
		};

		const result = v.safeParse(schema, dataToValidate);

		if (result.success) {
			validationError = '';
			onValidationChange();
		} else {
			const titleError = result.issues.find((issue) => issue.path?.[0]?.key === 'title');
			if (titleError) {
				validationError = titleError.message;
				onValidationChange(titleError.message);
			}
		}
	}

	function handleInput(e: Event) {
		const newValue = (e.target as HTMLInputElement).value;
		onUpdate(newValue);
		validateTitle(newValue);
	}
</script>

<div>
	<label for="title" class="text-mofu-light-100 dark:text-mofu-dark-100 mb-2 block text-sm font-medium"
		>{m.publish_title_label()}</label
	>
	<div class="relative">
		<Input
			id="title"
			{value}
			oninput={handleInput}
			placeholder={m.publish_title_placeholder()}
			class="dark:bg-mofu-dark-700 bg-mofu-light-700 border-mofu-light-600 dark:border-mofu-dark-600 placeholder:text-mofu-light-400 dark:placeholder:text-mofu-dark-400 pr-12  text-black dark:text-white {validationError
				? 'border-rose-500'
				: ''}"
		/>
		<div
			class="absolute top-1/2 right-3 -translate-y-1/2 text-xs {validationError
				? 'text-rose-500'
				: 'text-mofu-light-400 dark:text-mofu-dark-400'}"
		>
			{titleCount}/80
		</div>
	</div>
	{#if validationError}
		<p class="mt-1 text-xs text-rose-500">{validationError}</p>
	{/if}
</div>
