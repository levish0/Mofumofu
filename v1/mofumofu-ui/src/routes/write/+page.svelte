<script lang="ts">
	import '$lib/styles/markdown.css';
	import { useResizable } from '$lib/hooks/ui/useResizable.svelte';
	import WriteEditor from '$lib/components/write/WriteEditor.svelte';
	import WritePreview from '$lib/components/write/WritePreview.svelte';
	import { processMarkdown } from '$lib/utils/markdown';
	import * as m from '../../paraglide/messages';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import { goto } from '$app/navigation';
	import { onMount, onDestroy } from 'svelte';
	import LoadingOverlay from '$lib/components/common/LoadingOverlay.svelte';
	import AuthErrorScreen from '$lib/components/common/AuthErrorScreen.svelte';
	import NotVerifiedScreen from '$lib/components/common/NotVerifiedScreen.svelte';
	import { createDraft, updateDraft, generateDraftSlug, getDraft, type DraftInfo } from '$lib/api/draft';
	import { toast } from 'svelte-sonner';
	import { page } from '$app/state';

	let title = $state('');
	let tags = $state('');
	let content = $state('');
	let htmlOutput = $state('');
	let containerElement: HTMLElement | undefined = $state();
	let isPreviewMode = $state(false); // 모바일에서 프리뷰 모드인지
	let isAuthChecking = $state(true); // 인증 체크 중인지
	let authError = $state(false); // 인증 실패 상태
	let verificationError = $state(false); // 이메일 인증 실패 상태

	// Draft 관련 상태
	let currentDraftId = $state<string | null>(null);
	let lastSaveTime = $state<Date | null>(null);
	let autoSaveInterval: ReturnType<typeof setInterval> | null = null;

	// URL에서 draft ID 가져오기
	const draftIdFromUrl = $derived(page.url.searchParams.get('draft'));

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

	async function handleSaveDraft(isAutoSave = false) {
		try {
			if (currentDraftId) {
				// 기존 draft 업데이트
				await updateDraft({
					draft_id: currentDraftId,
					title: title || null,
					content: content || null,
					summary: tags || null
				});
				lastSaveTime = new Date();
				if (!isAutoSave) {
					toast.success('초안이 저장되었습니다.');
				}
			} else {
				// 새 draft 생성
				const slug = generateDraftSlug(title);
				const newDraft = await createDraft({
					slug,
					title: title || null,
					content: content || null,
					summary: tags || null
				});
				currentDraftId = newDraft.draft_id;
				lastSaveTime = new Date();
				if (isAutoSave) {
					toast.success('초안이 자동저장되었습니다.', {
						duration: 2000
					});
				} else {
					toast.success('초안이 저장되었습니다.');
				}
			}
		} catch (error: any) {
			console.error('Failed to save draft:', error);

			// Draft 개수 제한 에러 처리
			if (error?.response?.status === 400) {
				const responseText = await error.response.text();
				if (responseText.includes('draft:limit_exceeded')) {
					toast.error('초안은 최대 10개까지만 저장할 수 있습니다. 기존 초안을 삭제한 후 다시 시도해주세요.');
					return;
				}
			}

			// Slug 중복 에러 처리
			if (error?.response?.status === 409) {
				const responseText = await error.response.text();
				if (responseText.includes('draft:slug_already_exists')) {
					toast.error('같은 이름의 초안이 이미 존재합니다.');
					return;
				}
			}

			// 기타 에러
			if (isAutoSave) {
				toast.error('자동저장에 실패했습니다.', {
					duration: 3000
				});
			} else {
				toast.error('초안 저장에 실패했습니다. 다시 시도해주세요.');
			}
		}
	}

	// 5분마다 자동 저장
	function startAutoSave() {
		if (autoSaveInterval) clearInterval(autoSaveInterval);

		autoSaveInterval = setInterval(
			async () => {
				// 내용이 있을 때만 자동 저장
				if (title.trim() || content.trim()) {
					await handleSaveDraft(true); // isAutoSave = true
				}
			},
			5 * 60 * 1000
		); // 5분
	}

	function stopAutoSave() {
		if (autoSaveInterval) {
			clearInterval(autoSaveInterval);
			autoSaveInterval = null;
		}
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

			// 자동 저장 시작
			startAutoSave();

			// URL에 draft ID가 있으면 로드
			if (draftIdFromUrl) {
				await loadDraft(draftIdFromUrl);
			}
		} finally {
			isAuthChecking = false;
		}
	});

	// 컴포넌트 정리
	onDestroy(() => {
		stopAutoSave();
	});

	// Draft 로드하는 함수
	async function loadDraft(draftId: string) {
		try {
			const draft = await getDraft({ draft_id: draftId });

			// Draft 데이터를 에디터에 로드
			title = draft.title || '';
			content = draft.content || '';
			tags = draft.summary || '';
			currentDraftId = draft.draft_id;

			toast.success('초안이 로드되었습니다.');
		} catch (error) {
			console.error('Failed to load draft:', error);
			toast.error('초안을 불러오는데 실패했습니다.');
		}
	}
</script>

<svelte:head>
	<title>{m.write_page_title()}</title>
	<meta name="description" content={m.write_page_description()} />
	<meta name="robots" content="noindex, nofollow" />

	<!-- Open Graph -->
	<meta property="og:title" content={m.write_page_title()} />
	<meta property="og:description" content={m.write_page_description()} />
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content="Mofu" />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary" />
	<meta name="twitter:title" content={m.write_page_title()} />
	<meta name="twitter:description" content={m.write_page_description()} />
</svelte:head>

<LoadingOverlay isVisible={isAuthChecking} message={m.write_preparing_editor()} />

<AuthErrorScreen isVisible={authError} description={m.write_auth_required()} />

<NotVerifiedScreen isVisible={verificationError} description={m.write_email_verification_required()} />

{#if !authError && !verificationError}
	<div class="dark:bg-mofu-dark-900 bg-mofu-light-900 flex h-full w-full break-all text-black dark:text-white">
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
					onSaveDraft={() => handleSaveDraft(false)}
					onPublished={() => {}}
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
						onSaveDraft={() => handleSaveDraft(false)}
						onPublished={() => {}}
						isPreviewMode={false}
						onTogglePreviewMode={undefined}
						htmlOutput=""
					/>
				</div>

				<!-- Resizer (드래그 핸들) -->
				<button
					type="button"
					aria-label={m.write_resize_handle()}
					class="dark:bg-mofu-dark-700 bg-mofu-light-700 w-1 flex-shrink-0 cursor-col-resize p-0 transition-colors"
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
