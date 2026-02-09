<script lang="ts">
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth.svelte.js';
	import { Button } from '../ui/button';
	import type { UserInfoResponse } from '$lib/api/user/types';
	import FollowButton from './FollowButton.svelte';
	import * as m from '../../../paraglide/messages';

	type Props = {
		profile: UserInfoResponse;
		isOwnProfile: boolean;
		isLoading: boolean;
		topPosition: string;
	};

	const { profile, isOwnProfile, isLoading, topPosition }: Props = $props();

	let bannerLoaded = $state(false);
	let profileImageLoaded = $state(false);

	function handleEditProfile() {
		goto('/settings');
	}

	// Reset loading states when image URLs change
	$effect(() => {
		if (profile.banner_image) {
			bannerLoaded = false;
		}
	});

	$effect(() => {
		if (profile.profile_image) {
			profileImageLoaded = false;
		}
	});
</script>

<div class="transition-all duration-100 ease-out">
	<!-- Banner Section -->
	<div class="relative aspect-[3/1] w-full">
		{#if profile.banner_image}
			<img
				src={profile.banner_image}
				alt="Banner"
				class="absolute inset-0 h-full w-full overflow-hidden rounded-xl object-cover"
				loading="lazy"
				onload={() => (bannerLoaded = true)}
			/>
		{:else}
			<div class="h-full w-full overflow-hidden rounded-xl bg-gradient-to-r from-blue-400 to-purple-500"></div>
		{/if}

		<!-- Action Button (next to profile image) -->
		<div class="absolute right-4 -bottom-12 z-20">
			{#if isLoading}
				<div class="shimmer h-10 w-20 rounded-full"></div>
			{:else if isOwnProfile}
				<Button variant="outline" onclick={handleEditProfile} class=" bg-transparent px-3 py-0">Edit Profile</Button>
			{:else}
				<FollowButton handle={profile.handle} />
			{/if}
		</div>

		<!-- Profile Image (overlapping banner) -->
		<div class="absolute -bottom-12 left-4 z-10">
			<div class="relative h-24 w-24">
				{#if profile.profile_image}
					<img
						src={profile.profile_image}
						alt={profile.name}
						class="dark:border-mofu-dark-900 dark:bg-mofu-dark-900 bg-mofu-light-100 border-mofu-light-100 absolute inset-0 h-24 w-24 rounded-full border-4 object-cover"
						loading="lazy"
						onload={() => (profileImageLoaded = true)}
					/>
				{:else}
					<div
						class="dark:border-mofu-dark-900 dark:bg-mofu-dark-700 bg-mofu-light-700 border-mofu-light-100 flex h-24 w-24 items-center justify-center rounded-full border-4"
					>
						<span class="text-2xl font-medium text-black dark:text-white">
							{profile.name.charAt(0).toUpperCase()}
						</span>
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
