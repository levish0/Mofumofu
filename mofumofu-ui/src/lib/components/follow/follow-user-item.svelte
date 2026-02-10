<script lang="ts">
	import type { FollowUserItem } from '$lib/api/types';

	interface Props {
		user: FollowUserItem;
	}

	const { user }: Props = $props();
</script>

<a
	href="/@{user.handle}"
	class="flex items-center gap-3 rounded-xl px-3 py-3 transition-colors hover:bg-mofu-light-800 dark:hover:bg-mofu-dark-800"
>
	{#if user.profile_image}
		<img
			src={user.profile_image}
			alt={user.display_name}
			class="h-11 w-11 rounded-full object-cover"
			loading="lazy"
		/>
	{:else}
		<div
			class="flex h-11 w-11 items-center justify-center rounded-full bg-mofu-light-700 dark:bg-mofu-dark-700"
		>
			<span class="text-sm font-medium text-black dark:text-white">
				{user.display_name.charAt(0).toUpperCase()}
			</span>
		</div>
	{/if}
	<div class="min-w-0 flex-1">
		<p class="truncate font-bold text-gray-900 dark:text-white">{user.display_name}</p>
		<p class="truncate text-sm text-mofu-light-300 dark:text-mofu-dark-300">@{user.handle}</p>
	</div>
</a>
