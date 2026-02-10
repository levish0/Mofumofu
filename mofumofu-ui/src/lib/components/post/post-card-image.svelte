<script lang="ts">
	let {
		image,
		title,
		isSkeleton = false
	}: { image?: string | null; title: string; isSkeleton?: boolean } = $props();

	let loaded = $state(false);
	let error = $state(false);
</script>

{#if isSkeleton}
	<div class="relative w-full pt-[56.25%]">
		<div class="shimmer absolute inset-0 rounded-t-xl"></div>
	</div>
{:else if image}
	<div class="relative w-full overflow-hidden pt-[56.25%]">
		{#if !loaded}
			<div class="shimmer absolute inset-0"></div>
		{/if}
		<img
			src={image}
			alt={title}
			loading="lazy"
			class="absolute inset-0 h-full w-full rounded-t-xl object-cover transition-opacity duration-300"
			class:opacity-0={!loaded}
			class:opacity-100={loaded}
			onload={() => (loaded = true)}
			onerror={() => (error = true)}
		/>
		{#if error}
			<div
				class="absolute inset-0 flex items-center justify-center bg-mofu-light-700 dark:bg-mofu-dark-700"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-10 w-10 text-mofu-light-500 dark:text-mofu-dark-400"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="1.5"
						d="m2.25 15.75 5.159-5.159a2.25 2.25 0 0 1 3.182 0l5.159 5.159m-1.5-1.5 1.409-1.409a2.25 2.25 0 0 1 3.182 0l2.909 2.909M3.75 21h16.5A2.25 2.25 0 0 0 22.5 18.75V5.25A2.25 2.25 0 0 0 20.25 3H3.75A2.25 2.25 0 0 0 1.5 5.25v13.5A2.25 2.25 0 0 0 3.75 21Z"
					/>
				</svg>
			</div>
		{/if}
	</div>
{/if}
