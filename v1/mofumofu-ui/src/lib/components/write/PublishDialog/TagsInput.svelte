<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
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
	let currentTagInput = $state('');

	const tagArray = $derived([
		...new Set(
			value
				.split(/[,\n]/)
				.map((tag) => tag.trim())
				.filter((tag) => tag.length > 0)
		)
	]);
	const tagCount = $derived(tagArray.length);

	function validateTags(tagsValue: string) {
		const schema = createPostSchema();
		const dataToValidate = {
			title: 'temp',
			content: 'temp',
			slug: 'temp',
			summary: '',
			tags: tagsValue
		};

		const result = v.safeParse(schema, dataToValidate);

		if (result.success) {
			validationError = '';
			onValidationChange();
		} else {
			const tagsError = result.issues.find((issue) => issue.path?.[0]?.key === 'tags');
			if (tagsError) {
				validationError = tagsError.message;
				onValidationChange(tagsError.message);
			}
		}
	}

	function addTag() {
		const trimmedTag = currentTagInput.trim();
		if (trimmedTag && tagArray.length < 8 && !tagArray.includes(trimmedTag)) {
			const newTags = [...tagArray, trimmedTag];
			const newValue = newTags.join(',');
			onUpdate(newValue);
			currentTagInput = '';
			validateTags(newValue);
		}
	}

	function removeTag(tagToRemove: string) {
		const newTags = tagArray.filter((tag) => tag !== tagToRemove);
		const newValue = newTags.join(',');
		onUpdate(newValue);
		validateTags(newValue);
	}

	function handleTagKeyPress(event: KeyboardEvent) {
		if (event.key === 'Enter' || event.key === ',') {
			event.preventDefault();
			addTag();
		}
	}
</script>

<div>
	<label for="tags" class=" text-mofu-light-100 dark:text-mofu-dark-100 mb-2 block text-sm font-medium"
		>{m.publish_tags_label()}</label
	>
	<div class="space-y-2">
		<!-- 기존 태그들 표시 -->
		{#if tagArray.length > 0}
			<div class="flex flex-wrap gap-2">
				{#each tagArray as tag}
					<Badge
						variant="default"
						class="dark:hover:bg-mofu-dark-700 hover:bg-mofu-light-700 bg-mofu-light-800 dark:bg-mofu-dark-800 text-mofu cursor-pointer"
						onclick={() => removeTag(tag)}
					>
						#{tag}
					</Badge>
				{/each}
			</div>
		{/if}

		<!-- 태그가 5개 미만일 때만 입력 필드 표시 -->
		{#if tagArray.length < 8}
			<div class="relative">
				<Input
					bind:value={currentTagInput}
					onkeydown={handleTagKeyPress}
					placeholder={m.publish_tags_placeholder()}
					class="dark:bg-mofu-dark-700 bg-mofu-light-700 border-mofu-light-600 dark:border-mofu-dark-600 placeholder:text-mofu-light-400 dark:placeholder:text-mofu-dark-400 pr-12 text-white"
				/>
				<div
					class="absolute top-1/2 right-3 -translate-y-1/2 text-xs {validationError
						? 'text-rose-500'
						: 'dark:text-mofu-dark-400 text-mofu-light-400'}"
				>
					{tagCount}/8
				</div>
			</div>
		{:else}
			<!-- 태그가 5개일 때는 개수만 표시 -->
			<div
				class="text-xs {validationError ? 'text-rose-500' : 'text-mofu-light-400 dark:text-mofu-dark-400'} text-right"
			>
				{tagCount}/8
			</div>
		{/if}
	</div>
	{#if validationError}
		<p class="mt-1 text-xs text-rose-500">{validationError}</p>
	{:else}
		<p class="text-mofu-dark-400 mt-1 text-xs">태그를 클릭하면 삭제됩니다. Enter 또는 쉼표로 구분하여 입력하세요.</p>
	{/if}
</div>
