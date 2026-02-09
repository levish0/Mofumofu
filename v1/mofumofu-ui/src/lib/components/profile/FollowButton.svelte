<script lang="ts">
	import { goto } from '$app/navigation';
	import { Button } from '../ui/button';
	import { createFollow, deleteFollow, checkFollowStatus } from '$lib/api/follow/followApi';
	import { authStore } from '$lib/stores/auth.svelte';
	import * as m from '../../../paraglide/messages';

	interface Props {
		handle: string;
		onFollowChange?: (isFollowing: boolean) => void;
	}

	const { handle, onFollowChange }: Props = $props();

	let isFollowing = $state(false);
	let followLoading = $state(false);
	let followStatusLoaded = $state(false);

	// 팔로우 상태 확인
	async function loadFollowStatus() {
		if (!authStore.isAuthenticated) {
			followStatusLoaded = true;
			return;
		}

		try {
			const response = await checkFollowStatus({ handle });
			isFollowing = response.is_following;
		} catch (error) {
			console.error('Failed to check follow status:', error);
		} finally {
			followStatusLoaded = true;
		}
	}

	// 팔로우/언팔로우 토글
	async function handleFollowToggle() {
		if (!authStore.isAuthenticated) {
			goto('/account/signin');
			return;
		}

		followLoading = true;
		try {
			if (isFollowing) {
				await deleteFollow({ followee_handle: handle });
				isFollowing = false;
			} else {
				await createFollow({ followee_handle: handle });
				isFollowing = true;
			}

			// 부모 컴포넌트에 상태 변경 알림
			onFollowChange?.(isFollowing);
		} catch (error) {
			console.error('Failed to toggle follow:', error);
		} finally {
			followLoading = false;
		}
	}

	// 컴포넌트 마운트 시 팔로우 상태 로드
	$effect(() => {
		loadFollowStatus();
	});
</script>

{#if !followStatusLoaded}
	<div class="shimmer h-10 w-20 rounded-full"></div>
{:else}
	<Button
		onclick={handleFollowToggle}
		disabled={followLoading}
		variant={isFollowing ? 'outline' : 'default'}
		class="px-3 py-0 {isFollowing ? 'bg-transparent' : 'dark:text-mofu-dark-900'}"
	>
		{#if followLoading}
			...
		{:else if !authStore.isAuthenticated}
			{m.profile_sign_in_to_follow()}
		{:else if isFollowing}
			{m.profile_unfollow()}
		{:else}
			{m.profile_follow()}
		{/if}
	</Button>
{/if}
