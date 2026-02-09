<script lang="ts">
	import { createComment } from '$lib/api/comment/commentApi';
	import type { CommentInfo, CreateCommentRequest } from '$lib/api/comment/types';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import { Send, Loader2 } from '@lucide/svelte';

	interface Props {
		postId: string;
		parentId?: string;
		placeholder?: string;
		onSubmit?: (comment: CommentInfo) => void;
		onCancel?: () => void;
		focus?: boolean;
	}

	let { postId, parentId, placeholder = '댓글을 작성하세요...', onSubmit, onCancel, focus = false }: Props = $props();

	let content = $state('');
	let isSubmitting = $state(false);
	let error = $state('');
	let textareaElement = $state<HTMLTextAreaElement>();

	const maxLength = 500;
	const remainingChars = $derived(maxLength - content.length);
	const user = $derived(userStore.user);

	// focus 처리
	$effect(() => {
		if (focus && textareaElement) {
			textareaElement.focus();
			textareaElement.setSelectionRange(textareaElement.value.length, textareaElement.value.length);
		}
	});

	// 자동 높이 조절
	const adjustHeight = () => {
		if (textareaElement) {
			textareaElement.style.height = 'auto';
			textareaElement.style.height = textareaElement.scrollHeight + 'px';
		}
	};

	const handleSubmit = async () => {
		if (!content.trim() || isSubmitting || !authStore.isAuthenticated) return;

		error = '';

		try {
			isSubmitting = true;

			const request: CreateCommentRequest = {
				content: content.trim(),
				post_id: postId,
				parent_id: parentId || null
			};

			const commentContent = content.trim();
			const response = await createComment(request);

			// 부모 컴포넌트에 성공 알림
			if (onSubmit) {
				const newComment: CommentInfo = {
					id: response.comment_id,
					post_id: postId,
					content: commentContent,
					parent_id: parentId || null,
					user_id: null,
					user_handle: user?.handle || null,
					user_name: user?.name || null,
					user_profile_image: user?.profile_image || null,
					like_count: 0,
					reply_count: 0,
					is_deleted: false,
					created_at: new Date().toISOString(),
					updated_at: null
				};
				onSubmit(newComment);
			}

			// 성공 시 폼 초기화
			content = '';
			adjustHeight();
		} catch (err: any) {
			console.error('Failed to submit comment:', err);
			error = err.message || '댓글 작성에 실패했습니다.';
		} finally {
			isSubmitting = false;
		}
	};

	const handleKeyDown = (e: KeyboardEvent) => {
		// Enter만으로 전송 (모바일에서 Ctrl+Enter 어려움)
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			handleSubmit();
		}
	};

	const handleCancel = () => {
		content = '';
		error = '';
		adjustHeight();
		onCancel?.();
	};
</script>

<div class="comment-form" id={parentId ? undefined : 'comment_write'}>
	{#if !authStore.isAuthenticated}
		<div
			class="text-mofu-light-500 dark:text-mofu-dark-400 border-mofu-light-600 dark:border-mofu-dark-600 ml-2 flex items-center justify-center rounded-lg border py-8 text-sm"
		>
			로그인이 필요합니다.
		</div>
	{:else}
		<div class="flex gap-3">
			<!-- 사용자 프로필 이미지 -->
			<div class="z-20 flex-shrink-0">
				{#if user?.profile_image}
					<img src={user.profile_image} alt={user.name} class="h-10 w-10 rounded-full object-cover" />
				{:else}
					<div class="bg-mofu-light-700 dark:bg-mofu-dark-700 flex h-10 w-10 items-center justify-center rounded-full">
						<span class="text-mofu-light-200 dark:text-mofu-dark-200 text-sm font-medium">
							{user?.name?.charAt(0).toUpperCase()}
						</span>
					</div>
				{/if}
			</div>

			<!-- 댓글 입력 영역 -->
			<div
				class="min-w-0 flex-1"
				onclick={(e) => e.stopPropagation()}
				onkeydown={(e) => {
					if (e.key === 'Escape') e.stopPropagation();
				}}
				role="dialog"
				tabindex="0"
			>
				<div
					class="bg-mofu-light-950 dark:bg-mofu-dark-900 border-mofu-light-600 dark:border-mofu-dark-600 focus-within:border-mofu focus-within:ring-mofu rounded-lg border transition-colors focus-within:ring-1"
				>
					<!-- 텍스트 영역 -->
					<textarea
						bind:this={textareaElement}
						bind:value={content}
						onkeydown={handleKeyDown}
						onclick={(e) => e.stopPropagation()}
						{placeholder}
						disabled={isSubmitting}
						maxlength={maxLength}
						class="text-mofu-light-100 dark:text-mofu-dark-100 placeholder-mofu-light-400 dark:placeholder-mofu-dark-500 w-full resize-none border-0 bg-transparent p-3 text-sm focus:ring-0 focus:outline-none"
						rows="3"
						oninput={adjustHeight}
						id={parentId ? undefined : 'comment_write_area'}
					></textarea>

					<!-- 오류 메시지 및 하단 바 -->
					{#if error}
						<div class="px-3 pb-2 text-xs text-red-600 dark:text-red-400">
							{error}
						</div>
					{/if}

					<div class="flex items-center justify-between px-3 pb-3">
						<div class="text-mofu-light-500 dark:text-mofu-dark-400 flex items-center gap-2 text-xs">
							<!-- 문자 수 카운터 -->
							<span class="text-xs" class:text-red-500={remainingChars < 50}>
								{content.length}/{maxLength}자
							</span>

							<!-- 단축키 힌트 -->
							<span class="text-mofu-light-400 dark:text-mofu-dark-500 text-xs">
								Enter로 등록 • Shift+Enter로 줄바꿈
							</span>
						</div>

						<!-- 액션 버튼들 -->
						<div class="flex items-center gap-2">
							{#if onCancel}
								<button
									type="button"
									onclick={(e) => {
										e.stopPropagation();
										handleCancel();
									}}
									disabled={isSubmitting}
									class="text-mofu-light-600 hover:text-mofu-light-200 disabled:text-mofu-light-400 hover:bg-mofu-light-800 dark:hover:bg-mofu-dark-700 rounded px-3 py-1.5 text-xs transition-colors"
								>
									취소
								</button>
							{/if}

							<button
								type="button"
								onclick={(e) => {
									e.stopPropagation();
									handleSubmit();
								}}
								disabled={!content.trim() || isSubmitting || remainingChars < 0}
								class="bg-mofu hover:bg-mofu/90 disabled:bg-mofu-light-600 flex items-center gap-1.5 rounded-md px-4 py-1.5 text-xs text-white transition-colors disabled:cursor-not-allowed"
							>
								{#if isSubmitting}
									<Loader2 class="size-3 animate-spin" />
									등록 중...
								{:else}
									<Send class="size-3" />
									{parentId ? '답글' : '댓글'} 등록
								{/if}
							</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	textarea {
		min-height: 44px;
		max-height: 200px;
	}
</style>
