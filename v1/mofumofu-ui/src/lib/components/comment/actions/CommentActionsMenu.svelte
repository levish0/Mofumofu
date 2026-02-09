<script lang="ts">
	import { fly } from 'svelte/transition';
	import { Icon, EllipsisVertical, PencilSquare, Trash, Flag } from 'svelte-hero-icons';
	import * as m from '../../../../paraglide/messages';

	interface Props {
		isDropdownOpen: boolean;
		isOwner: boolean;
		onEdit: () => void;
		onDelete: () => void;
		onReport: () => void;
		onOpenDropdown: () => void;
		onScheduleClose: () => void;
	}

	let { isDropdownOpen, isOwner, onEdit, onDelete, onReport, onOpenDropdown, onScheduleClose }: Props = $props();
</script>

<div
	class="relative"
	role="button"
	tabindex="0"
	onmouseenter={onOpenDropdown}
	onmouseleave={onScheduleClose}
	onclick={(e) => e.stopPropagation()}
	onkeydown={(e) => {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			e.stopPropagation();
		}
	}}
>
	<button
		class="text-mofu-light-400 dark:text-mofu-dark-400 hover:text-mofu-light-200 dark:hover:text-mofu-dark-200 hover:bg-mofu-light-800 dark:hover:bg-mofu-dark-700 rounded p-1 transition-colors"
	>
		<Icon src={EllipsisVertical} class="h-4 w-4" />
	</button>

	{#if isDropdownOpen}
		<div
			class="bg-mofu-light-800 dark:bg-mofu-dark-800 absolute top-8 right-0 z-50 w-48 rounded-lg text-sm font-bold shadow-lg"
			transition:fly={{ y: -8, duration: 150 }}
			style="transform-origin: top right;"
		>
			<div class="py-1">
				{#if isOwner}
					<button
						class="text-mofu-light-200 dark:text-mofu-dark-200 flex w-full items-center px-4 py-2 hover:opacity-70"
						onclick={onEdit}
					>
						<Icon src={PencilSquare} solid size="16" class="mr-3" />
						{m.comment_edit()}
					</button>
					<button
						class="flex w-full items-center px-4 py-2 text-rose-600 hover:opacity-70 dark:text-rose-500"
						onclick={onDelete}
					>
						<Icon src={Trash} solid size="16" class="mr-3" />
						{m.comment_delete()}
					</button>
				{/if}
				<button
					class="text-mofu-light-200 dark:text-mofu-dark-200 flex w-full items-center px-4 py-2 hover:opacity-70"
					onclick={onReport}
				>
					<Icon src={Flag} solid size="16" class="mr-3" />
					{m.comment_report()}
				</button>
			</div>
		</div>
	{/if}
</div>
