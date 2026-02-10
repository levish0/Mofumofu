<script lang="ts">
	import { cn } from '$lib/utils';
	import { Icon, XMark, User, ShieldCheck, PaintBrush } from 'svelte-hero-icons';
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
</script>

<div class="flex h-full w-full">
	<!-- Left Sidebar -->
	<aside class="flex w-64 shrink-0 flex-col border-r bg-mofu-light-950 dark:bg-mofu-dark-900">
		<!-- Header -->
		<div class="flex h-14 items-center border-b px-4">
			<h2 class="text-lg font-semibold">Settings</h2>
		</div>

		<!-- Navigation -->
		<nav class="flex-1 overflow-y-auto p-2">
			<ul class="space-y-1">
				{#each categories as cat (cat.id)}
					<li>
						<button
							type="button"
							onclick={() => (activeCategory = cat.id)}
							class={cn(
								'flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left text-sm font-medium transition-colors',
								activeCategory === cat.id ? 'bg-mofu/10 text-mofu' : ''
							)}
						>
							<Icon src={iconMap[cat.icon]} class="size-5" solid />
							{cat.label}
						</button>
					</li>
				{/each}
			</ul>
		</nav>
	</aside>

	<!-- Main Content -->
	<main class="flex flex-1 flex-col overflow-hidden bg-mofu-light-950 dark:bg-mofu-dark-950">
		<!-- Header with close button -->
		<div class="flex h-14 items-center justify-between border-b pr-2 pl-6">
			<h3 class="text-lg font-semibold">
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
		<div class="flex-1 overflow-y-auto p-6">
			{#if activeCategory === 'account'}
				<AccountSettings />
			{:else if activeCategory === 'security'}
				<SecuritySettings />
			{:else if activeCategory === 'appearance'}
				<AppearanceSettings />
			{/if}
		</div>
	</main>
</div>
