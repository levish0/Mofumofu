<script lang="ts">
	import '$lib/styles/markdown.css';
	import { useResizable } from '$lib/hooks/ui/useResizable.svelte';
	import WriteEditor from '$lib/components/write/WriteEditor.svelte';
	import WritePreview from '$lib/components/write/WritePreview.svelte';
	import { processMarkdown } from '$lib/utils/markdown';
	import { updatePost } from '$lib/api/post/postApi';
	import { userStore } from '$lib/stores/user.svelte';
	import { goto } from '$app/navigation';
	import { toast } from 'svelte-sonner';
	import type { PageData } from './$types';
	import * as m from '../../../paraglide/messages';
	import { authStore } from '$lib/stores/auth.svelte';
	import { onMount } from 'svelte';
	import LoadingOverlay from '$lib/components/common/LoadingOverlay.svelte';
	import AuthErrorScreen from '$lib/components/common/AuthErrorScreen.svelte';
	import NotVerifiedScreen from '$lib/components/common/NotVerifiedScreen.svelte';

	const { data }: { data: PageData } = $props();

	// 현재 사용자 확인 (권한 체크는 서버에서 하지만 클라이언트에서도 확인)
	const currentUser = $derived(userStore.user);

	// 기존 포스트 데이터로 초기화
	let title = $state(data.post.title);
	let tags = $state(data.post.tags.join(', '));
	let content = $state(data.post.content); // 이제 원본 마크다운을 가져올 수 있음
	let htmlOutput = $state('');
	let containerElement: HTMLElement | undefined = $state();
	let isPreviewMode = $state(false); // 모바일에서 프리뷰 모드인지
	let isAuthChecking = $state(true); // 인증 체크 중인지
	let authError = $state(false); // 인증 실패 상태
	let verificationError = $state(false); // 이메일 인증 실패 상태

	// Resizable hook
	let resizableHook = $state<ReturnType<typeof useResizable> | null>(null);

	$effect(() => {
		if (containerElement) {
			resizableHook = useResizable(containerElement);
		}
	});

	$effect(() => {
		(async () => {
			const { htmlContent } = await processMarkdown(content);
			htmlOutput = htmlContent;
		})();
	});

	function handleTitleChange(value: string) {
		title = value;
	}

	function handleTagsChange(value: string) {
		tags = value;
	}

	function handleContentChange(value: string) {
		content = value;
	}

	function handleSaveDraft() {
		console.log('Draft saved:', { title, tags, content });
	}

	function handleExit() {
		history.back();
	}

	function handleTogglePreviewMode(isPreview: boolean) {
		isPreviewMode = isPreview;
	}

	// 인증 체크
	onMount(async () => {
		try {
			// 토큰이 없으면 refresh 시도
			if (!authStore.isAuthenticated) {
				const refreshSuccess = await authStore.tryRefreshToken();

				if (!refreshSuccess) {
					authError = true;
					return;
				}
			}

			// 사용자 정보 로드 및 이메일 인증 체크
			await userStore.loadProfile();
			if (!userStore.user?.is_verified) {
				verificationError = true;
				return;
			}
		} finally {
			isAuthChecking = false;
		}
	});
</script>

<svelte:head>
	<title>{m.edit_page_title({ title: data.post.title })}</title>
	<meta name="description" content={m.edit_page_description()} />
	<meta name="robots" content="noindex, nofollow" />

	<!-- Open Graph -->
	<meta property="og:title" content={m.edit_page_meta_title()} />
	<meta property="og:description" content={m.edit_page_description()} />
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content="Mofumofu" />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary" />
	<meta name="twitter:title" content={m.edit_page_meta_title()} />
	<meta name="twitter:description" content={m.edit_page_description()} />
</svelte:head>

<LoadingOverlay isVisible={isAuthChecking} message={m.edit_preparing_editor()} />

<AuthErrorScreen isVisible={authError} description={m.edit_auth_required()} />

<NotVerifiedScreen isVisible={verificationError} description={m.edit_email_verification_required()} />

{#if !authError && !verificationError}
	<div class="flex h-full w-full bg-gray-900 break-all text-white dark:bg-gray-900">
		<!-- 메인 컨텐츠 영역 -->
		<div bind:this={containerElement} class="flex flex-1 overflow-hidden">
			<!-- 모바일/태블릿: 전체 화면, 데스크톱: 분할 -->
			<div class="w-full lg:hidden">
				<WriteEditor
					{title}
					{tags}
					{content}
					onTitleChange={handleTitleChange}
					onTagsChange={handleTagsChange}
					onContentChange={handleContentChange}
					onExit={handleExit}
					onSaveDraft={handleSaveDraft}
					onPublished={() => {}}
					isEditMode={true}
					editSlug={data.slug}
					editPostId={data.post.id}
					summary={data.post.summary || undefined}
					existingThumbnail={data.post.thumbnail_image}
					{isPreviewMode}
					onTogglePreviewMode={handleTogglePreviewMode}
					{htmlOutput}
				/>
			</div>

			<!-- 데스크톱: 분할 뷰 -->
			<div class="hidden lg:flex lg:flex-1 lg:overflow-hidden">
				<!-- 에디터 영역 -->
				<div style="width: {resizableHook?.leftWidth() ?? 50}%">
					<WriteEditor
						{title}
						{tags}
						{content}
						onTitleChange={handleTitleChange}
						onTagsChange={handleTagsChange}
						onContentChange={handleContentChange}
						onExit={handleExit}
						onSaveDraft={handleSaveDraft}
						onPublished={() => {}}
						isEditMode={true}
						editPostId={data.post.id}
						editSlug={data.slug}
						summary={data.post.summary || undefined}
						existingThumbnail={data.post.thumbnail_image}
						isPreviewMode={false}
						onTogglePreviewMode={undefined}
						htmlOutput=""
					/>
				</div>

				<!-- Resizer (드래그 핸들) -->
				<button
					type="button"
					aria-label={m.write_resize_handle()}
					class="bg-mofu-dark-700 w-1 flex-shrink-0 cursor-col-resize p-0 transition-colors"
					onmousedown={resizableHook?.handleMouseDown}
					class:bg-gray-400={resizableHook?.isDragging()}
				></button>

				<!-- 미리보기 영역 -->
				<div style="width: {resizableHook?.rightWidth() ?? 50}%">
					<WritePreview {title} {htmlOutput} />
				</div>
			</div>
		</div>
	</div>
{/if}
