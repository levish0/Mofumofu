<script lang="ts">
	import { page } from '$app/state';
	import { fade, scale, fly } from 'svelte/transition';
	import SettingsDesktop from './settings-desktop.svelte';
	import SettingsMobile from './settings-mobile.svelte';

	type Category = {
		id: string;
		label: string;
		icon: string;
		authRequired: boolean;
	};

	const ALL_CATEGORIES: Category[] = [
		{ id: 'account', label: 'Account', icon: 'user', authRequired: true },
		{ id: 'security', label: 'Security', icon: 'shield-check', authRequired: true },
		{ id: 'appearance', label: 'Appearance', icon: 'paint-brush', authRequired: false }
	];

	let { open = $bindable(false) }: { open: boolean } = $props();

	const user = $derived(page.data.user);
	const categories = $derived(ALL_CATEGORIES.filter((c) => !c.authRequired || user));
	const defaultCategory = $derived(user ? 'account' : 'appearance');

	let activeCategory = $state('');
	let mobileRef: SettingsMobile | null = $state(null);

	function close() {
		open = false;
	}

	function handleEscape(e: KeyboardEvent) {
		if (!open || e.key !== 'Escape') return;

		e.preventDefault();
		if (mobileRef && mobileRef.getCurrentView() === 'content' && window.innerWidth < 1024) {
			mobileRef.goBackToCategories();
		} else {
			close();
		}
	}

	// Reset mobile view when dialog closes
	$effect(() => {
		if (!open && mobileRef) {
			mobileRef.reset();
		}
	});

	// Set default category when dialog opens
	$effect(() => {
		if (open) {
			activeCategory = defaultCategory;
		}
	});

	// Body scroll lock
	$effect(() => {
		if (open) {
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
		}
		return () => {
			document.body.style.overflow = '';
		};
	});
</script>

<svelte:window onkeydown={handleEscape} />

{#if open}
	<!-- Mobile (< lg) - fullscreen page -->
	<div
		class="fixed inset-0 z-[100] flex flex-col bg-mofu-light-950 lg:hidden dark:bg-mofu-dark-950"
		transition:fly={{ x: '100%', duration: 200 }}
		role="dialog"
		aria-modal="true"
		aria-label="Settings"
		tabindex="-1"
	>
		<SettingsMobile bind:this={mobileRef} {categories} bind:activeCategory onClose={close} />
	</div>

	<!-- Desktop (>= lg) - modal dialog -->
	<!-- Backdrop -->
	<div
		class="fixed inset-0 z-[60] hidden bg-black/60 backdrop-blur-sm lg:block"
		transition:fade={{ duration: 150 }}
		onclick={close}
		onkeydown={(e) => e.key === 'Enter' && close()}
		role="button"
		tabindex="-1"
		aria-label="Close settings"
	></div>

	<!-- Dialog -->
	<div
		class="fixed inset-2 z-[60] mx-auto hidden max-h-[calc(100vh-1rem)] overflow-hidden rounded-xl border bg-mofu-light-950 shadow-2xl sm:inset-4 sm:max-h-[calc(100vh-2rem)] lg:inset-12 lg:flex lg:max-h-[calc(100vh-6rem)] lg:max-w-7xl dark:bg-mofu-dark-950"
		transition:scale={{ duration: 150, start: 0.95, opacity: 0 }}
		role="dialog"
		aria-modal="true"
		aria-label="Settings"
		tabindex="-1"
	>
		<SettingsDesktop {categories} bind:activeCategory onClose={close} />
	</div>
{/if}
