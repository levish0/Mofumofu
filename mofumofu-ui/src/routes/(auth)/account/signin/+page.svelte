<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';
	import { login, totpVerify } from '$lib/api/auth';
	import { getGoogleAuthUrl, getGithubAuthUrl } from '$lib/api/auth';
	import { isApiError } from '$lib/api';
	import { validateField, emailSchema, totpCodeSchema } from '$lib/schemas/auth';
	import { Button } from '$lib/components/ui/button';
	import { FlatInput, FLAT_INPUT_CLASS } from '$lib/components/flat-input';
	import { Label } from '$lib/components/ui/label';
	import { Separator } from '$lib/components/ui/separator';
	import * as Password from '$lib/components/ui/password';
	import { toast } from 'svelte-sonner';

	let email = $state('');
	let password = $state('');
	let rememberMe = $state(false);
	let loading = $state(false);
	let error = $state('');

	// Inline validation
	let emailTouched = $state(false);
	let emailError = $derived(emailTouched ? validateField(emailSchema, email) : null);

	// TOTP state
	let totpStep = $state(false);
	let totpCode = $state('');
	let tempToken = $state('');
	let totpTouched = $state(false);
	let totpError = $derived(totpTouched ? validateField(totpCodeSchema, totpCode) : null);

	async function handleLogin(e: SubmitEvent) {
		e.preventDefault();
		emailTouched = true;
		if (emailError) return;

		error = '';
		loading = true;

		try {
			const result = await login({ email, password, remember_me: rememberMe });

			if (result.kind === 'totp_required') {
				totpStep = true;
				tempToken = result.temp_token;
			} else {
				await invalidateAll();
				goto('/');
			}
		} catch (e) {
			if (isApiError(e)) {
				if (e.status === 401) error = 'Invalid email or password.';
				else if (e.status === 404) error = 'Account not found.';
				else error = e.details ?? 'Login failed. Please try again.';
			} else {
				error = 'An unexpected error occurred.';
			}
		} finally {
			loading = false;
		}
	}

	async function handleTotpVerify(e: SubmitEvent) {
		e.preventDefault();
		totpTouched = true;
		if (totpError) return;

		error = '';
		loading = true;

		try {
			await totpVerify(totpCode, tempToken);
			await invalidateAll();
			goto('/');
		} catch (e) {
			if (isApiError(e)) {
				error = e.status === 400 ? 'Invalid TOTP code.' : (e.details ?? 'Verification failed.');
			} else {
				error = 'An unexpected error occurred.';
			}
		} finally {
			loading = false;
		}
	}

	async function handleOAuth(provider: 'google' | 'github') {
		try {
			const { auth_url } =
				provider === 'google' ? await getGoogleAuthUrl() : await getGithubAuthUrl();
			window.location.href = auth_url;
		} catch {
			toast.error(`Failed to connect to ${provider}.`);
		}
	}
</script>

<svelte:head>
	<title>Sign In - Mofumofu</title>
</svelte:head>

<div class="flex min-h-full">
	<!-- Left: Image (hidden on mobile) -->
	<div class="relative hidden w-0 flex-1 items-center justify-center overflow-hidden lg:flex">
		<a href="/" class="block">
			<img src="/mofumofu_kawaii_mini.svg" alt="Mofumofu" class="h-auto w-[500px] object-contain" />
		</a>
	</div>

	<!-- Right: Form -->
	<div
		class="relative flex flex-1 flex-col justify-center bg-background px-4 py-12 shadow-lg sm:px-6 lg:flex-none lg:px-20 xl:px-24"
	>
		<div class="mx-auto w-full max-w-sm lg:w-96">
			{#if !totpStep}
				<div>
					<h2 class="text-2xl font-bold tracking-tight">Welcome back</h2>
					<p class="mt-2 text-sm text-muted-foreground">
						Don't have an account?
						<a href="/account/signup" class="font-semibold text-primary hover:opacity-70">
							Sign up
						</a>
					</p>
				</div>

				<form onsubmit={handleLogin} class="mt-8 space-y-4">
					{#if error}
						<div class="rounded-lg border border-red-500/20 bg-red-900/20 p-3">
							<p class="text-xs text-rose-400">{error}</p>
						</div>
					{/if}

					<div class="space-y-2">
						<Label for="email">Email</Label>
						<FlatInput
							id="email"
							type="email"
							bind:value={email}
							required
							autocomplete="email"
							onblur={() => (emailTouched = true)}
						/>
						{#if emailError}
							<p class="text-xs text-rose-400">{emailError}</p>
						{/if}
					</div>

					<div class="space-y-2">
						<div class="flex items-center justify-between">
							<Label for="password">Password</Label>
							<a
								href="/account/forgot-password"
								class="text-sm font-semibold text-primary hover:opacity-70"
							>
								Forgot password?
							</a>
						</div>
						<Password.Root>
							<Password.Input
								id="password"
								bind:value={password}
								required
								autocomplete="current-password"
								class={FLAT_INPUT_CLASS}
							>
								<Password.ToggleVisibility />
							</Password.Input>
						</Password.Root>
					</div>

					<label class="flex items-center gap-2 text-sm">
						<input type="checkbox" bind:checked={rememberMe} class="accent-primary" />
						Remember me
					</label>

					<Button type="submit" class="w-full" disabled={loading}>
						{loading ? 'Signing in...' : 'Sign in'}
					</Button>
				</form>

				<div class="mt-6">
					<div class="relative">
						<Separator />
						<span
							class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-background px-4 text-sm text-muted-foreground"
						>
							or
						</span>
					</div>

					<div class="mt-6 grid grid-cols-2 gap-4">
						<Button variant="outline" onclick={() => handleOAuth('google')} class="gap-3">
							<svg viewBox="0 0 24 24" aria-hidden="true" class="h-5 w-5">
								<path
									d="M12.0003 4.75C13.7703 4.75 15.3553 5.36002 16.6053 6.54998L20.0303 3.125C17.9502 1.19 15.2353 0 12.0003 0C7.31028 0 3.25527 2.69 1.28027 6.60998L5.27028 9.70498C6.21525 6.86002 8.87028 4.75 12.0003 4.75Z"
									fill="#EA4335"
								/>
								<path
									d="M23.49 12.275C23.49 11.49 23.415 10.73 23.3 10H12V14.51H18.47C18.18 15.99 17.34 17.25 16.08 18.1L19.945 21.1C22.2 19.01 23.49 15.92 23.49 12.275Z"
									fill="#4285F4"
								/>
								<path
									d="M5.26498 14.2949C5.02498 13.5699 4.88501 12.7999 4.88501 11.9999C4.88501 11.1999 5.01998 10.4299 5.26498 9.7049L1.275 6.60986C0.46 8.22986 0 10.0599 0 11.9999C0 13.9399 0.46 15.7699 1.28 17.3899L5.26498 14.2949Z"
									fill="#FBBC05"
								/>
								<path
									d="M12.0004 24.0001C15.2404 24.0001 17.9654 22.935 19.9454 21.095L16.0804 18.095C15.0054 18.82 13.6204 19.245 12.0004 19.245C8.8704 19.245 6.21537 17.135 5.2654 14.29L1.27539 17.385C3.25539 21.31 7.31040 24.0001 12.0004 24.0001Z"
									fill="#34A853"
								/>
							</svg>
							Google
						</Button>
						<Button variant="outline" onclick={() => handleOAuth('github')} class="gap-3">
							<svg viewBox="0 0 20 20" aria-hidden="true" class="size-5 fill-current">
								<path
									d="M10 0C4.477 0 0 4.484 0 10.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0110 4.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.942.359.31.678.921.678 1.856 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0020 10.017C20 4.484 15.522 0 10 0z"
									clip-rule="evenodd"
									fill-rule="evenodd"
								/>
							</svg>
							GitHub
						</Button>
					</div>
				</div>
			{:else}
				<div>
					<h2 class="text-2xl font-bold tracking-tight">Two-factor authentication</h2>
					<p class="mt-2 text-sm text-muted-foreground">
						Enter the 6-digit code from your authenticator app, or an 8-character backup code.
					</p>
				</div>

				<form onsubmit={handleTotpVerify} class="mt-8 space-y-4">
					{#if error}
						<div class="rounded-lg border border-red-500/20 bg-red-900/20 p-3">
							<p class="text-xs text-rose-400">{error}</p>
						</div>
					{/if}

					<div class="space-y-2">
						<Label for="totp-code">Code</Label>
						<FlatInput
							id="totp-code"
							type="text"
							bind:value={totpCode}
							required
							autocomplete="one-time-code"
							placeholder="000000"
							onblur={() => (totpTouched = true)}
						/>
						{#if totpError}
							<p class="text-xs text-rose-400">{totpError}</p>
						{/if}
					</div>

					<Button type="submit" class="w-full" disabled={loading}>
						{loading ? 'Verifying...' : 'Verify'}
					</Button>

					<button
						type="button"
						class="w-full text-center text-sm text-muted-foreground hover:underline"
						onclick={() => {
							totpStep = false;
							totpCode = '';
							tempToken = '';
							totpTouched = false;
							error = '';
						}}
					>
						Back to login
					</button>
				</form>
			{/if}
		</div>
	</div>
</div>
