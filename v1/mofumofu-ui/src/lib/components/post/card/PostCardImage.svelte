<script lang="ts">
	const {
		image = undefined,
		title = '',
		isSkeleton = false
	}: {
		image?: string;
		title: string;
		isSkeleton?: boolean;
	} = $props();

	let imageLoaded = $state(false);
	let imageError = $state(false);

	$effect(() => {
		if (image) {
			imageLoaded = false;
			imageError = false;
		}
	});
</script>

<div class="relative w-full overflow-hidden pt-[56.25%]">
	{#if isSkeleton}
		<div class="shimmer absolute inset-0"></div>
	{:else if image}
		{#if !imageLoaded && !imageError}
			<div class="shimmer absolute inset-0"></div>
		{/if}

		{#if imageError}
			<!-- Fallback when image fails to load -->
			<div class="bg-mofu-dark-700 absolute inset-0 flex items-center justify-center">
				<div class="text-mofu-dark-300 text-center">
					<svg class="mx-auto mb-2 h-12 w-12" fill="currentColor" viewBox="0 0 24 24">
						<path
							d="M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z"
						/>
					</svg>
					<p class="text-xs">이미지를 불러올 수 없습니다</p>
				</div>
			</div>
		{:else}
			<img
				src={image}
				alt={title}
				loading="lazy"
				class="absolute inset-0 h-full w-full object-cover transition-opacity duration-300"
				class:opacity-0={!imageLoaded}
				class:opacity-100={imageLoaded}
				onload={() => (imageLoaded = true)}
				onerror={() => {
					imageError = true;
					imageLoaded = true;
				}}
			/>
		{/if}
	{/if}
</div>
