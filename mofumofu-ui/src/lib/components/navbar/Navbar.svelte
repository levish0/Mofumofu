<script lang="ts">
	import {
		Icon,
		ArrowTrendingUp,
		Clock,
		Bell,
		MagnifyingGlass,
		User,
		ArrowRightOnRectangle,
		DocumentText,
		Cog6Tooth
	} from 'svelte-hero-icons';
	import { page } from '$app/state';
	import { invalidateAll } from '$app/navigation';
	import { logout } from '$lib/api/auth';
	import { Button } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import { fly } from 'svelte/transition';
	import { toast } from 'svelte-sonner';

	let {
		isVisible,
		isAtTop,
		onOpenSettings
	}: { isVisible: () => boolean; isAtTop: () => boolean; onOpenSettings: () => void } = $props();

	const user = $derived(page.data.user);
	const pathname = $derived(page.url.pathname as string);
	const isHomePage = $derived(pathname === '/');
	const isLatestPage = $derived(pathname === '/latest');

	let dropdownOpen = $state(false);
	let dropdownEl = $state<HTMLElement | null>(null);

	function handleClickOutside(e: MouseEvent) {
		if (dropdownEl && !dropdownEl.contains(e.target as Node)) {
			dropdownOpen = false;
		}
	}

	async function handleLogout() {
		try {
			await logout();
			await invalidateAll();
			dropdownOpen = false;
		} catch {
			toast.error('Logout failed.');
		}
	}
</script>

<svelte:window onclick={handleClickOutside} />

<nav
	class={cn(
		'fixed top-0 right-0 left-0 z-50 max-h-[60px] w-full transition-all duration-100 ease-out',
		isAtTop()
			? 'bg-mofu-light-900 dark:bg-mofu-dark-900'
			: 'bg-mofu-light-800 dark:bg-mofu-dark-800'
	)}
	style:transform="translateY({isVisible() ? '0' : '-100%'})"
>
	<div class="mx-auto flex max-w-8xl items-center justify-between px-4 py-3">
		<!-- Left -->
		<div class="flex items-center space-x-2">
			<a href="/" class="text-3xl font-bold whitespace-nowrap text-black sm:mr-4 dark:text-white">
				もふもふ
			</a>

			<div class="hidden items-center space-x-5 sm:flex">
				<Button variant="ghost" href="/" class={cn('p-0 text-lg', isHomePage && 'opacity-80')}>
					<Icon src={ArrowTrendingUp} solid size="20" class="mr-3 text-black dark:text-white" />
					Trending
				</Button>
				<Button
					variant="ghost"
					href="/latest"
					class={cn('p-0 text-lg', isLatestPage && 'opacity-80')}
				>
					<Icon src={Clock} size="20" solid class="mr-3 text-black dark:text-white" />
					Latest
				</Button>
			</div>
		</div>

		<!-- Right -->
		<div class="flex items-center space-x-3">
			{#if user}
				<Button href="/" variant="ghost" class="p-2" aria-label="notifications">
					<Icon src={Bell} solid size="20" class="text-black dark:text-white" />
				</Button>

				<Button href="/search" variant="ghost" class="p-2" aria-label="search">
					<Icon src={MagnifyingGlass} solid size="20" class="text-black dark:text-white" />
				</Button>

				<Button href="/write" variant="outline" class="bg-transparent px-3 py-0">New Post</Button>

				<!-- Profile dropdown -->
				<div class="relative" bind:this={dropdownEl}>
					<button
						class="h-9 w-9 overflow-hidden rounded-full"
						aria-label="profile menu"
						onclick={() => (dropdownOpen = !dropdownOpen)}
					>
						{#if user.profile_image}
							<img
								src={user.profile_image}
								alt="{user.handle}'s profile"
								class="h-full w-full object-cover"
							/>
						{:else}
							<span
								class="flex h-full w-full items-center justify-center bg-mofu-light-700 text-sm font-medium text-black dark:bg-mofu-dark-700 dark:text-white"
							>
								{user.handle.charAt(0).toUpperCase()}
							</span>
						{/if}
					</button>

					{#if dropdownOpen}
						<div
							class="absolute top-14 right-0 z-50 w-48 rounded-lg bg-mofu-light-800 text-sm font-bold shadow-lg dark:bg-mofu-dark-800"
							transition:fly={{ y: -8, duration: 150 }}
						>
							<div class="py-1">
								<a
									href="/@{user.handle}"
									class="flex items-center px-4 py-2 text-mofu-light-200 hover:text-mofu dark:text-mofu-dark-200"
								>
									<Icon src={User} solid size="16" class="mr-3" />
									My Page
								</a>
								<a
									href="/drafts"
									class="flex items-center px-4 py-2 text-mofu-light-200 hover:text-mofu dark:text-mofu-dark-200"
								>
									<Icon src={DocumentText} solid size="16" class="mr-3" />
									Drafts
								</a>
								<button
									class="flex w-full items-center px-4 py-2 text-mofu-light-200 hover:text-mofu dark:text-mofu-dark-200"
									onclick={() => {
										dropdownOpen = false;
										onOpenSettings();
									}}
								>
									<Icon src={Cog6Tooth} solid size="16" class="mr-3" />
									Settings
								</button>
								<button
									class="flex w-full items-center px-4 py-2 text-mofu-light-200 hover:text-mofu dark:text-mofu-dark-200"
									onclick={handleLogout}
								>
									<Icon src={ArrowRightOnRectangle} solid size="16" class="mr-3" />
									Sign out
								</button>
							</div>
						</div>
					{/if}
				</div>
			{:else}
				<Button href="/search" variant="ghost" class="p-2" aria-label="search">
					<Icon src={MagnifyingGlass} solid size="20" class="text-black dark:text-white" />
				</Button>
				<Button variant="ghost" class="p-2" aria-label="settings" onclick={onOpenSettings}>
					<Icon src={Cog6Tooth} solid size="20" class="text-black dark:text-white" />
				</Button>
				<Button href="/account/signin" class="py-0">Sign in</Button>
			{/if}
		</div>
	</div>
</nav>
