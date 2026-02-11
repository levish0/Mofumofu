<script lang="ts">
	import { page } from '$app/state';
	import { goto, invalidateAll } from '$app/navigation';
	import { onMount } from 'svelte';
	import { loginWithGithub, linkGithub } from '$lib/api/auth';
	import { isApiError } from '$lib/api';

	let error = $state('');
	let mode = $state<'login' | 'link'>('login');

	onMount(async () => {
		const code = page.url.searchParams.get('code');
		const state = page.url.searchParams.get('state');

		if (!code || !state) {
			error = 'Missing OAuth parameters.';
			return;
		}

		const linkMode = sessionStorage.getItem('oauth_link_mode');
		mode = linkMode === 'link' ? 'link' : 'login';
		sessionStorage.removeItem('oauth_link_mode');

		try {
			if (mode === 'link') {
				await linkGithub(code, state);
				await invalidateAll();
				goto('/');
			} else {
				const result = await loginWithGithub(code, state);

				if (result.kind === 'existing_user') {
					await invalidateAll();
					goto('/');
				} else {
					sessionStorage.setItem(
						'oauth_pending',
						JSON.stringify({ provider: 'github', ...result.data })
					);
					goto('/account/set-handle/github');
				}
			}
		} catch (e) {
			if (isApiError(e)) {
				if (mode === 'link') {
					if (e.status === 409) error = 'This account is already linked.';
					else error = e.details ?? 'Failed to link account.';
				} else {
					if (e.status === 409) error = 'An account with this email already exists.';
					else error = e.details ?? 'OAuth login failed.';
				}
			} else {
				error = 'An unexpected error occurred.';
			}
		}
	});
</script>

<svelte:head>
	<title>{mode === 'link' ? 'Link GitHub Account' : 'GitHub Login'} - Mofumofu</title>
</svelte:head>

<div class="flex min-h-full items-center justify-center p-4">
	<div class="w-full max-w-sm space-y-4 text-center">
		{#if error}
			<h1 class="text-2xl font-bold">{mode === 'link' ? 'Link failed' : 'Login failed'}</h1>
			<p class="text-sm text-destructive">{error}</p>
			{#if mode === 'link'}
				<a href="/" class="text-sm font-medium text-primary hover:underline">Back to home</a>
			{:else}
				<a href="/account/signin" class="text-sm font-medium text-primary hover:underline">
					Back to sign in
				</a>
			{/if}
		{:else}
			<h1 class="text-2xl font-bold">
				{mode === 'link' ? 'Linking GitHub account...' : 'Signing in with GitHub...'}
			</h1>
			<p class="text-sm text-muted-foreground">Please wait.</p>
		{/if}
	</div>
</div>
