<script lang="ts">
	import { Badge } from '../ui/badge';
	import * as m from '../../../paraglide/messages';

	interface Props {
		title: string;
		tags: string;
		showStickyToolbar: boolean;
		onTitleChange: (value: string) => void;
		onTagsChange: (value: string) => void;
	}

	const { title, tags, showStickyToolbar, onTitleChange, onTagsChange }: Props = $props();

	const tagArray = $derived([
		...new Set(
			tags
				.split(/[,\n]/)
				.map((tag) => tag.trim())
				.filter((tag) => tag.length > 0)
		)
	]);

	const titleCount = $derived(title.length);
	const titleOverLimit = $derived(titleCount > 80);

	let currentInput = $state('');

	function handleTagKeyPress(event: KeyboardEvent) {
		if (event.key === 'Enter' || event.key === ',') {
			event.preventDefault();
			addCurrentTag();
		} else if (event.key === 'Backspace' && currentInput === '' && tagArray.length > 0) {
			event.preventDefault();
			removeTag(tagArray[tagArray.length - 1]);
		}
	}

	function addCurrentTag() {
		const trimmedTag = currentInput.trim();
		if (trimmedTag) {
			const currentTags = new Set(
				tags
					.split(/[,\n]/)
					.map((tag) => tag.trim())
					.filter((tag) => tag.length > 0)
			);

			// 8개 제한 체크
			if (currentTags.size >= 8) {
				alert('태그는 최대 8개까지 추가할 수 있습니다.'); // TODO: 이것도 i18n 필요
				return;
			}

			currentTags.add(trimmedTag);
			onTagsChange([...currentTags].join(','));
			currentInput = '';
		}
	}

	function removeTag(tagToRemove: string) {
		const currentTags = new Set(
			tags
				.split(/[,\n]/)
				.map((tag) => tag.trim())
				.filter((tag) => tag.length > 0)
		);
		currentTags.delete(tagToRemove);
		onTagsChange([...currentTags].join(','));
	}
</script>

<div
	class="dark:bg-mofu-dark-900 bg-mofu-light-900 overflow-hidden pt-4 break-all transition-all duration-400 ease-in-out"
	style="max-height: {showStickyToolbar ? '0px' : '480px'}; opacity: {showStickyToolbar ? '0' : '1'};"
>
	<!-- 제목 입력 -->
	<div class="mb-6 px-6">
		<div class="relative">
			<input
				value={title}
				oninput={(e) => onTitleChange((e.target as HTMLInputElement).value)}
				placeholder={m.write_title_placeholder()}
				class="dark:placeholder:text-mofu-dark-600 placeholder:text-mofu-light-600 text-mofu-light-200 dark:text-mofu-dark-200 h-auto w-full border-none bg-transparent p-0 text-4xl font-bold outline-none {titleOverLimit
					? 'text-red-400'
					: ''}"
				style="font-size: 2.5rem; line-height: 1.2;"
			/>
			<!-- 글자수 표시 -->
			<div
				class="absolute right-0 -bottom-6 text-sm {titleOverLimit
					? 'text-red-400'
					: 'text-mofu-light-400 dark:text-mofu-dark-400'}"
			>
				{titleCount}/80
			</div>
		</div>
	</div>

	<!-- 태그 입력 -->
	<div class="mb-4 px-6">
		<div class="flex flex-wrap items-center gap-2 text-lg">
			<!-- 기존 태그들 표시 -->
			{#each tagArray as tag}
				<Badge onclick={() => removeTag(tag)}>
					#{tag}
				</Badge>
			{/each}

			<!-- 태그 입력 필드 (5개 미만일 때만 표시) -->
			{#if tagArray.length < 8}
				<input
					bind:value={currentInput}
					onkeydown={handleTagKeyPress}
					placeholder={m.write_tags_placeholder()}
					class="placeholder:text-mofu-dark-600 text-mofu-dark-200 min-w-[120px] flex-1 border-none bg-transparent p-0 outline-none"
				/>
			{/if}
		</div>
		<!-- 태그 개수 표시 -->
		<div class="mt-2 text-xs {tagArray.length >= 8 ? 'text-red-400' : 'text-mofu-dark-400'}">
			태그: {tagArray.length}/8
		</div>
	</div>
</div>
