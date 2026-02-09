<script lang="ts">
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import { getDrafts, deleteDraft, type DraftInfo } from '$lib/api/draft';
	import { goto } from '$app/navigation';
	import * as m from '../../../paraglide/messages';
	import LoadingOverlay from '$lib/components/common/LoadingOverlay.svelte';
	import AuthErrorScreen from '$lib/components/common/AuthErrorScreen.svelte';
	import DraftCard from '$lib/components/draft/DraftCard.svelte';
	import DraftDeleteModal from '$lib/components/draft/DraftDeleteModal.svelte';
	import { toast } from 'svelte-sonner';
	let drafts = $state<DraftInfo[]>([]);
	let isLoading = $state(true);
	let authError = $state(false);

	// 삭제 모달 관련 상태
	let isDeleteModalOpen = $state(false);
	let isDeleting = $state(false);
	let draftToDelete = $state<string | null>(null);

	// 권한 체크 및 데이터 로드
	onMount(async () => {
		try {
			// 인증 체크
			if (!authStore.isAuthenticated) {
				const refreshSuccess = await authStore.tryRefreshToken();
				if (!refreshSuccess) {
					authError = true;
					return;
				}
			}

			await userStore.loadProfile();

			// 인증된 사용자의 drafts 로드
			const response = await getDrafts();
			drafts = response.drafts;
		} catch (error) {
			console.error('Failed to load drafts:', error);
			authError = true;
		} finally {
			isLoading = false;
		}
	});

	// Draft 편집
	function editDraft(draft: DraftInfo) {
		goto(`/write?draft=${draft.draft_id}`);
	}

	// Draft 삭제 모달 열기
	function handleDeleteDraft(draftId: string) {
		draftToDelete = draftId;
		isDeleteModalOpen = true;
	}

	// 삭제 확인
	async function confirmDeleteDraft() {
		if (!draftToDelete) return;

		isDeleting = true;
		try {
			await deleteDraft({ draft_id: draftToDelete });
			drafts = drafts.filter((draft) => draft.draft_id !== draftToDelete);
			isDeleteModalOpen = false;
			draftToDelete = null;
		} catch (error) {
			console.error('Failed to delete draft:', error);
			toast.error('초안 삭제에 실패했습니다.');
		} finally {
			isDeleting = false;
		}
	}

	// 삭제 취소
	function cancelDeleteDraft() {
		isDeleteModalOpen = false;
		draftToDelete = null;
	}
</script>

<svelte:head>
	<title>내 초안 목록 - Mofu</title>
	<meta name="description" content="임시 저장된 글들을 관리하세요." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<LoadingOverlay isVisible={isLoading} message="초안을 불러오는 중..." />

<AuthErrorScreen isVisible={authError} description="초안을 볼 수 있는 권한이 없습니다." />

{#if !isLoading && !authError}
	<div class="min-h-screen">
		<div class="mb-8">
			<h1 class="text-3xl font-bold text-gray-900 dark:text-white">내 초안 목록</h1>
			<p class="mt-2 text-gray-600 dark:text-gray-300">
				임시 저장된 글들을 관리하세요 ({drafts.length}개)
			</p>
		</div>

		{#if drafts.length === 0}
			<div class="py-12 text-center">
				<div class="mb-4 text-gray-400 dark:text-gray-500">
					<svg class="mx-auto h-12 w-12" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
						/>
					</svg>
				</div>
				<h3 class="mb-2 text-lg font-medium text-gray-900 dark:text-white">저장된 초안이 없습니다</h3>
				<p class="mb-6 text-gray-500 dark:text-gray-400">글을 작성하고 임시저장하면 여기에 나타납니다</p>
				<button onclick={() => goto('/write')} class="bg-mofu rounded-lg px-4 py-2 text-white transition-colors">
					새 글 작성하기
				</button>
			</div>
		{:else}
			<!-- Draft Cards 그리드 -->
			<div class="min-h-screen">
				<div class="grid grid-cols-1 gap-x-5 gap-y-4 pb-20 sm:grid-cols-2 lg:grid-cols-4 xl:grid-cols-5">
					{#each drafts as draft (draft.draft_id)}
						<DraftCard {draft} onEdit={editDraft} onDelete={handleDeleteDraft} />
					{/each}
				</div>

				{#if drafts.length > 0}
					<div class="dark:text-mofu-dark-300 pb-20 text-center text-lg font-bold">
						총 {drafts.length}개의 초안이 있습니다 ✨
					</div>
				{/if}
			</div>
		{/if}
	</div>
{/if}

<!-- 삭제 확인 모달 -->
<DraftDeleteModal
	bind:isOpen={isDeleteModalOpen}
	{isDeleting}
	onConfirm={confirmDeleteDraft}
	onCancel={cancelDeleteDraft}
/>
