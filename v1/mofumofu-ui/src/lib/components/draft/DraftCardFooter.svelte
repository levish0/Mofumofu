<script lang="ts">
	import { Pencil, Trash2 } from '@lucide/svelte';
	import type { DraftInfo } from '$lib/api/draft/types';

	const {
		slug = '',
		onEdit,
		onDelete,
		draft = undefined,
		isSkeleton = false
	}: {
		slug: string;
		onEdit: (draft: DraftInfo) => void;
		onDelete: (draftId: string) => void;
		draft?: DraftInfo;
		isSkeleton?: boolean;
	} = $props();
</script>

{#if isSkeleton}
	<div class="flex items-center justify-between px-3 py-2">
		<div class="flex items-center gap-1.5">
			<div class="shimmer h-5 w-5 rounded-full"></div>
			<div class="shimmer h-3 w-14 rounded"></div>
		</div>
		<div class="flex items-center gap-3">
			<div class="shimmer h-3 w-6 rounded"></div>
			<div class="shimmer h-3 w-6 rounded"></div>
		</div>
	</div>
{:else}
	<!-- 편집/삭제 버튼들을 반으로 나누어 배치 -->
	<div class="flex h-10">
		<!-- 편집 버튼 (왼쪽 절반, 파란색) -->
		<button
			onclick={(e) => {
				e.stopPropagation();
				draft && onEdit(draft);
			}}
			class="flex flex-1 items-center justify-center gap-2 bg-blue-50 text-sm font-medium text-indigo-500 transition-colors hover:bg-blue-100 dark:bg-blue-900/20 dark:text-indigo-500 dark:hover:bg-blue-900/30"
		>
			<Pencil class="h-4 w-4" />
			편집
		</button>

		<!-- 삭제 버튼 (오른쪽 절반, 빨간색) -->
		<button
			onclick={(e) => {
				e.stopPropagation();
				draft && onDelete(draft.draft_id);
			}}
			class="flex flex-1 items-center justify-center gap-2 bg-red-50 text-sm font-medium text-rose-500 transition-colors hover:bg-red-100 dark:bg-red-900/20 dark:text-rose-500 dark:hover:bg-red-900/30"
		>
			<Trash2 class="h-4 w-4" />
			삭제
		</button>
	</div>
{/if}
