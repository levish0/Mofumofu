<script lang="ts">
	import { Icon, ChevronUp, ChevronDown } from 'svelte-hero-icons';

	function scrollToTop() {
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}

	function scrollToBottom() {
		window.scrollTo({ top: document.body.scrollHeight, behavior: 'smooth' });
	}
</script>

<div class="fixed right-4 bottom-4 z-40 flex flex-col gap-2 md:hidden">
	<button
		onclick={scrollToTop}
		class="flex h-12 w-12 items-center justify-center rounded-full bg-white opacity-70 shadow-lg transition-all duration-300 hover:scale-105 hover:shadow-xl dark:bg-mofu-dark-900"
		aria-label="Scroll to top"
	>
		<Icon src={ChevronUp} class="h-6 w-6 text-gray-700 dark:text-gray-300" />
	</button>

	<button
		onclick={scrollToBottom}
		class="flex h-12 w-12 items-center justify-center rounded-full bg-white opacity-70 shadow-lg transition-all duration-300 hover:scale-105 hover:shadow-xl dark:bg-mofu-dark-900"
		aria-label="Scroll to bottom"
	>
		<Icon src={ChevronDown} class="h-6 w-6 text-gray-700 dark:text-gray-300" />
	</button>
</div>
