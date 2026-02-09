<script lang="ts">
	interface Props {
		displayUserName: string | null | undefined;
		displayUserHandle: string | null | undefined;
		displayUserImage: string | null | undefined;
		createdAt: string;
		updatedAt: string | null | undefined;
		isDeleted: boolean;
	}

	let { displayUserName, displayUserHandle, displayUserImage, createdAt, updatedAt, isDeleted }: Props = $props();

	const getRelativeTime = (dateString: string) => {
		const date = new Date(dateString);
		const now = new Date();
		const diffInMinutes = Math.floor((now.getTime() - date.getTime()) / (1000 * 60));

		if (diffInMinutes < 1) return '방금 전';
		if (diffInMinutes < 60) return `${diffInMinutes}분 전`;

		const diffInHours = Math.floor(diffInMinutes / 60);
		if (diffInHours < 24) return `${diffInHours}시간 전`;

		const diffInDays = Math.floor(diffInHours / 24);
		if (diffInDays < 7) return `${diffInDays}일 전`;

		return date.toLocaleDateString();
	};
</script>

<div class="flex items-center gap-2 text-sm">
	{#if !isDeleted}
		<a
			href="/@{displayUserHandle}/profile"
			class="text-mofu-light-200 dark:text-mofu-dark-200 font-medium hover:underline"
		>
			{displayUserName}
		</a>
	{:else}
		<span class="text-mofu-light-400 dark:text-mofu-dark-400 font-medium">{displayUserName}</span>
	{/if}
	<span class="text-mofu-light-400 dark:text-mofu-dark-400">•</span>
	<span class="text-mofu-light-400 dark:text-mofu-dark-400">{getRelativeTime(createdAt)}</span>
	{#if updatedAt && updatedAt !== createdAt}
		<span class="text-mofu-light-500 dark:text-mofu-dark-500 text-xs">
			• {getRelativeTime(updatedAt)} 편집
		</span>
	{/if}
</div>
