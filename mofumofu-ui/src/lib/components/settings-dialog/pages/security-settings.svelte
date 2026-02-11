<script lang="ts">
	import { onMount } from 'svelte';
	import { isApiError } from '$lib/api';
	import {
		totpStatus,
		totpSetup,
		totpEnable,
		totpDisable,
		totpRegenerateBackupCodes
	} from '$lib/api/auth';
	import {
		listOAuthConnections,
		unlinkOAuth,
		getGoogleAuthUrl,
		getGithubAuthUrl
	} from '$lib/api/auth';
	import type {
		TotpStatusResponse,
		TotpSetupResponse,
		OAuthConnectionListResponse,
		OAuthProvider
	} from '$lib/api/types';
	import { Button } from '$lib/components/ui/button';
	import * as FieldSet from '$lib/components/ui/field-set';
	import * as InputOTP from '$lib/components/ui/input-otp';
	import { CopyButton } from '$lib/components/ui/copy-button';
	import { toast } from 'svelte-sonner';

	// --- TOTP ---
	let totpState = $state<TotpStatusResponse | null>(null);
	let totpLoading = $state(true);

	// Setup flow
	let setupData = $state<TotpSetupResponse | null>(null);
	let setupCode = $state('');
	let setupLoading = $state(false);
	let backupCodes = $state<string[] | null>(null);

	// Disable flow
	let disableMode = $state(false);
	let disableCode = $state('');
	let disableLoading = $state(false);

	// Regenerate flow
	let regenMode = $state(false);
	let regenCode = $state('');
	let regenLoading = $state(false);

	// --- OAuth ---
	let oauthConnections = $state<OAuthConnectionListResponse | null>(null);
	let oauthLoading = $state(true);

	onMount(async () => {
		try {
			totpState = await totpStatus();
		} catch {
			// ignore
		} finally {
			totpLoading = false;
		}

		try {
			oauthConnections = await listOAuthConnections();
		} catch {
			// ignore
		} finally {
			oauthLoading = false;
		}
	});

	async function handleSetupStart() {
		setupLoading = true;
		try {
			setupData = await totpSetup();
		} catch (err) {
			if (isApiError(err)) toast.error(err.details ?? 'Setup failed.');
			else toast.error('Setup failed.');
		} finally {
			setupLoading = false;
		}
	}

	async function handleSetupEnable() {
		if (setupCode.length !== 6) return;
		setupLoading = true;
		try {
			const result = await totpEnable(setupCode);
			backupCodes = result.backup_codes;
			setupData = null;
			setupCode = '';
			totpState = await totpStatus();
			toast.success('Two-factor authentication enabled.');
		} catch (err) {
			setupCode = '';
			if (isApiError(err)) toast.error(err.details ?? 'Invalid code.');
			else toast.error('Invalid code.');
		} finally {
			setupLoading = false;
		}
	}

	async function handleDisable() {
		if (disableCode.length !== 6) return;
		disableLoading = true;
		try {
			await totpDisable(disableCode);
			totpState = await totpStatus();
			disableMode = false;
			disableCode = '';
			toast.success('Two-factor authentication disabled.');
		} catch (err) {
			disableCode = '';
			if (isApiError(err)) toast.error(err.details ?? 'Invalid code.');
			else toast.error('Invalid code.');
		} finally {
			disableLoading = false;
		}
	}

	async function handleRegenerate() {
		if (regenCode.length !== 6) return;
		regenLoading = true;
		try {
			const result = await totpRegenerateBackupCodes(regenCode);
			backupCodes = result.backup_codes;
			regenMode = false;
			regenCode = '';
			totpState = await totpStatus();
			toast.success('Backup codes regenerated.');
		} catch (err) {
			regenCode = '';
			if (isApiError(err)) toast.error(err.details ?? 'Invalid code.');
			else toast.error('Invalid code.');
		} finally {
			regenLoading = false;
		}
	}

	function isConnected(provider: OAuthProvider) {
		return oauthConnections?.connections.some((c) => c.provider === provider) ?? false;
	}

	async function handleOAuthLink(provider: 'google' | 'github') {
		try {
			const { auth_url } =
				provider === 'google' ? await getGoogleAuthUrl() : await getGithubAuthUrl();
			sessionStorage.setItem('oauth_link_mode', 'link');
			window.location.href = auth_url;
		} catch {
			toast.error(`Failed to connect to ${provider}.`);
		}
	}

	async function handleOAuthUnlink(provider: OAuthProvider) {
		try {
			await unlinkOAuth(provider);
			oauthConnections = await listOAuthConnections();
			toast.success(`${provider} disconnected.`);
		} catch (err) {
			if (isApiError(err)) toast.error(err.details ?? 'Failed to disconnect.');
			else toast.error('Failed to disconnect.');
		}
	}
</script>

<div class="mx-auto max-w-2xl space-y-6">
	<!-- TOTP Section -->
	<section class="space-y-3">
		<h4 class="text-base font-semibold">Two-Factor Authentication</h4>
		<FieldSet.Root class="bg-mofu-light-900 dark:bg-mofu-dark-900">
			<FieldSet.Content>
				{#if totpLoading}
					<p class="text-sm text-muted-foreground">Loading...</p>
				{:else if backupCodes}
					<!-- Show backup codes after enable/regenerate -->
					<div class="space-y-3">
						<p class="font-medium">Save your backup codes</p>
						<p class="text-sm text-muted-foreground">
							Store these codes in a safe place. Each code can only be used once.
						</p>
						<div class="grid grid-cols-2 gap-2 rounded-md bg-muted p-3 font-mono text-sm">
							{#each backupCodes as code}
								<span>{code}</span>
							{/each}
						</div>
						<div class="flex gap-2">
							<CopyButton text={backupCodes.join('\n')} variant="outline" size="sm">
								Copy codes
							</CopyButton>
							<Button size="sm" onclick={() => (backupCodes = null)}>Done</Button>
						</div>
					</div>
				{:else if !totpState?.enabled}
					<!-- TOTP not enabled -->
					{#if setupData}
						<!-- QR code + verify -->
						<div class="space-y-4">
							<p class="text-sm text-muted-foreground">
								Scan this QR code with your authenticator app, then enter the code below.
							</p>
							<div class="flex justify-center">
								<img
									src="data:image/png;base64,{setupData.qr_code_base64}"
									alt="TOTP QR Code"
									class="h-48 w-48 rounded-lg"
								/>
							</div>
							<div class="flex items-center gap-2 text-xs text-muted-foreground">
								<span class="truncate font-mono">{setupData.qr_code_uri}</span>
								<CopyButton text={setupData.qr_code_uri} variant="ghost" size="icon" />
							</div>
							<div class="flex flex-col items-center space-y-2">
								<p class="text-sm font-medium">Verification Code</p>
								<InputOTP.Root maxlength={6} bind:value={setupCode} onComplete={handleSetupEnable}>
									{#snippet children({ cells })}
										<InputOTP.Group>
											{#each cells.slice(0, 3) as cell (cell)}
												<InputOTP.Slot {cell} />
											{/each}
										</InputOTP.Group>
										<InputOTP.Separator />
										<InputOTP.Group>
											{#each cells.slice(3, 6) as cell (cell)}
												<InputOTP.Slot {cell} />
											{/each}
										</InputOTP.Group>
									{/snippet}
								</InputOTP.Root>
							</div>
							<div class="flex justify-end gap-2">
								<Button
									variant="ghost"
									size="sm"
									onclick={() => {
										setupData = null;
										setupCode = '';
									}}
								>
									Cancel
								</Button>
								<Button size="sm" disabled={setupLoading} onclick={handleSetupEnable}>
									{setupLoading ? 'Verifying...' : 'Enable'}
								</Button>
							</div>
						</div>
					{:else}
						<div class="space-y-4">
							<p class="text-sm text-muted-foreground">
								Add an extra layer of security to your account using a TOTP authenticator app.
							</p>
							<Button size="sm" disabled={setupLoading} onclick={handleSetupStart}>
								{setupLoading ? 'Setting up...' : 'Set up 2FA'}
							</Button>
						</div>
					{/if}
				{:else}
					<!-- TOTP enabled -->
					<div class="space-y-4">
						<div class="flex items-center gap-2">
							<span class="inline-block size-2 rounded-full bg-green-500"></span>
							<span class="font-medium">Enabled</span>
						</div>
						{#if totpState.backup_codes_remaining != null}
							<p class="text-sm text-muted-foreground">
								{totpState.backup_codes_remaining} backup codes remaining
							</p>
						{/if}

						{#if regenMode}
							<div class="flex flex-col items-center space-y-3">
								<p class="text-sm text-muted-foreground">
									Enter TOTP code to regenerate backup codes
								</p>
								<InputOTP.Root maxlength={6} bind:value={regenCode} onComplete={handleRegenerate}>
									{#snippet children({ cells })}
										<InputOTP.Group>
											{#each cells.slice(0, 3) as cell (cell)}
												<InputOTP.Slot {cell} />
											{/each}
										</InputOTP.Group>
										<InputOTP.Separator />
										<InputOTP.Group>
											{#each cells.slice(3, 6) as cell (cell)}
												<InputOTP.Slot {cell} />
											{/each}
										</InputOTP.Group>
									{/snippet}
								</InputOTP.Root>
								<div class="flex w-full justify-end">
									<Button
										variant="ghost"
										size="sm"
										onclick={() => {
											regenMode = false;
											regenCode = '';
										}}
									>
										Cancel
									</Button>
								</div>
							</div>
						{:else if disableMode}
							<div class="flex flex-col items-center space-y-3">
								<p class="text-sm text-muted-foreground">Enter TOTP code to disable 2FA</p>
								<InputOTP.Root maxlength={6} bind:value={disableCode} onComplete={handleDisable}>
									{#snippet children({ cells })}
										<InputOTP.Group>
											{#each cells.slice(0, 3) as cell (cell)}
												<InputOTP.Slot {cell} />
											{/each}
										</InputOTP.Group>
										<InputOTP.Separator />
										<InputOTP.Group>
											{#each cells.slice(3, 6) as cell (cell)}
												<InputOTP.Slot {cell} />
											{/each}
										</InputOTP.Group>
									{/snippet}
								</InputOTP.Root>
								<div class="flex w-full justify-end">
									<Button
										variant="ghost"
										size="sm"
										onclick={() => {
											disableMode = false;
											disableCode = '';
										}}
									>
										Cancel
									</Button>
								</div>
							</div>
						{:else}
							<div class="flex gap-2">
								<Button variant="outline" size="sm" onclick={() => (regenMode = true)}>
									Regenerate backup codes
								</Button>
								<Button variant="destructive" size="sm" onclick={() => (disableMode = true)}>
									Disable 2FA
								</Button>
							</div>
						{/if}
					</div>
				{/if}
			</FieldSet.Content>
		</FieldSet.Root>
	</section>

	<!-- OAuth Section -->
	<section class="space-y-3">
		<h4 class="text-base font-semibold">Connected Accounts</h4>
		<FieldSet.Root class="bg-mofu-light-900 dark:bg-mofu-dark-900">
			<FieldSet.Content>
				{#if oauthLoading}
					<p class="text-sm text-muted-foreground">Loading...</p>
				{:else}
					<div class="space-y-3">
						<!-- Google -->
						<div class="flex items-center justify-between">
							<div class="flex items-center gap-3">
								<svg viewBox="0 0 24 24" aria-hidden="true" class="size-5">
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
								<span class="font-medium">Google</span>
							</div>
							{#if isConnected('Google')}
								<Button variant="outline" size="sm" onclick={() => handleOAuthUnlink('Google')}>
									Unlink
								</Button>
							{:else}
								<Button variant="outline" size="sm" onclick={() => handleOAuthLink('google')}>
									Link
								</Button>
							{/if}
						</div>

						<div class="border-t"></div>

						<!-- GitHub -->
						<div class="flex items-center justify-between">
							<div class="flex items-center gap-3">
								<svg viewBox="0 0 20 20" aria-hidden="true" class="size-5 fill-current">
									<path
										d="M10 0C4.477 0 0 4.484 0 10.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0110 4.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.942.359.31.678.921.678 1.856 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0020 10.017C20 4.484 15.522 0 10 0z"
										clip-rule="evenodd"
										fill-rule="evenodd"
									/>
								</svg>
								<span class="font-medium">GitHub</span>
							</div>
							{#if isConnected('Github')}
								<Button variant="outline" size="sm" onclick={() => handleOAuthUnlink('Github')}>
									Unlink
								</Button>
							{:else}
								<Button variant="outline" size="sm" onclick={() => handleOAuthLink('github')}>
									Link
								</Button>
							{/if}
						</div>
					</div>
				{/if}
			</FieldSet.Content>
		</FieldSet.Root>
	</section>
</div>
