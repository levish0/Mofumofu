<script lang="ts">
	interface Props {
		displayUserName: string | null | undefined;
		displayUserImage: string | null | undefined;
	}

	let { displayUserName, displayUserImage }: Props = $props();
</script>

<div class="z-20 flex-shrink-0">
	{#if displayUserImage}
		<img alt={displayUserName} class="size-9 rounded-full object-cover" src={displayUserImage} />
	{:else}
		<div class="bg-mofu-light-700 dark:bg-mofu-dark-700 flex size-8 items-center justify-center rounded-full">
			<span class="text-mofu-light-200 dark:text-mofu-dark-200 text-sm font-medium">
				{displayUserName?.charAt(0)?.toUpperCase() || '?'}
			</span>
		</div>
	{/if}
</div>
