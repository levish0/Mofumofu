<script lang="ts">
	import { page } from '$app/state';
	import { goto, invalidateAll } from '$app/navigation';
	import { onMount } from 'svelte';
	import { completeSignup } from '$lib/api/auth';
	import { checkHandleAvailable } from '$lib/api/user';
	import { isApiError } from '$lib/api';
	import { validateField, handleSchema } from '$lib/schemas/auth';
	import { Button } from '$lib/components/ui/button';
	import { FlatInput } from '$lib/components/flat-input';
	import { Label } from '$lib/components/ui/label';
	const provider = $derived(page.params.provider);

	let handle = $state('');
	let pendingToken = $state('');
	let displayName = $state('');
	let email = $state('');
	let loading = $state(false);
	let error = $state('');
	let ready = $state(false);

	// Inline validation
	let handleTouched = $state(false);
	let handleError = $derived(handleTouched ? validateField(handleSchema, handle) : null);

	// Handle availability
	let handleStatus = $state<'idle' | 'checking' | 'available' | 'taken'>('idle');
	let handleCheckTimeout: ReturnType<typeof setTimeout>;

	onMount(() => {
		const raw = sessionStorage.getItem('oauth_pending');
		if (!raw) {
			goto('/account/signin');
			return;
		}

		try {
			const data = JSON.parse(raw);
			pendingToken = data.pending_token;
			displayName = data.display_name;
			email = data.email;
			ready = true;
		} catch {
			goto('/account/signin');
		}
	});

	function onHandleInput() {
		handleTouched = true;
		handleStatus = 'idle';
		clearTimeout(handleCheckTimeout);
		if (!handleError && handle.length >= 3) {
			handleCheckTimeout = setTimeout(async () => {
				handleStatus = 'checking';
				try {
					const { available } = await checkHandleAvailable(handle);
					handleStatus = available ? 'available' : 'taken';
				} catch {
					handleStatus = 'idle';
				}
			}, 400);
		}
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		handleTouched = true;
		if (handleError) return;

		error = '';
		loading = true;

		try {
			await completeSignup({ pending_token: pendingToken, handle });
			sessionStorage.removeItem('oauth_pending');
			await invalidateAll();
			goto('/');
		} catch (e) {
			if (isApiError(e)) {
				if (e.status === 409) error = 'This handle is already taken.';
				else if (e.status === 401) {
					error = 'Session expired. Please try signing in again.';
				} else error = e.details ?? 'Signup failed.';
			} else {
				error = 'An unexpected error occurred.';
			}
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Choose Handle - Mofumofu</title>
</svelte:head>

<div class="flex min-h-full items-center justify-center p-4">
	<div class="w-full max-w-sm space-y-6">
		{#if ready}
			<div class="text-center">
				<h1 class="text-2xl font-bold">Almost there!</h1>
				<p class="mt-1 text-sm text-muted-foreground">
					Choose a handle for your {provider} account.
				</p>
			</div>

			<div class="space-y-1 text-sm text-muted-foreground">
				<p>Name: {displayName}</p>
				<p>Email: {email}</p>
			</div>

			<form onsubmit={handleSubmit} class="space-y-4">
				{#if error}
					<p class="text-sm text-destructive">{error}</p>
				{/if}

				<div class="space-y-2">
					<Label for="handle">Handle</Label>
					<FlatInput
						id="handle"
						type="text"
						bind:value={handle}
						required
						autocomplete="username"
						oninput={onHandleInput}
						onblur={() => (handleTouched = true)}
					/>
					{#if handleError}
						<p class="text-xs text-destructive">{handleError}</p>
					{:else if handleStatus === 'checking'}
						<p class="text-xs text-muted-foreground">Checking availability...</p>
					{:else if handleStatus === 'available'}
						<p class="text-xs text-green-500">Handle is available!</p>
					{:else if handleStatus === 'taken'}
						<p class="text-xs text-destructive">Handle is already taken.</p>
					{/if}
				</div>

				<Button type="submit" class="w-full" disabled={loading || handleStatus === 'taken'}>
					{loading ? 'Creating account...' : 'Complete signup'}
				</Button>
			</form>
		{:else}
			<p class="text-center text-sm text-muted-foreground">Loading...</p>
		{/if}
	</div>
</div>
