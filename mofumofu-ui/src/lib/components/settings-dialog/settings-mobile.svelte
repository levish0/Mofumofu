<script lang="ts">
	import {
		Icon,
		XMark,
		User,
		ShieldCheck,
		PaintBrush,
		ChevronLeft,
		ChevronRight
	} from 'svelte-hero-icons';
	import { fly } from 'svelte/transition';
	import AccountSettings from './pages/account-settings.svelte';
	import SecuritySettings from './pages/security-settings.svelte';
	import AppearanceSettings from './pages/appearance-settings.svelte';

	type Category = { id: string; label: string; icon: string; authRequired: boolean };

	let {
		categories,
		activeCategory = $bindable(),
		onClose
	}: {
		categories: Category[];
		activeCategory: string;
		onClose: () => void;
	} = $props();

	const iconMap: Record<string, typeof User> = {
		user: User,
		'shield-check': ShieldCheck,
		'paint-brush': PaintBrush
	};

	type MobileView = 'categories' | 'content';
	let mobileView: MobileView = $state('categories');

	function selectCategory(id: string) {
		activeCategory = id;
		mobileView = 'content';
	}

	export function goBackToCategories() {
		mobileView = 'categories';
	}

	export function getCurrentView() {
		return mobileView;
	}

	export function reset() {
		mobileView = 'categories';
	}
</script>

<div class="relative flex h-full flex-col overflow-hidden">
	<!-- Categories View -->
	{#if mobileView === 'categories'}
		<div
			class="absolute inset-0 flex flex-col bg-mofu-light-950 dark:bg-mofu-dark-950"
			in:fly={{ x: -100, duration: 200 }}
			out:fly={{ x: -100, duration: 200 }}
		>
			<!-- Header -->
			<div class="flex h-14 items-center justify-between border-b px-4">
				<h2 class="text-lg font-semibold">Settings</h2>
				<button
					type="button"
					onclick={onClose}
					class="rounded-lg p-2 text-muted-foreground transition-colors"
					aria-label="Close settings"
				>
					<Icon src={XMark} class="size-5" />
				</button>
			</div>

			<!-- Category List -->
			<nav class="flex-1 overflow-y-auto">
				<ul>
					{#each categories as cat (cat.id)}
						<li class="border-b">
							<button
								type="button"
								onclick={() => selectCategory(cat.id)}
								class="flex w-full items-center gap-4 px-4 py-4 text-left transition-colors"
							>
								<div class="flex size-10 items-center justify-center rounded-lg bg-muted">
									<Icon src={iconMap[cat.icon]} class="size-5 text-muted-foreground" solid />
								</div>
								<span class="flex-1 font-medium">{cat.label}</span>
								<Icon src={ChevronRight} class="size-5 text-muted-foreground" />
							</button>
						</li>
					{/each}
				</ul>
			</nav>
		</div>
	{/if}

	<!-- Content View -->
	{#if mobileView === 'content'}
		<div
			class="absolute inset-0 flex flex-col bg-mofu-light-950 dark:bg-mofu-dark-950"
			in:fly={{ x: 100, duration: 200 }}
			out:fly={{ x: 100, duration: 200 }}
		>
			<!-- Header with back button -->
			<div class="flex h-14 items-center gap-2 border-b px-2">
				<button
					type="button"
					onclick={goBackToCategories}
					class="rounded-lg p-2 text-muted-foreground transition-colors"
					aria-label="Back to categories"
				>
					<Icon src={ChevronLeft} class="size-5" />
				</button>
				<h3 class="flex-1 text-lg font-semibold">
					{categories.find((c) => c.id === activeCategory)?.label}
				</h3>
				<button
					type="button"
					onclick={onClose}
					class="rounded-lg p-2 text-muted-foreground transition-colors"
					aria-label="Close settings"
				>
					<Icon src={XMark} class="size-5" />
				</button>
			</div>

			<!-- Content Area -->
			<div class="flex-1 overflow-y-auto p-4">
				{#if activeCategory === 'account'}
					<AccountSettings />
				{:else if activeCategory === 'security'}
					<SecuritySettings />
				{:else if activeCategory === 'appearance'}
					<AppearanceSettings />
				{/if}
			</div>
		</div>
	{/if}
</div>
