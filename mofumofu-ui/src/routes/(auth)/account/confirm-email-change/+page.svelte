<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { confirmEmailChange } from '$lib/api/auth';
	import { isApiError } from '$lib/api';

	let status = $state<'loading' | 'success' | 'error'>('loading');
	let errorMsg = $state('');

	onMount(async () => {
		const token = page.url.searchParams.get('token');
		if (!token) {
			status = 'error';
			errorMsg = 'No confirmation token provided.';
			return;
		}

		try {
			await confirmEmailChange(token);
			status = 'success';
		} catch (e) {
			status = 'error';
			if (isApiError(e)) {
				if (e.status === 409) errorMsg = 'Email change has already been confirmed.';
				else errorMsg = e.details ?? 'Confirmation failed. The link may have expired.';
			} else {
				errorMsg = 'An unexpected error occurred.';
			}
		}
	});
</script>

<svelte:head>
	<title>Confirm Email Change - Mofumofu</title>
</svelte:head>

<div class="flex min-h-full items-center justify-center p-4">
	<div class="w-full max-w-sm space-y-4 text-center">
		{#if status === 'loading'}
			<h1 class="text-2xl font-bold">Confirming email change...</h1>
			<p class="text-sm text-muted-foreground">Please wait.</p>
		{:else if status === 'success'}
			<h1 class="text-2xl font-bold">Email changed!</h1>
			<p class="text-sm text-muted-foreground">Your email has been updated successfully.</p>
			<a href="/" class="text-sm font-medium text-primary hover:underline"> Go to home </a>
		{:else}
			<h1 class="text-2xl font-bold">Confirmation failed</h1>
			<p class="text-sm text-destructive">{errorMsg}</p>
			<a href="/" class="text-sm font-medium text-primary hover:underline"> Go to home </a>
		{/if}
	</div>
</div>
