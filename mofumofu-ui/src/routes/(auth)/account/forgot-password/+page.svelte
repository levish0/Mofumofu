<script lang="ts">
	import { goto } from '$app/navigation';
	import { forgotPassword } from '$lib/api/auth';
	import { isApiError } from '$lib/api';
	import { Button } from '$lib/components/ui/button';
	import { FlatInput } from '$lib/components/flat-input';
	import { Label } from '$lib/components/ui/label';

	let email = $state('');
	let loading = $state(false);
	let error = $state('');

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		loading = true;

		try {
			await forgotPassword(email);
			goto('/account/password-reset-sent');
		} catch (e) {
			if (isApiError(e)) {
				error = e.details ?? 'Failed to send reset email.';
			} else {
				error = 'An unexpected error occurred.';
			}
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Forgot Password - Mofumofu</title>
</svelte:head>

<div class="flex min-h-full items-center justify-center p-4">
	<div class="w-full max-w-sm space-y-6">
		<div class="text-center">
			<h1 class="text-2xl font-bold">Forgot your password?</h1>
			<p class="mt-1 text-sm text-muted-foreground">
				Enter your email and we'll send you a reset link.
			</p>
		</div>

		<form onsubmit={handleSubmit} class="space-y-4">
			{#if error}
				<p class="text-sm text-destructive">{error}</p>
			{/if}

			<div class="space-y-2">
				<Label for="email">Email</Label>
				<FlatInput id="email" type="email" bind:value={email} required autocomplete="email" />
			</div>

			<Button type="submit" class="w-full" disabled={loading}>
				{loading ? 'Sending...' : 'Send reset link'}
			</Button>
		</form>

		<p class="text-center text-sm text-muted-foreground">
			<a href="/account/signin" class="font-medium text-foreground hover:underline">
				Back to sign in
			</a>
		</p>
	</div>
</div>
