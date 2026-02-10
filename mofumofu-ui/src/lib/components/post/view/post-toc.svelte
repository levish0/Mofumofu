<script lang="ts">
	import type { TocItem } from '$lib/api/types';

	interface Props {
		tocItems: TocItem[];
		topPosition: string;
	}

	const { tocItems, topPosition }: Props = $props();
</script>

<aside class="hidden xl:block">
	<div class="sticky transition-all duration-100 ease-out" style="top: {topPosition}">
		<div class="border-l border-mofu-light-800 px-4 dark:border-mofu-dark-800">
			<h3 class="mb-2 text-lg font-semibold text-mofu-light-200 dark:text-mofu-dark-200">
				Table of Contents
			</h3>
			<nav class="max-h-[80vh] space-y-2 overflow-y-auto">
				{#each tocItems as item}
					<a
						href="#{item.id}"
						class="block text-sm text-mofu-light-300 transition-colors hover:text-black dark:text-mofu-dark-300 dark:hover:text-white"
						style="padding-left: {(item.level - 1) * 12}px"
					>
						{item.text}
					</a>
				{/each}
			</nav>
		</div>
	</div>
</aside>
