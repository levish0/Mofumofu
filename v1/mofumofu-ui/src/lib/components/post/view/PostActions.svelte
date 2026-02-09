<script lang="ts">
	import { Icon, EllipsisVertical, PencilSquare, Trash, Flag } from 'svelte-hero-icons';
	import { Button } from '$lib/components/ui/button';
	import { fly } from 'svelte/transition';

	interface Props {
		isOwner: boolean;
		onEdit: () => void;
		onDelete: () => void;
		onReport: () => void;
	}

	const { isOwner, onEdit, onDelete, onReport }: Props = $props();

	let isDropdownOpen = $state(false);
	let closeTimer: ReturnType<typeof setTimeout> | null = null;

	function openDropdown() {
		if (closeTimer) {
			clearTimeout(closeTimer);
			closeTimer = null;
		}
		isDropdownOpen = true;
	}

	function scheduleClose() {
		closeTimer = setTimeout(() => {
			isDropdownOpen = false;
			closeTimer = null;
		}, 100);
	}

	function handleEdit() {
		isDropdownOpen = false;
		onEdit();
	}

	function handleDelete() {
		isDropdownOpen = false;
		onDelete();
	}

	function handleReport() {
		isDropdownOpen = false;
		onReport();
	}
</script>

<div class="relative" role="button" tabindex="0" onmouseenter={openDropdown} onmouseleave={scheduleClose}>
	<Button variant="ghost" class="dark:text-mofu-dark-400 h-9 w-9 p-2">
		<Icon src={EllipsisVertical} class="h-5 w-5" />
	</Button>

	{#if isDropdownOpen}
		<div
			class="dark:bg-mofu-dark-800 bg-mofu-light-800 absolute top-12 right-0 z-50 w-48 rounded-lg text-sm font-bold shadow-lg"
			transition:fly={{ y: -8, duration: 150 }}
			style="transform-origin: top right;"
		>
			<div class="py-1">
				{#if isOwner}
					<button
						class="dark:text-mofu-dark-200 text-mofu-light-200 flex w-full items-center px-4 py-2 hover:opacity-70"
						onclick={handleEdit}
					>
						<Icon src={PencilSquare} solid size="16" class="mr-3" />
						수정하기
					</button>
					<button
						class="flex w-full items-center px-4 py-2 text-rose-600 hover:opacity-70 dark:text-rose-500"
						onclick={handleDelete}
					>
						<Icon src={Trash} solid size="16" class="mr-3" />
						삭제하기
					</button>
				{/if}
				<button
					class="dark:text-mofu-dark-200 text-mofu-light-200 flex w-full items-center px-4 py-2 hover:opacity-70"
					onclick={handleReport}
				>
					<Icon src={Flag} solid size="16" class="mr-3" />
					신고하기
				</button>
			</div>
		</div>
	{/if}
</div>
