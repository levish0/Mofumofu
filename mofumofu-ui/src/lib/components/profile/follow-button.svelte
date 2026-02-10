<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import PillButton from '$lib/components/pill-button/pill-button.svelte';
	import { createFollow, deleteFollow, checkFollowStatus } from '$lib/api/follows';

	interface Props {
		followeeId: string;
		isAuthenticated: boolean;
		onFollowChange?: (isFollowing: boolean) => void;
	}

	const { followeeId, isAuthenticated, onFollowChange }: Props = $props();

	let isFollowing = $state(false);
	let isLoading = $state(true);
	let isSubmitting = $state(false);

	onMount(async () => {
		if (!isAuthenticated) {
			isLoading = false;
			return;
		}

		try {
			const status = await checkFollowStatus(followeeId);
			isFollowing = status.following;
		} catch (err) {
			console.warn('Failed to check follow status:', err);
		} finally {
			isLoading = false;
		}
	});

	async function handleToggle() {
		if (!isAuthenticated) {
			goto('/account/signin');
			return;
		}

		isSubmitting = true;
		try {
			if (isFollowing) {
				await deleteFollow(followeeId);
				isFollowing = false;
			} else {
				await createFollow(followeeId);
				isFollowing = true;
			}
			onFollowChange?.(isFollowing);
		} catch (err) {
			console.error('Failed to toggle follow:', err);
		} finally {
			isSubmitting = false;
		}
	}
</script>

{#if isLoading}
	<div class="h-9 w-20 animate-pulse rounded-full bg-mofu-light-700 dark:bg-mofu-dark-700"></div>
{:else}
	<PillButton
		onclick={handleToggle}
		disabled={isSubmitting}
		variant={isFollowing ? 'outline' : 'default'}
		class="px-4"
	>
		{#if isSubmitting}
			...
		{:else if !isAuthenticated}
			Follow
		{:else if isFollowing}
			Unfollow
		{:else}
			Follow
		{/if}
	</PillButton>
{/if}
