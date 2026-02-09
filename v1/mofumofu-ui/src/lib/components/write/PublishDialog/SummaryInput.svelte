<script lang="ts">
	import { Textarea } from '$lib/components/ui/textarea';
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
	const summaryCount = $derived(value.length);

	function validateSummary(summaryValue: string) {
		const schema = createPostSchema();
		const dataToValidate = {
			title: 'temp',
			content: 'temp',
			slug: 'temp',
			summary: summaryValue,
			tags: ''
		};

		const result = v.safeParse(schema, dataToValidate);

		if (result.success) {
			validationError = '';
			onValidationChange();
		} else {
			const summaryError = result.issues.find((issue) => issue.path?.[0]?.key === 'summary');
			if (summaryError) {
				validationError = summaryError.message;
				onValidationChange(summaryError.message);
			}
		}
	}

	function handleInput() {
		onUpdate(value);
		validateSummary(value);
	}
</script>

<div>
	<label for="summary" class="text-mofu-light-100 dark:text-mofu-dark-100 mb-2 block text-sm font-medium"
		>{m.publish_summary_label()}</label
	>
	<div class="relative">
		<Textarea
			id="summary"
			bind:value
			oninput={handleInput}
			placeholder="포스트 요약을 입력하세요"
			class="dark:bg-mofu-dark-700 bg-mofu-light-700 border-mofu-light-600  dark:border-mofu-dark-600 placeholder:text-mofu-light-400 dark:placeholder:text-mofu-dark-400 min-h-[80px] text-white {validationError
				? 'border-rose-500'
				: ''}"
		/>
		<div class="absolute right-2 bottom-2 text-xs {validationError ? 'text-rose-500' : 'text-mofu-dark-400'}">
			{summaryCount}/500
		</div>
	</div>
	{#if validationError}
		<p class="mt-1 text-xs text-rose-500">{validationError}</p>
	{/if}
</div>
