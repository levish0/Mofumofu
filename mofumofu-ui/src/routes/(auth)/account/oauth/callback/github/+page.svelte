<script lang="ts">
	import { page } from '$app/state';
	import { goto, invalidateAll } from '$app/navigation';
	import { onMount } from 'svelte';
	import { loginWithGithub } from '$lib/api/auth';
	import { isApiError } from '$lib/api';

	let error = $state('');

	onMount(async () => {
		const code = page.url.searchParams.get('code');
		const state = page.url.searchParams.get('state');

		if (!code || !state) {
			error = 'Missing OAuth parameters.';
			return;
		}

		try {
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
		} catch (e) {
			if (isApiError(e)) {
				if (e.status === 409) error = 'An account with this email already exists.';
				else error = e.details ?? 'OAuth login failed.';
			} else {
				error = 'An unexpected error occurred.';
			}
		}
	});
</script>

<svelte:head>
	<title>GitHub Login - Mofumofu</title>
</svelte:head>

<div class="flex min-h-full items-center justify-center p-4">
	<div class="w-full max-w-sm space-y-4 text-center">
		{#if error}
			<h1 class="text-2xl font-bold">Login failed</h1>
			<p class="text-sm text-destructive">{error}</p>
			<a href="/account/signin" class="text-sm font-medium text-primary hover:underline">
				Back to sign in
			</a>
		{:else}
			<h1 class="text-2xl font-bold">Signing in with GitHub...</h1>
			<p class="text-sm text-muted-foreground">Please wait.</p>
		{/if}
	</div>
</div>
