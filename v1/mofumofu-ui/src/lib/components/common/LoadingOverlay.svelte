<script lang="ts">
	import * as m from '../../../paraglide/messages';

	let { isVisible, message = m.loading_default() } = $props<{
		isVisible: boolean;
		message?: string;
	}>();
</script>

{#if isVisible}
	<div
		class="pointer-events-auto fixed inset-0 z-[100] flex items-center justify-center bg-black/50 backdrop-blur-sm"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<div class="flex flex-col items-center space-y-4">
			<!-- 스피너 -->
			<div class="border-mofu h-12 w-12 animate-spin rounded-full border-4 border-t-transparent"></div>
			<!-- 로딩 텍스트 -->
			<p class="text-lg font-medium text-white">
				{message}
			</p>
		</div>
	</div>
{/if}
