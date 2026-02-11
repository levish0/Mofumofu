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
<div class="pt-15">
	{@render children()}
</div>

<SettingsDialog bind:open={settingsOpen} />
