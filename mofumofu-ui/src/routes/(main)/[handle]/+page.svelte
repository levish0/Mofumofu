<script lang="ts">
	import { page } from '$app/state';
	import { useNavbarScroll } from '$lib/hooks/useNavbarScroll.svelte';
	import { ProfileHeader, ProfileInfo, ProfilePosts } from '$lib/components/profile';
	import FollowList from '$lib/components/follow/follow-list.svelte';
	import { getFollowers, getFollowing } from '$lib/api/follows';
	import type { FollowUserItem } from '$lib/api/types';
	import { Icon, ArrowLeft } from 'svelte-hero-icons';

	const profile = $derived(page.data.profile);
	const posts = $derived(page.data.posts);
	const user = $derived(page.data.user);
	const isAuthenticated = $derived(!!user);
	const isOwnProfile = $derived(user?.handle === profile.handle);

	const { isVisible } = useNavbarScroll();
	const topPosition = $derived(isVisible() ? '68px' : '8px');

	// View state: 'posts' | 'followers' | 'following'
	type ViewMode = 'posts' | 'followers' | 'following';
	let view = $state<ViewMode>('posts');

	const PAGE_SIZE = 20;
	let followUsers = $state<FollowUserItem[]>([]);
	let followLoading = $state(false);
	let followHasMore = $state(false);

	async function loadFollowList(mode: 'followers' | 'following', cursorId?: string) {
		followLoading = true;
		try {
			const fetcher = mode === 'followers' ? getFollowers : getFollowing;
			const res = await fetcher({
				user_id: profile.id,
				limit: PAGE_SIZE,
				cursor_id: cursorId,
				cursor_direction: cursorId ? 'Older' : undefined
			});
			followUsers = cursorId ? [...followUsers, ...res.data] : res.data;
			followHasMore = res.has_older;
		} finally {
			followLoading = false;
		}
	}

	function switchView(mode: ViewMode) {
		if (mode === view) return;
		view = mode;
		if (mode !== 'posts') {
			followUsers = [];
			followHasMore = false;
			loadFollowList(mode);
		}
	}

	function loadMoreFollows() {
		const last = followUsers[followUsers.length - 1];
		if (last && (view === 'followers' || view === 'following')) {
			loadFollowList(view, last.id);
		}
	}
</script>

<svelte:head>
	<title>{profile.display_name} (@{profile.handle}) - Mofumofu</title>
	<meta
		name="description"
		content={profile.bio || `${profile.display_name}'s profile on Mofumofu`}
	/>

	<meta property="og:title" content="{profile.display_name} (@{profile.handle}) - Mofumofu" />
	<meta
		property="og:description"
		content={profile.bio || `${profile.display_name}'s profile on Mofumofu`}
	/>
	<meta property="og:type" content="profile" />
	<meta property="og:url" content="https://mofumofu.ink/@{profile.handle}" />
	<meta
		property="og:image"
		content={profile.banner_image || profile.profile_image || 'https://mofumofu.ink/og-default.png'}
	/>
	<meta property="og:site_name" content="Mofumofu" />
	<meta property="profile:username" content={profile.handle} />

	<meta name="twitter:card" content="summary" />
	<meta name="twitter:title" content="{profile.display_name} (@{profile.handle}) - Mofumofu" />
	<meta
		name="twitter:description"
		content={profile.bio || `${profile.display_name}'s profile on Mofumofu`}
	/>
	<meta
		name="twitter:image"
		content={profile.banner_image || profile.profile_image || 'https://mofumofu.ink/og-default.png'}
	/>
</svelte:head>

<div class="min-h-screen">
	<div class="mx-auto max-w-8xl px-4 pt-2">
		<div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
			<!-- Left: Profile info (sticky) -->
			<div>
				<div
					class="sticky z-20 bg-mofu-light-900 transition-all duration-100 ease-out dark:bg-mofu-dark-900"
					style="top: {topPosition}"
				>
					<ProfileHeader {profile} {isOwnProfile} {isAuthenticated} />
					<ProfileInfo {profile} onViewChange={switchView} activeView={view} />
				</div>
			</div>

			<!-- Right: Content panel -->
			<div class="min-h-screen lg:col-span-1">
				{#if view === 'posts'}
					<ProfilePosts {posts} profileName={profile.display_name} />
				{:else}
					<div class="space-y-2 pt-2">
						<button
							onclick={() => switchView('posts')}
							class="flex items-center gap-2 rounded-lg px-2 py-1.5 text-sm font-medium text-mofu-light-300 transition-colors hover:text-black dark:text-mofu-dark-300 dark:hover:text-white"
						>
							<Icon src={ArrowLeft} class="h-4 w-4" />
							Back to posts
						</button>
						<h2 class="px-2 text-lg font-bold text-gray-900 dark:text-white">
							{view === 'followers' ? 'Followers' : 'Following'}
						</h2>
						{#if !followLoading && followUsers.length === 0}
							<div class="py-16 text-center">
								<p class="text-mofu-light-300 dark:text-mofu-dark-300">
									{view === 'followers' ? 'No followers yet' : 'Not following anyone yet'}
								</p>
							</div>
						{:else}
							<FollowList
								users={followUsers}
								loading={followLoading}
								hasMore={followHasMore}
								onLoadMore={loadMoreFollows}
							/>
						{/if}
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
