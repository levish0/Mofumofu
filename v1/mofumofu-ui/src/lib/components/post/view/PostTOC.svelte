<script lang="ts">
	interface TOCItem {
		id: string;
		text: string;
		level: number;
	}

	interface Props {
		tocItems: TOCItem[];
		topPosition: string;
	}

	const { tocItems, topPosition }: Props = $props();
</script>

<div class="border-mofu-light-800 dark:border-mofu-dark-800 hidden w-80 flex-shrink-0 border-l md:block">
	<div class="sticky transition-all duration-100 ease-out" style="top: {topPosition}">
		<div class="px-4">
			<h3 class="dark:text-mofu-dark-200 border-mofu-light-200 mb-2 text-xl font-semibold">목차</h3>
			<nav class="max-h-[80vh] space-y-2 overflow-y-auto">
				{#each tocItems as item}
					<a
						href="#{item.id}"
						class="dark:text-mofu-dark-300 text-mofu-light-300 block text-sm transition-colors hover:text-black dark:hover:text-white"
						style="padding-left: {(item.level - 1) * 12}px"
					>
						{item.text}
					</a>
				{/each}
			</nav>
		</div>
	</div>
</div>
