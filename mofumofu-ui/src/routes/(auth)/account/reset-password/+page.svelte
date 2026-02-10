<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { resetPassword } from '$lib/api/auth';
	import { isApiError } from '$lib/api';
	import { Button } from '$lib/components/ui/button';
	import { FLAT_INPUT_CLASS } from '$lib/components/flat-input';
	import { Label } from '$lib/components/ui/label';
	import * as Password from '$lib/components/ui/password';
	import type { ZxcvbnResult } from '@zxcvbn-ts/core';

	const SCORE_NAMING = ['Poor', 'Weak', 'Average', 'Strong', 'Secure'];

	let newPassword = $state('');
	let strength = $state<ZxcvbnResult>();
	let loading = $state(false);
	let error = $state('');

	const token = $derived(page.url.searchParams.get('token'));

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!token) return;

		error = '';
		loading = true;

		try {
			await resetPassword({ token, new_password: newPassword });
			goto('/account/password-reset-success');
		} catch (e) {
			if (isApiError(e)) {
				error = e.details ?? 'Password reset failed. The link may have expired.';
			} else {
				error = 'An unexpected error occurred.';
			}
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Reset Password - Mofumofu</title>
</svelte:head>

<div class="flex min-h-full items-center justify-center p-4">
	<div class="w-full max-w-sm space-y-6">
		<div class="text-center">
			<h1 class="text-2xl font-bold">Set a new password</h1>
			<p class="mt-1 text-sm text-muted-foreground">Choose a strong password for your account.</p>
		</div>

		{#if !token}
			<p class="text-center text-sm text-destructive">No reset token provided.</p>
			<a
				href="/account/forgot-password"
				class="block text-center text-sm font-medium text-primary hover:underline"
			>
				Request a new reset link
			</a>
		{:else}
			<form onsubmit={handleSubmit} class="space-y-4">
				{#if error}
					<p class="text-sm text-destructive">{error}</p>
				{/if}

				<div class="space-y-2">
					<Label for="new-password">New Password</Label>
					<Password.Root minScore={4}>
						<Password.Input
							id="new-password"
							bind:value={newPassword}
							required
							autocomplete="new-password"
							placeholder="At least 8 characters"
							class={FLAT_INPUT_CLASS}
						>
							<Password.ToggleVisibility />
						</Password.Input>
						<div class="flex flex-col gap-1">
							<Password.Strength bind:strength />
							<span class="text-sm text-muted-foreground">
								{SCORE_NAMING[strength?.score ?? 0]}
							</span>
						</div>
					</Password.Root>
				</div>

				<Button type="submit" class="w-full" disabled={loading}>
					{loading ? 'Resetting...' : 'Reset password'}
				</Button>
			</form>
		{/if}
	</div>
</div>
