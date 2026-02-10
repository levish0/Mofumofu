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

<div
	class="relative"
	role="button"
	tabindex="0"
	onmouseenter={openDropdown}
	onmouseleave={scheduleClose}
>
	<Button variant="ghost" class="h-9 w-9 p-2 text-mofu-light-400 dark:text-mofu-dark-400">
		<Icon src={EllipsisVertical} class="h-5 w-5" />
	</Button>

	{#if isDropdownOpen}
		<div
			class="absolute top-12 right-0 z-50 w-48 rounded-lg bg-mofu-light-800 text-sm font-bold shadow-lg dark:bg-mofu-dark-800"
			transition:fly={{ y: -8, duration: 150 }}
		>
			<div class="py-1">
				{#if isOwner}
					<button
						class="flex w-full items-center px-4 py-2 text-mofu-light-200 hover:opacity-70 dark:text-mofu-dark-200"
						onclick={handleEdit}
					>
						<Icon src={PencilSquare} solid size="16" class="mr-3" />
						Edit
					</button>
					<button
						class="flex w-full items-center px-4 py-2 text-rose-600 hover:opacity-70 dark:text-rose-500"
						onclick={handleDelete}
					>
						<Icon src={Trash} solid size="16" class="mr-3" />
						Delete
					</button>
				{/if}
				<button
					class="flex w-full items-center px-4 py-2 text-mofu-light-200 hover:opacity-70 dark:text-mofu-dark-200"
					onclick={handleReport}
				>
					<Icon src={Flag} solid size="16" class="mr-3" />
					Report
				</button>
			</div>
		</div>
	{/if}
</div>
