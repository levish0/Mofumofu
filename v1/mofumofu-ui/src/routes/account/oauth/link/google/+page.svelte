<script lang="ts">
	import { goto } from '$app/navigation';
	import { linkOAuth } from '$lib/api/auth/authApi';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';
	import { ApiError } from '$lib/api/error/common_error';
	import { ExclamationTriangle, Icon } from 'svelte-hero-icons';
	import * as m from '../../../../../paraglide/messages';
	import { Button } from '$lib/components/ui/button';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	let loading = $state(true);
	let error = $state<string | null>(null);

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

			await linkOAuth('Google', code);
			toast.success('Google 계정이 성공적으로 연결되었습니다.');
			await goto('/settings', { replaceState: true });
		} catch (err) {
			console.error('Google OAuth link error:', err);

			if (err instanceof ApiError) {
				error = `연결 실패: ${err.message}`;
			} else if (err instanceof Error) {
				error = err.message;
			} else {
				error = 'Google 계정 연결 중 예기치 않은 오류가 발생했습니다.';
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
					<h2 class="text-xl font-semibold">Google 계정 연결 중...</h2>
					<p>잠시만 기다려주세요.</p>
				</div>
			{:else if error}
				<div class="space-y-4">
					<div class="text-rose-600">
						<Icon src={ExclamationTriangle} solid size="40" class="inline-block" />
					</div>
					<h2 class="text-xl font-semibold">오류가 발생했습니다</h2>
					<p class="text-sm">{error}</p>
					<Button
						onclick={() => goto('/settings')}
						variant="ghost"
						class="dark:text-mofu-dark-300 h-fit rounded-md py-0 text-sm hover:opacity-70"
					>
						← 설정으로 돌아가기
					</Button>
				</div>
			{/if}
		</div>
	</div>
</div>
