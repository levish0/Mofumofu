<!-- src/lib/components/Navbar.svelte -->
<script lang="ts">
	import { Cog6Tooth, Icon } from 'svelte-hero-icons';
	import {
		ArrowTrendingUp,
		Clock,
		Rss,
		Bell,
		MagnifyingGlass,
		ChevronDown,
		User,
		ArrowRightOnRectangle,
		DocumentText
	} from 'svelte-hero-icons';
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import { signOut } from '$lib/api/auth/authApi';
	import { fly, scale } from 'svelte/transition';
	import { Button } from '../ui/button';
	import { invalidateAll } from '$app/navigation';
	import { page } from '$app/state';
	import { postsStore } from '$lib/stores/posts.svelte';
	import * as m from '../../../paraglide/messages';
	import { cn } from '$lib/utils';

	let { isVisible, isAtTop } = $props();

	// 현재 경로에 따른 활성 상태 확인
	const isHomePage = $derived(page.url.pathname === '/');
	const isLatestPage = $derived(page.url.pathname === '/latest');

	// 네비게이션 클릭 시 postsStore 리셋 (다른 페이지로 이동할 때만)
	const handleNavClick = (targetPath: string) => {
		if (page.url.pathname !== targetPath) {
			postsStore.reset();
		}
	};

	let isDropdownOpen = $state(false);
	let closeTimer: ReturnType<typeof setTimeout> | null = null;

	const userInfo = $derived(userStore.user);
	const isLoading = $derived(userStore.isLoading);
	const isAuthenticated = $derived(authStore.isAuthenticated);

	onMount(async () => {
		// 초기 로드시 항상 refresh 시도하여 토큰 상태 확인
		// refresh token이 있으면 새 access token을 받고, 없으면 로그아웃 상태 유지
		const refreshSuccess = await authStore.tryRefreshToken();

		// refresh 성공하거나 이미 유효한 토큰이 있으면 프로필 로드
		if ((refreshSuccess || authStore.isAuthenticated) && !userStore.user) {
			await userStore.loadProfile();
		}
	});

	async function handleLogout() {
		try {
			await signOut();
			authStore.clearToken();
			userStore.clear();
			isDropdownOpen = false;
			await invalidateAll();
			window.location.reload();
		} catch (error) {
			console.error('Logout failed:', error);
			return;
		}
	}

	function openDropdown() {
		if (closeTimer) {
			clearTimeout(closeTimer);
			closeTimer = null;
		}
		isDropdownOpen = true;
	}

	function scheduleClose() {
		closeTimer = setTimeout(() => {
			isDropdownOpen = false;
			closeTimer = null;
		}, 100);
	}
</script>

<nav
	class={cn(
		'fixed top-0 right-0 left-0 z-50 max-h-[60px] w-full transition-all duration-100 ease-out',
		isAtTop() ? 'dark:bg-mofu-dark-900 bg-mofu-light-900' : 'dark:bg-mofu-dark-800 bg-mofu-light-800'
	)}
	style="transform: translateY({isVisible() ? '0' : '-100%'});"
>
	<div class="max-w-8xl mx-auto flex items-center justify-between px-4 py-3">
		<!-- 좌측 -->
		<div class="flex items-center space-x-2">
			<a
				href="/"
				class="text-3xl font-bold whitespace-nowrap text-black sm:mr-4 dark:text-white"
				onclick={() => handleNavClick('/')}>もふもふ</a
			>

			<div class="hidden items-center space-x-5 sm:flex">
				<Button
					variant="ghost"
					href="/"
					class={cn('p-0 text-lg', isHomePage ? 'opacity-80' : '')}
					onclick={() => handleNavClick('/')}
				>
					<Icon src={ArrowTrendingUp} solid size="20" class="mr-3 text-black dark:text-white" />
					{m.navbar_trending()}
				</Button>
				<Button
					variant="ghost"
					href="/latest"
					class={cn('p-0 text-lg', isLatestPage ? 'opacity-80' : '')}
					onclick={() => handleNavClick('/latest')}
				>
					<Icon src={Clock} size="20" solid class="mr-3 text-black dark:text-white" />
					{m.navbar_latest()}
				</Button>
			</div>
		</div>

		<!-- 우측 -->
		<div class="flex items-center space-x-3">
			{#if isLoading}
				<div class="shimmer h-9 w-9 rounded-full"></div>
				<div class="shimmer hidden h-9 w-9 rounded-full sm:block"></div>
				<div class="shimmer h-9 w-18 rounded-full sm:hidden"></div>
				<div class="shimmer hidden h-9 w-28 rounded-full sm:block"></div>
				<div class="shimmer hidden h-9 w-9 rounded-full sm:block"></div>
				<div class="shimmer hidden h-9 w-9 rounded-full sm:block"></div>
			{:else if isAuthenticated && userInfo}
				<Button href="/" variant="icon" aria-label="notifications">
					<Icon src={Bell} size="20" class="text-black dark:text-white" />
				</Button>

				<Button href="/search" variant="icon" aria-label="search">
					<Icon src={MagnifyingGlass} size="20" class="text-black dark:text-white" />
				</Button>

				<Button href="/write" variant="outline" class="bg-transparent px-3 py-0">{m.navbar_new_post()}</Button>

				<div
					class=" dropdown-container relative"
					role="button"
					tabindex="0"
					onmouseenter={openDropdown}
					onmouseleave={scheduleClose}
				>
					<button class="flex h-9 items-center space-x-1 rounded-lg" aria-label="profile_menu">
						<div class="h-9 w-9 overflow-hidden rounded-full">
							{#if userInfo.profile_image}
								<img src={userInfo.profile_image} alt="{userInfo.handle}의 프로필" class="h-full w-full object-cover" />
							{:else}
								<span
									class="dark:bg-mofu-dark-700 bg-mofu-light-700 flex h-full w-full items-center justify-center text-sm font-medium text-black dark:text-white"
								>
									{userInfo.handle.charAt(0).toUpperCase()}
								</span>
							{/if}
						</div>
					</button>

					{#if isDropdownOpen}
						<div
							class="dark:bg-mofu-dark-800 bg-mofu-light-800 absolute top-14 right-0 z-50 w-48 rounded-lg text-sm font-bold shadow-lg"
							transition:fly={{ y: -8, duration: 150 }}
							style="transform-origin: top right;"
						>
							<div class="py-1">
								<a
									href="/@{userInfo.handle}/profile"
									class="dark:text-mofu-dark-200 text-mofu-light-200 hover:text-mofu flex items-center px-4 py-2"
								>
									<Icon src={User} solid size="16" class="mr-3" />
									{m.navbar_my_page()}
								</a>
								<a
									href="/drafts"
									class="dark:text-mofu-dark-200 text-mofu-light-200 hover:text-mofu flex items-center px-4 py-2"
								>
									<Icon src={DocumentText} solid size="16" class="mr-3" />
									내 초안
								</a>
								<a
									href="/settings"
									class="dark:text-mofu-dark-200 text-mofu-light-200 hover:text-mofu flex items-center px-4 py-2"
								>
									<Icon src={Cog6Tooth} solid size="16" class="mr-3" />
									{m.navbar_settings()}
								</a>
								<button
									class="dark:text-mofu-dark-200 text-mofu-light-200 hover:text-mofu flex w-full items-center px-4 py-2"
									onclick={handleLogout}
								>
									<Icon src={ArrowRightOnRectangle} solid size="16" class="mr-3" />
									{m.navbar_sign_out()}
								</button>
							</div>
						</div>
					{/if}
				</div>
			{:else}
				<Button href="/search" variant="icon" aria-label="search">
					<Icon src={MagnifyingGlass} size="20" class="text-black dark:text-white" />
				</Button>
				<Button href="/settings" variant="icon" aria-label="settings">
					<Icon src={Cog6Tooth} solid size="20" class="text-black dark:text-white" />
				</Button>
				<Button href="/account/signin" class="py-0">{m.navbar_sign_in()}</Button>
			{/if}
		</div>
	</div>
</nav>
