<script lang="ts">
	import { setContext } from 'svelte';
	import Navbar from '$lib/components/navbar/Navbar.svelte';
	import { SettingsDialog } from '$lib/components/settings-dialog';
	import { useNavbarScroll } from '$lib/hooks/useNavbarScroll.svelte';

	let { children } = $props();

	const { isVisible, isAtTop } = useNavbarScroll();

	let settingsOpen = $state(false);

	setContext('openSettings', () => (settingsOpen = true));
</script>

<Navbar {isVisible} {isAtTop} onOpenSettings={() => (settingsOpen = true)} />
<div class="min-h-screen bg-mofu-light-900 pt-15 dark:bg-mofu-dark-900">
	{@render children()}
</div>

<SettingsDialog bind:open={settingsOpen} />
