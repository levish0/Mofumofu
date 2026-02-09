<script lang="ts">
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import { checkAdminStatus } from '$lib/api/admin';
	import LoadingOverlay from '$lib/components/common/LoadingOverlay.svelte';

	let { children } = $props();
	let isLoading = $state(true);
	let hasAdminAccess = $state(false);

	onMount(async () => {
		try {
			// 로그인 여부 확인
			if (!authStore.isAuthenticated) {
				const refreshSuccess = await authStore.tryRefreshToken();
				if (!refreshSuccess) {
					goto('/account/signin');
					return;
				}
			}

			// 관리자 권한 확인
			const adminStatusResult = await checkAdminStatus();
			if (adminStatusResult.is_admin) {
				hasAdminAccess = true;
			} else {
				// 관리자 권한이 없으면 메인으로 리다이렉트
				goto('/');
				return;
			}
		} catch (error) {
			console.error('Admin access check failed:', error);
			goto('/');
		} finally {
			isLoading = false;
		}
	});
</script>

<LoadingOverlay isVisible={isLoading} message="관리자 권한을 확인하는 중..." />

{#if !isLoading && hasAdminAccess}
	<div class="max-w-8xl mx-auto pb-40">
		<div class="px-4 pt-2">{@render children()}</div>
	</div>
{/if}
