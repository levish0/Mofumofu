<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { verifyEmail } from '$lib/api/auth';
	import { isApiError } from '$lib/api';

	let status = $state<'loading' | 'success' | 'error'>('loading');
	let errorMsg = $state('');

	onMount(async () => {
		const token = page.url.searchParams.get('token');
		if (!token) {
			status = 'error';
			errorMsg = 'No verification token provided.';
			return;
		}

		try {
			await verifyEmail(token);
			status = 'success';
		} catch (e) {
			status = 'error';
			if (isApiError(e)) {
				if (e.status === 409) errorMsg = 'Email is already verified.';
				else errorMsg = e.details ?? 'Verification failed. The link may have expired.';
			} else {
				errorMsg = 'An unexpected error occurred.';
			}
		}
	});
</script>

<svelte:head>
	<title>Verify Email - Mofumofu</title>
</svelte:head>

<div class="flex min-h-full items-center justify-center p-4">
	<div class="w-full max-w-sm space-y-4 text-center">
		{#if status === 'loading'}
			<h1 class="text-2xl font-bold">Verifying your email...</h1>
			<p class="text-sm text-muted-foreground">Please wait.</p>
		{:else if status === 'success'}
			<h1 class="text-2xl font-bold">Email verified!</h1>
			<p class="text-sm text-muted-foreground">
				Your email has been verified. You can now sign in.
			</p>
			<a href="/account/signin" class="text-sm font-medium text-primary hover:underline">
				Go to sign in
			</a>
		{:else}
			<h1 class="text-2xl font-bold">Verification failed</h1>
			<p class="text-sm text-destructive">{errorMsg}</p>
			<a href="/account/signin" class="text-sm font-medium text-primary hover:underline">
				Go to sign in
			</a>
		{/if}
	</div>
</div>
