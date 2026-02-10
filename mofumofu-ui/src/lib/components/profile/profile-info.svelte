<script lang="ts">
	import { Icon, CalendarDays } from 'svelte-hero-icons';
	import type { PublicUserProfile } from '$lib/api/types';

	interface Props {
		profile: PublicUserProfile;
	}

	const { profile }: Props = $props();

	function formatJoinDate(createdAt: string): string {
		const date = new Date(createdAt);
		return `Joined ${date.toLocaleDateString('en-US', { year: 'numeric', month: 'long' })}`;
	}
</script>

<div class="px-2 pt-14 pb-4">
	<div class="space-y-2">
		<!-- Name and handle -->
		<div>
			<h1 class="text-xl font-bold text-gray-900 dark:text-white">
				{profile.display_name}
			</h1>
			<p class="text-sm text-mofu-light-300 dark:text-mofu-dark-300">
				@{profile.handle}
			</p>
		</div>

		<!-- Bio -->
		{#if profile.bio}
			<p class="text-md whitespace-pre-wrap text-gray-700 dark:text-mofu-dark-200">
				{profile.bio}
			</p>
		{/if}

		<!-- Join date -->
		<div class="flex items-center gap-4 text-sm text-mofu-light-300 dark:text-mofu-dark-300">
			<div class="flex items-center gap-1">
				<Icon src={CalendarDays} class="h-4 w-4" />
				<span>{formatJoinDate(profile.created_at)}</span>
			</div>
		</div>

		<!-- Stats -->
		<div class="flex items-center space-x-4 text-sm">
			<div>
				<span class="font-bold text-gray-900 dark:text-white">
					{profile.following_count.toLocaleString()}
				</span>
				<span class="ml-1 text-mofu-light-300 dark:text-mofu-dark-300">Following</span>
			</div>
			<div>
				<span class="font-bold text-gray-900 dark:text-white">
					{profile.follower_count.toLocaleString()}
				</span>
				<span class="ml-1 text-mofu-light-300 dark:text-mofu-dark-300">Followers</span>
			</div>
		</div>
	</div>
</div>
