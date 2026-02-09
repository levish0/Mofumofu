<script lang="ts">
	import { ChevronUp, ChevronDown } from 'svelte-hero-icons';
	import { Icon } from 'svelte-hero-icons';
	import { onMount } from 'svelte';

	function scrollToTop() {
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}

	function scrollToBottom() {
		window.scrollTo({ top: document.body.scrollHeight, behavior: 'smooth' });
	}
</script>

<!-- 플로팅 네비게이션 버튼들 (모바일에서만 표시) -->
<div class="fixed right-4 bottom-4 z-40 flex flex-col gap-2 md:hidden">
	<!-- 위로 가기 버튼 -->
	<button
		onclick={scrollToTop}
		class="dark:bg-mofu-dark-900 flex h-12 w-12 items-center justify-center rounded-full bg-white opacity-70 shadow-lg transition-all duration-300 hover:scale-105 hover:shadow-xl"
		aria-label="위로 가기"
	>
		<Icon src={ChevronUp} class="h-6 w-6 text-gray-700 dark:text-gray-300" />
	</button>

	<!-- 아래로 가기 버튼 -->
	<button
		onclick={scrollToBottom}
		class="dark:bg-mofu-dark-900 flex h-12 w-12 items-center justify-center rounded-full bg-white opacity-70 shadow-lg transition-all duration-300 hover:scale-105 hover:shadow-xl"
		aria-label="아래로 가기"
	>
		<Icon src={ChevronDown} class="h-6 w-6 text-gray-700 dark:text-gray-300" />
	</button>
</div>
