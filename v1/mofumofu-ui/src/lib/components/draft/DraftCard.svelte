<script lang="ts">
	import PostCardImage from '../post/card/PostCardImage.svelte';
	import DraftCardContent from './DraftCardContent.svelte';
	import DraftCardFooter from './DraftCardFooter.svelte';
	import type { DraftInfo } from '$lib/api/draft/types';

	const {
		draft = undefined,
		onEdit,
		onDelete
	}: {
		draft?: DraftInfo;
		onEdit: (draft: DraftInfo) => void;
		onDelete: (draftId: string) => void;
	} = $props();

	// date formatting 함수 - 정확한 날짜/시간 표시
	const formatDate = (dateStr: string) => {
		const date = new Date(dateStr);
		const year = date.getFullYear();
		const month = String(date.getMonth() + 1).padStart(2, '0');
		const day = String(date.getDate()).padStart(2, '0');
		const hours = String(date.getHours()).padStart(2, '0');
		const minutes = String(date.getMinutes()).padStart(2, '0');

		return `${year}.${month}.${day} ${hours}:${minutes}`;
	};

	// skeleton 모드인지 확인 (draft가 없으면 skeleton)
	let isSkeleton = $derived(!draft);

	// 파생된 값들
	let formattedDate = $derived(draft ? formatDate(draft.updated_at || draft.created_at) : '');
</script>

<!-- 카드 전체 -->
<div
	data-sveltekit-preload-data="false"
	class="dark:bg-mofu-dark-800 bg-mofu-light-800 border-mofu-light-800 dark:border-mofu-dark-800 group flex h-full cursor-pointer flex-col overflow-hidden rounded-xl border transition-all duration-200 hover:-translate-y-1 hover:opacity-75 hover:shadow-lg md:min-h-[300px]"
>
	<button onclick={() => draft && onEdit(draft)} class="flex flex-grow flex-col">
		<!-- 이미지 영역 (스켈레톤이거나 이미지가 있을 때만) -->
		{#if isSkeleton || draft?.thumbnail_image}
			<PostCardImage image={draft?.thumbnail_image || undefined} title={draft?.title || ''} {isSkeleton} />
		{/if}

		<!-- 텍스트 영역 -->
		<DraftCardContent title={draft?.title || ''} summary={draft?.summary || ''} date={formattedDate} {isSkeleton} />
	</button>

	<!-- 구분선과 푸터 -->
	<div class="dark:border-mofu-dark-600 border-mofu-light-600 border-t">
		<DraftCardFooter slug={draft?.slug || ''} {onEdit} {onDelete} {draft} {isSkeleton} />
	</div>
</div>
