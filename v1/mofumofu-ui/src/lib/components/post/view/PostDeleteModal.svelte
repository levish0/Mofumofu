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
	<Dialog.Content class="dark:bg-mofu-dark-800 p-2 text-black sm:max-w-md dark:text-white">
		<div class="rounded-lg px-2 pt-4">
			<Dialog.Header class="mb-2 p-0">
				<Dialog.Title class="text-lg font-semibold">포스트 삭제</Dialog.Title>
				<Dialog.Description class="text-gray-600 dark:text-gray-300">
					이 포스트를 정말 삭제하시겠습니까?<br />
					삭제된 포스트는 복구할 수 없습니다.
				</Dialog.Description>
			</Dialog.Header>
		</div>

		<div class="flex justify-end gap-3 rounded-b-lg px-2 py-2">
			<Button variant="ghost" onclick={onCancel} disabled={isDeleting}>취소</Button>
			<Button variant="destructive" onclick={onConfirm} disabled={isDeleting}>
				{isDeleting ? '삭제 중...' : '삭제하기'}
			</Button>
		</div>
	</Dialog.Content>
</Dialog.Root>

{#if isDeleting}
	<div class="fixed inset-0 z-100 flex items-center justify-center bg-black/50 backdrop-blur-sm">
		<div class="flex flex-col items-center space-y-4">
			<div class="border-mofu h-12 w-12 animate-spin rounded-full border-4 border-t-transparent"></div>
			<p class="text-lg font-medium text-white">삭제 중...</p>
		</div>
	</div>
{/if}
