<script lang="ts">
	import { getComments } from '$lib/api/comment/commentApi';
	import type { CommentInfo, GetCommentsRequest } from '$lib/api/comment/types';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import CommentItem from './CommentItem.svelte';
	import CommentForm from './forms/CommentForm.svelte';
	import CommentSkeleton from './CommentSkeleton.svelte';
	import { ChevronDown, ChevronUp, TrendingUp, Edit3, MessageCircle } from '@lucide/svelte';
	import { onMount } from 'svelte';

	interface Props {
		postId: string;
		initialComments?: CommentInfo[];
		initialCount?: number;
		perPage?: number;
		replyPerPage?: number;
		defaultSort?: 'latest' | 'oldest' | 'popular';
	}

	let {
		postId,
		initialComments = [],
		initialCount = 0,
		perPage = 10,
		replyPerPage = 5,
		defaultSort = 'latest'
	}: Props = $props();

	// 상태 관리
	let comments = $state<CommentInfo[]>(initialComments);
	let commentCount = $state(initialCount);
	let currentPage = $state(1);
	let loading = $state(false);
	let allLoaded = $state(false);
	let sortOrder = $state<'latest' | 'oldest' | 'popular'>(defaultSort);

	// 정렬 버튼 클릭
	const toggleSortOrder = async () => {
		const orders: Array<'latest' | 'oldest' | 'popular'> = ['latest', 'oldest', 'popular'];
		const currentIndex = orders.indexOf(sortOrder);
		sortOrder = orders[(currentIndex + 1) % orders.length];

		currentPage = 1;
		comments = [];
		allLoaded = false;
		await loadComments();
	};

	// 댓글 로드
	const loadComments = async () => {
		if (loading || allLoaded) return;

		try {
			loading = true;

			const request: GetCommentsRequest = {
				post_id: postId,
				page: currentPage,
				per_page: perPage,
				sort: sortOrder
			};

			const response = await getComments(request);

			if (currentPage === 1) {
				comments = response.comments;
			} else {
				comments = [...comments, ...response.comments];
			}

			commentCount = response.total_count;
			allLoaded = !response.has_next;
			currentPage++;
		} catch (error) {
			console.error('Failed to load comments:', error);
		} finally {
			loading = false;
		}
	};

	// 새 댓글 추가 처리
	const handleNewComment = async (newComment: CommentInfo) => {
		// 최상위 댓글인 경우만 목록에 추가
		if (!newComment.parent_id) {
			if (sortOrder === 'latest') {
				comments = [newComment, ...comments];
			} else {
				comments = [...comments, newComment];
			}
			commentCount++;
		}
	};

	// 답글 추가 처리
	const handleReply = (newReply: CommentInfo) => {
		commentCount++;
	};

	// 좋아요 상태 업데이트 함수 (더 이상 필요 없음)
	const updateLikeStatus = (commentId: string, isLiked: boolean) => {
		// 각 댓글이 자체적으로 관리
	};

	// 댓글 찾기 헬퍼 함수
	const findCommentById = (commentList: CommentInfo[], id: string): CommentInfo | null => {
		for (const comment of commentList) {
			if (comment.id === id) return comment;
		}
		return null;
	};

	// 클라이언트 사이드에서만 초기 로드
	onMount(() => {
		if (initialComments.length === 0) {
			loadComments();
		}
	});

	// 정렬 옵션 표시 텍스트
	const getSortText = (sort: string) => {
		switch (sort) {
			case 'latest':
				return '최신순';
			case 'oldest':
				return '등록순';
			case 'popular':
				return '인기순';
			default:
				return '최신순';
		}
	};
</script>

<div class="">
	<!-- 헤더 -->
	<div class="flex items-center justify-between">
		<div class="flex items-center gap-4">
			<h2 class="text-mofu-light-200 dark:text-mofu-dark-200 text-lg font-medium">댓글 {commentCount}개</h2>
			<button
				onclick={toggleSortOrder}
				disabled={loading}
				class="text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 hover:bg-mofu-light-900 dark:hover:bg-mofu-dark-800 flex items-center gap-1.5 rounded px-2 py-1 text-sm transition-colors disabled:opacity-50"
			>
				{#if sortOrder === 'latest'}
					<ChevronDown class="size-4" />
				{:else if sortOrder === 'oldest'}
					<ChevronUp class="size-4" />
				{:else}
					<TrendingUp class="size-4" />
				{/if}
				{getSortText(sortOrder)}
			</button>
		</div>

		{#if userStore.user}
			<button
				onclick={() => {
					document.getElementById('comment_write')?.scrollIntoView({ block: 'center' });
					document.getElementById('comment_write_area')?.focus();
				}}
				class="bg-mofu hover:bg-mofu/90 flex items-center gap-2 rounded-md px-3 py-1.5 text-sm text-white transition-colors"
			>
				<Edit3 class="size-4" />
				댓글 쓰기
			</button>
		{/if}
	</div>

	<!-- 로딩 상태 (처음 로드시) -->
	{#if loading && comments.length === 0}
		<div class="space-y-2">
			{#each Array(3) as _}
				<CommentSkeleton />
			{/each}
		</div>
	{/if}

	<!-- 댓글 목록 -->
	<div class="overflow-x-auto overflow-y-hidden pb-2">
		{#each comments as comment (comment.id)}
			<CommentItem {comment} {postId} depth={0} onReply={handleReply} onLikeUpdate={updateLikeStatus} {replyPerPage} />
		{/each}

		<!-- 더 보기 버튼 -->
		{#if !allLoaded && comments.length > 0}
			<div class="ml-4 flex">
				<button
					onclick={loadComments}
					disabled={loading}
					class="text-mofu-light-400 hover:text-mofu-light-200 dark:text-mofu-dark-400 dark:hover:text-mofu-dark-200 flex cursor-pointer items-center gap-1.5 pt-2 pl-2 text-sm hover:underline disabled:opacity-50"
				>
					<MessageCircle class="size-5" />
					댓글 더 불러오기
					{#if loading}
						<div
							class="border-mofu-light-400 dark:border-mofu-dark-400 ml-2 h-4 w-4 animate-spin rounded-full border-2 border-t-transparent"
						></div>
					{/if}
				</button>
			</div>
		{/if}
	</div>

	<!-- 댓글 작성 폼 -->
	{#if userStore.user}
		<div class="p-4" id="comment_write">
			<CommentForm {postId} onSubmit={handleNewComment} />
		</div>
	{/if}
</div>
