<script lang="ts">
	import {
		Cog6Tooth,
		MagnifyingGlass,
		ArrowPath,
		Trash,
		ChartBar,
		Server,
		UserGroup,
		Heart,
		Clock,
		Icon
	} from 'svelte-hero-icons';
	import { onMount } from 'svelte';
	import * as admin from '$lib/api/admin';
	import LoadingOverlay from '$lib/components/common/LoadingOverlay.svelte';

	let isLoading = $state(false);
	let lastAction = $state('');
	let actionResult = $state<{ success: boolean; message: string } | null>(null);

	// 각 섹션별 상태
	let taskResults = $state<{ [key: string]: any }>({});

	const sections = [
		{
			id: 'cleanup',
			title: '데이터 정리',
			description: '시스템 데이터 정리 및 최적화',
			icon: Trash,
			actions: [
				{
					id: 'cleanup-events',
					title: '이벤트 정리',
					description: '30일 이상된 시스템 이벤트 정리',
					icon: Clock,
					action: admin.cleanupOldEvents,
					showResult: true
				},
				{
					id: 'cleanup-tokens',
					title: '토큰 정리',
					description: '만료된 리프레시 토큰 정리',
					icon: Trash,
					action: admin.cleanupExpiredTokens,
					showResult: true
				}
			]
		},
		{
			id: 'search',
			title: '검색 관리',
			description: 'Meilisearch 색인 관리',
			icon: MagnifyingGlass,
			actions: [
				{
					id: 'search-health',
					title: '헬스체크',
					description: 'Meilisearch 서버 상태 확인',
					icon: Server,
					action: admin.checkSearchHealth,
					showResult: true
				},
				{
					id: 'search-reindex',
					title: '전체 재색인',
					description: '모든 포스트 재색인',
					icon: ArrowPath,
					action: admin.reindexAllPosts,
					showResult: true
				},
				{
					id: 'search-stats',
					title: '색인 통계',
					description: '검색 색인 통계 조회',
					icon: ChartBar,
					action: admin.getSearchStats,
					showResult: true
				}
			]
		},
		{
			id: 'sync',
			title: '데이터 동기화',
			description: '카운트 데이터 동기화',
			icon: ArrowPath,
			actions: [
				{
					id: 'sync-all',
					title: '전체 동기화',
					description: '모든 카운트 동기화',
					icon: Cog6Tooth,
					action: admin.syncAllCounts,
					showResult: true
				},
				{
					id: 'sync-follows',
					title: '팔로우 동기화',
					description: '팔로워/팔로잉 수 동기화',
					icon: UserGroup,
					action: admin.syncFollows,
					showResult: true
				},
				{
					id: 'sync-likes',
					title: '좋아요 동기화',
					description: '포스트 좋아요 수 동기화',
					icon: Heart,
					action: admin.syncLikes,
					showResult: true
				}
			]
		}
	];

	async function executeAction(actionId: string, actionFn: () => Promise<any>, showResult = false) {
		isLoading = true;
		lastAction = actionId;
		actionResult = null;

		try {
			const result = await actionFn();

			if (showResult && result) {
				// 결과를 보여주는 액션의 경우
				taskResults[actionId] = result;
				actionResult = {
					success: true,
					message: result.message || '작업이 완료되었습니다.'
				};
			} else {
				// 단순 실행 액션의 경우
				actionResult = {
					success: true,
					message: '작업이 성공적으로 완료되었습니다.'
				};
			}
		} catch (error: any) {
			console.error(`Action ${actionId} failed:`, error);
			actionResult = {
				success: false,
				message: error.message || '작업 실행 중 오류가 발생했습니다.'
			};
		} finally {
			isLoading = false;
		}
	}

	// 결과 자동 초기화
	$effect(() => {
		if (actionResult) {
			const timer = setTimeout(() => {
				actionResult = null;
			}, 5000);
			return () => clearTimeout(timer);
		}
	});
</script>

<svelte:head>
	<title>관리자 패널 - Mofumofu</title>
	<meta name="description" content="Mofumofu 관리자 도구" />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<LoadingOverlay isVisible={isLoading} message="작업을 실행하는 중..." />

<div class="min-h-screen">
	<div class="mx-auto max-w-7xl px-4 py-8">
		<!-- 헤더 -->
		<div class="mb-8">
			<h1 class="text-mofu-light-100 dark:text-mofu-dark-100 text-3xl font-bold">관리자 패널</h1>
			<p class="text-mofu-light-400 dark:text-mofu-dark-400 mt-2">시스템 관리 및 유지보수 도구</p>
		</div>

		<!-- 결과 알림 -->
		{#if actionResult}
			<div
				class="mb-6 rounded-lg p-4 {actionResult.success
					? 'border border-green-200 bg-green-50 dark:border-green-800 dark:bg-green-900/20'
					: 'border border-red-200 bg-red-50 dark:border-red-800 dark:bg-red-900/20'}"
			>
				<div class="flex items-center">
					<div class="flex-shrink-0">
						{#if actionResult.success}
							<svg class="h-5 w-5 text-green-400" viewBox="0 0 20 20" fill="currentColor">
								<path
									fill-rule="evenodd"
									d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
									clip-rule="evenodd"
								/>
							</svg>
						{:else}
							<svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
								<path
									fill-rule="evenodd"
									d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
									clip-rule="evenodd"
								/>
							</svg>
						{/if}
					</div>
					<div class="ml-3">
						<p
							class="text-sm font-medium {actionResult.success
								? 'text-green-800 dark:text-green-300'
								: 'text-red-800 dark:text-red-300'}"
						>
							{actionResult.message}
						</p>
					</div>
				</div>
			</div>
		{/if}

		<!-- 섹션들 -->
		<div class="grid gap-6 md:grid-cols-1 lg:grid-cols-1">
			{#each sections as section}
				<div class="bg-mofu-light-900 dark:bg-mofu-dark-800 rounded-lg p-6 shadow">
					<div class="mb-4 flex items-center">
						<div class="bg-mofu-light-800 dark:bg-mofu-dark-700 mr-3 rounded-lg p-2">
							<Icon src={section.icon} size="24" class="text-mofu dark:text-mofu" />
						</div>
						<div>
							<h3 class="text-mofu-light-100 dark:text-mofu-dark-100 text-lg font-semibold">{section.title}</h3>
							<p class="text-mofu-light-500 dark:text-mofu-dark-400 text-sm">{section.description}</p>
						</div>
					</div>

					<div class="grid gap-3 md:grid-cols-2 lg:grid-cols-3">
						{#each section.actions as action}
							<button
								class="border-mofu-light-700 bg-mofu-light-800 hover:bg-mofu-light-700 dark:border-mofu-dark-600 dark:bg-mofu-dark-700 dark:hover:bg-mofu-dark-600 flex items-center rounded-lg border p-4 text-left transition-colors disabled:opacity-50"
								disabled={isLoading}
								onclick={() => executeAction(action.id, action.action, action.showResult)}
							>
								<div class="bg-mofu-light-700 dark:bg-mofu-dark-600 mr-3 rounded-lg p-2">
									<Icon src={action.icon} size="20" class="text-mofu-light-400 dark:text-mofu-dark-300" />
								</div>
								<div class="flex-1">
									<h4 class="text-mofu-light-100 dark:text-mofu-dark-100 text-sm font-medium">{action.title}</h4>
									<p class="text-mofu-light-500 dark:text-mofu-dark-400 text-xs">{action.description}</p>
								</div>
							</button>
						{/each}
					</div>

					<!-- 작업 결과 표시 -->
					{#each section.actions as action}
						{#if action.showResult && taskResults[action.id]}
							<div class="bg-mofu-light-850 dark:bg-mofu-dark-750 mt-4 rounded-lg p-4">
								<h4 class="text-mofu-light-100 dark:text-mofu-dark-100 mb-2 text-sm font-medium">
									{action.title} 결과
								</h4>
								<div class="text-mofu-light-400 dark:text-mofu-dark-300 text-xs">
									<p class="mb-2"><strong>메시지:</strong> {taskResults[action.id].message}</p>
									{#if taskResults[action.id].data}
										<details class="mt-2">
											<summary class="text-mofu cursor-pointer transition-colors hover:opacity-70"
												>상세 데이터 보기</summary
											>
											<pre class="mt-2 overflow-auto whitespace-pre-wrap">{JSON.stringify(
													taskResults[action.id].data,
													null,
													2
												)}</pre>
										</details>
									{/if}
								</div>
							</div>
						{/if}
					{/each}
				</div>
			{/each}
		</div>
	</div>
</div>
