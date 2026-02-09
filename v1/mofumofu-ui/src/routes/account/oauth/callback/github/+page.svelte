<!-- src/routes/accounts/oauth/callback/github/+page.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { githubAuth, linkOAuth } from '$lib/api/auth/authApi';
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth.svelte';
	import { oauthHandleStore } from '$lib/stores/oauthHandle.svelte';
	import { ApiError } from '$lib/api/error/common_error';
	import { ExclamationTriangle, Icon } from 'svelte-hero-icons';
	import * as m from '../../../../../paraglide/messages';
	import { Button } from '$lib/components/ui/button';
	import { toast } from 'svelte-sonner';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	let loading = $state(true);
	let error = $state<string | null>(null);
	let isLinkRequest = $state(false);

	onMount(async () => {
		try {
			// Check if server detected an error
			if (data.error) {
				throw new Error(data.error);
			}

			// Get code from server data (already verified)
			const code = data.code;
			if (!code) {
				throw new Error('Authorization code not found');
			}

			// Get link request status from server data
			isLinkRequest = data.isLinkRequest ?? false;

			if (isLinkRequest) {
				// 계정 연결 처리
				await linkOAuth('Github', code);
				toast.success('GitHub 계정이 성공적으로 연결되었습니다.');
				await goto('/settings', { replaceState: true });
			} else {
				// 기존 로그인/가입 처리
				// Use handle from server data (already extracted from cookie)
				const handle = data.handle;
				console.log('Handle from server:', handle);

				// GitHub OAuth 처리 (핸들이 있으면 가입, 없으면 로그인)
				const response = await githubAuth(code, handle || undefined);

				// 토큰을 스토어에 저장
				authStore.setToken(response.access_token);

				// 핸들 스토어 정리 (서버에서 이미 쿠키를 정리했지만 클라이언트 스토어도 정리)
				if (handle) {
					oauthHandleStore.clearHandle();
				}

				// 성공 시 메인 페이지로 리다이렉트
				await goto('/', { replaceState: true });
			}
		} catch (err) {
			console.error('GitHub OAuth error:', err);

			if (err instanceof ApiError) {
				error = `처리 실패: ${err.message}`;
			} else if (err instanceof Error) {
				error = err.message;
			} else {
				error = 'GitHub 처리 중 예기치 않은 오류가 발생했습니다.';
			}
		} finally {
			loading = false;
		}
	});
</script>

<div class="text-mofu-dark-200 bg-mofu-dark-900 flex min-h-screen items-center justify-center">
	<div class="w-full max-w-md space-y-8 p-8">
		<div class="text-center">
			{#if loading}
				<div class="space-y-4">
					<div class="border-mofu-dark-100 mx-auto h-12 w-12 animate-spin rounded-full border-b-2"></div>
					<h2 class="text-xl font-semibold">
						{isLinkRequest ? 'GitHub 계정 연결 중...' : m.oauth_processing_github()}
					</h2>
					<p>{m.oauth_please_wait()}</p>
				</div>
			{:else if error}
				<div class="space-y-4">
					<div class="text-rose-600">
						<Icon src={ExclamationTriangle} solid size="40" class="inline-block" />
					</div>
					<h2 class="text-xl font-semibold">{m.oauth_error_occurred()}</h2>
					<p class="text-sm">{error}</p>
					<Button
						onclick={() => goto(isLinkRequest ? '/settings' : '/account/signin')}
						variant="ghost"
						class="dark:text-mofu-dark-300 rounded-md text-sm hover:opacity-70"
					>
						← {isLinkRequest ? '설정으로 돌아가기' : m.oauth_go_back()}
					</Button>
				</div>
			{/if}
		</div>
	</div>
</div>
