<script lang="ts">
	import { page } from '$app/state';
	import { useNavbarScroll } from '$lib/hooks/useNavbarScroll.svelte';
	import { ProfileHeader, ProfileInfo, ProfilePosts } from '$lib/components/profile';

	const profile = $derived(page.data.profile);
	const posts = $derived(page.data.posts);
	const user = $derived(page.data.user);
	const isAuthenticated = $derived(!!user);
	const isOwnProfile = $derived(user?.handle === profile.handle);

	const { isVisible } = useNavbarScroll();
	const topPosition = $derived(isVisible() ? '68px' : '8px');
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
	<meta
		name="twitter:title"
		content="{profile.display_name} (@{profile.handle}) - Mofumofu"
	/>
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
					<ProfileInfo {profile} />
				</div>
			</div>

			<!-- Right: Posts -->
			<div class="lg:col-span-1">
				<ProfilePosts {posts} profileName={profile.display_name} />
			</div>
		</div>
	</div>
</div>
