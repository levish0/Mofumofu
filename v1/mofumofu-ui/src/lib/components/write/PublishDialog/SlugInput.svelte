<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import * as v from 'valibot';
	import { createPostSchema } from '$lib/schemas/post';
	import { userStore } from '$lib/stores/user.svelte';
	import * as m from '../../../../paraglide/messages';

	interface Props {
		value: string;
		onUpdate: (value: string) => void;
		onValidationChange: (error?: string) => void;
	}

	let { value, onUpdate, onValidationChange }: Props = $props();

	let validationError = $state<string>('');
	const slugCount = $derived(value.length);

	function generateSlug(text: string): string {
		return (
			text
				.trim()
				// URL에 안전하지 않은 문자들만 제거 (공백, /, ?, #, [, ], @, !, $, &, ', (, ), *, +, ,, ;, =)
				.replace(/[\s\/\?#\[\]@!$&'()*+,;=]+/g, '-')
				// 연속된 하이픈을 하나로
				.replace(/-+/g, '-')
				// 앞뒤 하이픈 제거
				.replace(/^-+|-+$/g, '')
		);
	}

	function validateSlug(slugValue: string) {
		const schema = createPostSchema();
		const dataToValidate = {
			title: 'temp',
			content: 'temp',
			slug: slugValue,
			summary: '',
			tags: ''
		};

		const result = v.safeParse(schema, dataToValidate);

		if (result.success) {
			validationError = '';
			onValidationChange();
		} else {
			const slugError = result.issues.find((issue) => issue.path?.[0]?.key === 'slug');
			if (slugError) {
				validationError = slugError.message;
				onValidationChange(slugError.message);
			}
		}
	}

	function handleInput(e: Event) {
		const inputValue = (e.target as HTMLInputElement).value;
		const newValue = generateSlug(inputValue);
		onUpdate(newValue);
		validateSlug(newValue);
	}
</script>

<div>
	<label for="slug" class="dark:text-mofu-dark-100 text-mofu-light-100 mb-2 block text-sm font-medium"
		>{m.publish_slug_label()}</label
	>
	<div class="relative">
		<Input
			id="slug"
			{value}
			oninput={handleInput}
			placeholder={m.publish_slug_placeholder()}
			class="dark:bg-mofu-dark-700 bg-mofu-light-700 border-mofu-light-600 dark:border-mofu-dark-600 placeholder:text-mofu-light-400 dark:placeholder:text-mofu-dark-400 pr-12 text-black dark:text-white {validationError
				? 'border-rose-500'
				: ''}"
		/>
		<div
			class="absolute top-1/2 right-3 -translate-y-1/2 text-xs {validationError
				? 'text-rose-500'
				: 'dark:text-mofu-dark-400 text-mofu-light-400'}"
		>
			{slugCount}/80
		</div>
	</div>
	{#if validationError}
		<p class="mt-1 text-xs text-rose-500">{validationError}</p>
	{/if}
	<p class="dark:text-mofu-dark-400 text-mofu-light-400 mt-1 text-xs">
		@{userStore.user?.handle || '사용자핸들'}/{value}
	</p>
</div>
