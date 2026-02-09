<script lang="ts">
	import { Bars3 } from 'svelte-hero-icons';
	import { Icon } from 'svelte-hero-icons';
	import * as Drawer from '$lib/components/ui/drawer';
	import { buttonVariants } from '$lib/components/ui/button';

	type TOCItem = {
		id: string;
		text: string;
		level: number;
	};

	type Props = {
		tocItems: TOCItem[];
	};

	const { tocItems }: Props = $props();
	let open = $state(false);

	function handleTOCClick() {
		open = false;
	}
</script>

<!-- 플로팅 목차 버튼 (모바일에서만 표시) -->
<div class="fixed right-4 bottom-32 z-50 md:hidden">
	<Drawer.Root bind:open>
		<Drawer.Trigger
			class="dark:bg-mofu-dark-900 flex h-12 w-12 items-center justify-center rounded-full bg-white opacity-70 shadow-lg transition-all duration-300 hover:scale-105 hover:shadow-xl"
			aria-label="목차 보기"
		>
			<Icon src={Bars3} class="h-6 w-6 text-gray-700 dark:text-gray-300" />
		</Drawer.Trigger>

		<Drawer.Content class="dark:bg-mofu-dark-900 max-h-[70vh] bg-white">
			<Drawer.Header>
				<Drawer.Title class="text-left">목차</Drawer.Title>
			</Drawer.Header>

			<div class="max-h-[50vh] overflow-y-auto px-4 pb-4">
				<nav class="space-y-2">
					{#each tocItems as item}
						<a
							href="#{item.id}"
							onclick={handleTOCClick}
							class=" dark:text-mofu-dark-200 block py-2 text-sm text-gray-600 transition-colors hover:text-gray-900 dark:hover:text-white"
							style="padding-left: {(item.level - 1) * 16}px"
						>
							{item.text}
						</a>
					{/each}
				</nav>
			</div>
		</Drawer.Content>
	</Drawer.Root>
</div>
