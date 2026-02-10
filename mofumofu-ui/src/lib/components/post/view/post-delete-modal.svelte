<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';

	interface Props {
		isOpen?: boolean;
		isDeleting: boolean;
		onConfirm: () => void;
		onCancel: () => void;
	}

	let { isOpen = $bindable(false), isDeleting, onConfirm, onCancel }: Props = $props();
</script>

<Dialog.Root bind:open={isOpen}>
	<Dialog.Content class="p-2 text-black sm:max-w-md dark:bg-mofu-dark-800 dark:text-white">
		<div class="rounded-lg px-2 pt-4">
			<Dialog.Header class="mb-2 p-0">
				<Dialog.Title class="text-lg font-semibold">Delete Post</Dialog.Title>
				<Dialog.Description class="text-gray-600 dark:text-gray-300">
					Are you sure you want to delete this post?<br />
					This action cannot be undone.
				</Dialog.Description>
			</Dialog.Header>
		</div>

		<div class="flex justify-end gap-3 rounded-b-lg px-2 py-2">
			<Button variant="ghost" onclick={onCancel} disabled={isDeleting}>Cancel</Button>
			<Button variant="destructive" onclick={onConfirm} disabled={isDeleting}>
				{isDeleting ? 'Deleting...' : 'Delete'}
			</Button>
		</div>
	</Dialog.Content>
</Dialog.Root>

{#if isDeleting}
	<div class="fixed inset-0 z-100 flex items-center justify-center bg-black/50 backdrop-blur-sm">
		<div class="flex flex-col items-center space-y-4">
			<div
				class="h-12 w-12 animate-spin rounded-full border-4 border-mofu border-t-transparent"
			></div>
			<p class="text-lg font-medium text-white">Deleting...</p>
		</div>
	</div>
{/if}
