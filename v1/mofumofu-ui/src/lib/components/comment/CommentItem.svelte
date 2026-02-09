<script lang="ts">
	import type { CommentInfo } from '$lib/api/comment/types';
	import { getReplies, updateComment, deleteComment } from '$lib/api/comment/commentApi';
	import { userStore } from '$lib/stores/user.svelte';
	import { toast } from 'svelte-sonner';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import CommentForm from './forms/CommentForm.svelte';
	import CommentContent from './ui/CommentContent.svelte';
	import CommentAvatar from './ui/CommentAvatar.svelte';
	import CommentHeader from './ui/CommentHeader.svelte';
	import CommentLikeButton from './actions/CommentLikeButton.svelte';
	import CommentActionsMenu from './actions/CommentActionsMenu.svelte';
	import CommentEditForm from './forms/CommentEditForm.svelte';
	import ReportDialog from '../report/ReportDialog.svelte';
	import CommentThreadLines from './ui/CommentThreadLines.svelte';
	import Self from './CommentItem.svelte';
	import { ChevronRight, CircleArrowRight, Mail, MessageCirclePlus, Reply, SendHorizontal } from '@lucide/svelte';
	import * as m from '../../../paraglide/messages';

	interface Props {
		comment: CommentInfo;
		postId: string;
		depth?: number;
		isLast?: boolean;
		onReply?: (newComment: CommentInfo) => void;
		onLikeUpdate?: (commentId: string, isLiked: boolean) => void;
		replyPerPage?: number;
	}

	let { comment, postId, depth = 0, isLast = false, onReply, onLikeUpdate, replyPerPage = 5 }: Props = $props();

	// 로칼 상태 관리
	let showReplyForm = $state(false);
	let showEditForm = $state(false);

	let showChildren = $state(false);
	let children = $state<CommentInfo[]>([]);
	let loadingChildren = $state(false);
	let allChildrenLoaded = $state(false);
	let currentPage = $state(1);
	let editContent = $state(comment.content || '');

	// 드롭다운 관련 상태
	let isDropdownOpen = $state(false);
	let closeTimer: ReturnType<typeof setTimeout> | null = null;

	// 삭제 다이얼로그 상태
	let isDeleteModalOpen = $state(false);
	let isDeleting = $state(false);

	// 신고 다이얼로그 상태
	let isReportDialogOpen = $state(false);

	// 댓글 로칼 상태
	let localCommentContent = $state(comment.content);
	let localIsDeleted = $state(comment.is_deleted);
	let localReplyCount = $state(comment.reply_count);

	// 남은 답글 수 계산
	const remainChildren = $derived(() => {
		if (localReplyCount === 0 || allChildrenLoaded || children.length === 0) return 0;
		return Math.max(0, localReplyCount - children.length);
	});

	// 삭제된 댓글인지 확인
	const isDeleted = $derived(localIsDeleted);
	const displayContent = $derived(isDeleted ? m.comment_deleted() : localCommentContent);
	const displayUserName = $derived(isDeleted ? 'Anon' : comment.user_name);
	const displayUserHandle = $derived(isDeleted ? '' : comment.user_handle);
	const displayUserImage = $derived(isDeleted ? null : comment.user_profile_image);

	// 현재 사용자가 댓글 작성자인지 확인
	const isOwner = $derived(userStore.user?.handle === comment.user_handle);

	// comment prop이 변경될 때 로컬 상태 업데이트
	$effect(() => {
		localCommentContent = comment.content;
		localIsDeleted = comment.is_deleted;
		localReplyCount = comment.reply_count;
		editContent = comment.content || '';
	});

	// 드롭다운 관리 함수들
	function openDropdown() {
		if (closeTimer) {
			clearTimeout(closeTimer);
			closeTimer = null;
		}
		isDropdownOpen = true;
	}

	function scheduleClose() {
		closeTimer = setTimeout(() => {
			isDropdownOpen = false;
			closeTimer = null;
		}, 100);
	}

	function handleEditClick() {
		isDropdownOpen = false;
		showEditForm = !showEditForm;
	}

	// 답글 목록 로드
	const loadChildren = async () => {
		if (loadingChildren || allChildrenLoaded) return;

		try {
			loadingChildren = true;

			const response = await getReplies({
				parent_comment_id: comment.id,
				page: currentPage,
				per_page: replyPerPage,
				sort: 'latest'
			});

			children = [...children, ...response.replies];
			currentPage++;
			allChildrenLoaded = !response.has_next;
		} catch (error) {
			console.error('Failed to load replies:', error);
		} finally {
			loadingChildren = false;
		}
	};

	// 답글 표시/숨김 토글
	const toggleChildren = async () => {
		if (!showChildren && children.length === 0 && localReplyCount > 0) {
			await loadChildren();
		}
		showChildren = !showChildren;
	};

	// 댓글 수정
	const handleEdit = async () => {
		if (!editContent.trim()) return;

		try {
			await updateComment({ comment_id: comment.id, content: editContent.trim() });
			localCommentContent = editContent.trim();
			showEditForm = false;
		} catch (error) {
			console.error('Failed to update comment:', error);
		}
	};

	// 댓글 삭제 다이얼로그 열기
	const handleDelete = () => {
		isDropdownOpen = false;
		isDeleteModalOpen = true;
	};

	// 댓글 삭제 확인
	const confirmDelete = async () => {
		try {
			isDeleting = true;
			await deleteComment({ comment_id: comment.id });
			localIsDeleted = true;
			localCommentContent = null;
			toast.success(m.comment_deleted_success());
			isDeleteModalOpen = false;
		} catch (error) {
			console.error('Failed to delete comment:', error);
			toast.error(m.comment_delete_failed());
		} finally {
			isDeleting = false;
		}
	};

	// 댓글 삭제 취소
	const cancelDelete = () => {
		isDeleteModalOpen = false;
	};

	// 신고하기 클릭
	const handleReport = () => {
		isDropdownOpen = false;
		isReportDialogOpen = true;
	};

	// 답글 작성 완료
	const handleReplySubmit = async (newReply: CommentInfo) => {
		children = [newReply, ...children];
		localReplyCount++;
		showReplyForm = false;
		showChildren = true;
		onReply?.(newReply);
	};

	// 자식 댓글 답글 처리
	const handleChildReply = (newReply: CommentInfo) => {
		onReply?.(newReply);
	};
</script>

<!-- Reddit-style Comment -->
<div
	class="relative py-1.5 pl-4"
	class:-ml-4={depth === 0}
	class:ml-4={depth > 0}
	class:mb-6={showChildren && remainChildren() > 0}
>
	<!-- Thread lines -->
	<CommentThreadLines
		{depth}
		{isLast}
		hasReplies={localReplyCount > 0}
		{showChildren}
		onToggleChildren={toggleChildren}
	/>

	<!-- Comment main content -->
	<div
		class="group/comment hover:bg-mofu-light-900 dark:hover:bg-mofu-dark-800/50 flex cursor-pointer items-start gap-3 rounded-lg p-2 pb-1.5 transition-colors"
		role="button"
		tabindex="0"
		onclick={() => {
			if (!isDeleted) {
				showReplyForm = !showReplyForm;
			}
		}}
		onkeydown={(e) => {
			if ((e.key === 'Enter' || e.key === ' ') && !isDeleted) {
				e.preventDefault();
				showReplyForm = !showReplyForm;
			}
		}}
	>
		<!-- Avatar -->
		<CommentAvatar {displayUserName} {displayUserImage} />

		<!-- Comment content -->
		<div class="min-w-0 flex-1">
			<div class="flex items-start justify-between">
				<!-- User info and timestamp -->
				<CommentHeader
					{displayUserName}
					{displayUserHandle}
					{displayUserImage}
					createdAt={comment.created_at}
					updatedAt={comment.updated_at}
					{isDeleted}
				/>

				<!-- Actions menu -->
				{#if !isDeleted}
					<div class="flex items-center gap-1">
						<!-- Like button -->
						<CommentLikeButton
							commentId={comment.id}
							initialLikeCount={comment.like_count || 0}
							{isDeleted}
							{onLikeUpdate}
						/>

						<!-- Actions menu -->
						<CommentActionsMenu
							{isDropdownOpen}
							{isOwner}
							onEdit={handleEditClick}
							onDelete={handleDelete}
							onReport={handleReport}
							onOpenDropdown={openDropdown}
							onScheduleClose={scheduleClose}
						/>
					</div>
				{/if}
			</div>

			<!-- Comment text or edit form -->
			{#if showEditForm && !isDeleted}
				<CommentEditForm
					{editContent}
					onSave={handleEdit}
					onCancel={() => (showEditForm = false)}
					onContentChange={(value) => (editContent = value)}
				/>
			{:else}
				<div class="mt-2">
					<div
						class="text-mofu-light-200 dark:text-mofu-dark-200 text-sm"
						class:text-mofu-light-500={isDeleted}
						class:dark:text-mofu-dark-500={isDeleted}
						class:italic={isDeleted}
					>
						{#if displayContent}
							<CommentContent content={displayContent} />
						{/if}
					</div>
				</div>
			{/if}
		</div>
	</div>

	<!-- Children section with thread lines -->
	{#if localReplyCount > 0}
		<!-- Reply items -->
		{#if showChildren}
			{#each children as reply, i (reply.id)}
				<Self
					comment={reply}
					{postId}
					onReply={handleChildReply}
					{onLikeUpdate}
					depth={depth + 1}
					isLast={i === children.length - 1}
					{replyPerPage}
				/>
			{/each}

			<!-- Reply and Load more buttons -->
			{#if !showReplyForm}
				<div
					class="pointer-events-auto absolute left-[31.5px] z-10 flex flex-col items-start gap-2"
					class:-bottom-8={remainChildren() > 0}
					class:-bottom-2={remainChildren() === 0}
				>
					<!-- Reply button -->
					<button
						onclick={(e) => {
							e.preventDefault();
							e.stopPropagation();
							showReplyForm = !showReplyForm;
						}}
						type="button"
						class="group/btn bg-mofu-light-950 dark:bg-mofu-dark-900 border-mofu-light-300 dark:border-mofu-dark-600 cursor-pointer rounded-full border p-0.5"
						title="답글 작성"
						aria-label="답글 작성"
					>
						<ChevronRight class="size-3.5 opacity-60 transition-opacity group-hover/btn:opacity-100" strokeWidth={2} />
					</button>

					<!-- Load more replies -->
					{#if remainChildren() > 0}
						<div class="-mt-2 ml-[9px] flex">
							<div
								class="border-mofu-light-300 dark:border-mofu-dark-600 h-4 w-4 rounded-bl-full border-b border-l"
							></div>
							<button
								onclick={loadChildren}
								disabled={loadingChildren}
								class="text-mofu-light-400 dark:text-mofu-dark-400 hover:text-mofu-light-200 dark:hover:text-mofu-dark-200 flex cursor-pointer items-center gap-1.5 pt-1 pl-2 text-sm hover:underline disabled:cursor-not-allowed disabled:opacity-50"
								aria-label={m.comment_load_more({ count: remainChildren() })}
							>
								<div>{m.comment_load_more({ count: remainChildren() })}</div>
								{#if loadingChildren}
									<div class="border-mofu h-4 w-4 animate-spin rounded-full border-1 border-t-transparent"></div>
								{/if}
							</button>
						</div>
					{/if}
				</div>
			{/if}
		{:else}
			<!-- Collapsed state - show count with connection line -->
			<div class="-mt-2 p-4">
				<div
					class="border-mofu-light-300 dark:border-mofu-dark-600 absolute bottom-2 left-[41px] h-4 w-4 rounded-bl-full border-b border-l"
				></div>
				<button
					onclick={toggleChildren}
					class="text-mofu-light-400 dark:text-mofu-dark-400 hover:text-mofu-light-200 dark:hover:text-mofu-dark-200 absolute left-[60px] flex cursor-pointer items-center gap-1.5 pt-1 text-sm hover:underline disabled:cursor-not-allowed disabled:opacity-50"
					disabled={loadingChildren}
					aria-label={m.comment_show_replies({ count: localReplyCount })}
				>
					<div>{m.comment_show_replies({ count: localReplyCount })}</div>
					{#if loadingChildren}
						<div class="border-mofu h-4 w-4 animate-spin rounded-full border-1 border-t-transparent"></div>
					{/if}
				</button>
			</div>
		{/if}
	{/if}

	<!-- Reply form -->
	{#if showReplyForm}
		<div class="relative ml-8 pt-4">
			<div class="bg-mofu-light-950 dark:bg-mofu-dark-900 absolute top-0 -left-6 h-[calc(100%+2rem)] w-6"></div>
			<div
				class="border-mofu-light-300 dark:border-mofu-dark-600 absolute top-0 -left-[7px] h-4 w-4 border-l border-dashed"
			></div>
			<div
				class="border-mofu-light-300 dark:border-mofu-dark-600 absolute top-4 -left-[7px] h-4 w-4 rounded-bl-full border-b border-l border-dashed"
			></div>

			<CommentForm
				{postId}
				parentId={comment.id}
				placeholder={m.comment_reply_placeholder({ name: displayUserName || 'User' })}
				onSubmit={handleReplySubmit}
				onCancel={() => (showReplyForm = false)}
			/>
		</div>
	{/if}
</div>

<!-- 댓글 삭제 확인 Dialog -->
<Dialog.Root bind:open={isDeleteModalOpen}>
	<Dialog.Content class="dark:bg-mofu-dark-800 p-2 text-black sm:max-w-md dark:text-white">
		<div class="rounded-lg px-2 pt-4">
			<Dialog.Header class="mb-2 p-0">
				<Dialog.Title class="text-lg font-semibold">{m.comment_delete_dialog_title()}</Dialog.Title>
				<Dialog.Description class="text-gray-600 dark:text-gray-300">
					{@html m.comment_delete_dialog_description()}
				</Dialog.Description>
			</Dialog.Header>
		</div>

		<!-- 버튼 영역 -->
		<div class="flex justify-end gap-3 rounded-b-lg px-2 py-2">
			<Button variant="ghost" onclick={cancelDelete} disabled={isDeleting}>{m.comment_delete_cancel()}</Button>
			<Button variant="destructive" onclick={confirmDelete} disabled={isDeleting}>
				{isDeleting ? m.comment_deleting() : m.comment_delete_confirm()}
			</Button>
		</div>
	</Dialog.Content>
</Dialog.Root>

<!-- 댓글 신고 Dialog -->
<ReportDialog
	targetId={comment.id}
	targetType="Comment"
	open={isReportDialogOpen}
	onOpenChange={(open) => (isReportDialogOpen = open)}
/>
