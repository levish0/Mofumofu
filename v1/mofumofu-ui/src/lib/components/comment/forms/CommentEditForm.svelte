<script lang="ts">
	interface Props {
		editContent: string;
		onSave: () => void;
		onCancel: () => void;
		onContentChange: (value: string) => void;
	}

	let { editContent, onSave, onCancel, onContentChange }: Props = $props();
</script>

<div
	class="mt-2"
	onclick={(e) => e.stopPropagation()}
	onkeydown={(e) => {
		if (e.key === 'Escape') e.stopPropagation();
	}}
	role="dialog"
	tabindex="0"
>
	<textarea
		bind:value={editContent}
		oninput={(e) => onContentChange((e.target as HTMLTextAreaElement).value)}
		onclick={(e) => e.stopPropagation()}
		class="border-mofu-light-600 dark:border-mofu-dark-600 bg-mofu-light-950 dark:bg-mofu-dark-800 focus:ring-mofu w-full resize-none rounded-md border p-2 text-sm focus:ring-2 focus:outline-none"
		rows="3"
		maxlength="500"
	></textarea>
	<div class="mt-2 flex justify-end gap-2">
		<button
			onclick={(e) => {
				e.stopPropagation();
				onCancel();
			}}
			class="text-mofu-light-400 dark:text-mofu-dark-400 hover:text-mofu-light-200 dark:hover:text-mofu-dark-200 px-3 py-1 text-sm transition-colors"
		>
			취소
		</button>
		<button
			onclick={(e) => {
				e.stopPropagation();
				onSave();
			}}
			disabled={!editContent.trim()}
			class="bg-mofu hover:bg-mofu/90 disabled:bg-mofu-light-600 rounded px-3 py-1 text-sm text-white transition-colors disabled:cursor-not-allowed"
		>
			저장
		</button>
	</div>
</div>
