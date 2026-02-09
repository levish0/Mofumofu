<!-- src/routes/account/set-handle/google/+page.svelte -->
<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { checkHandleAvailability } from '$lib/api/user/userApi';
	import { createPersonalInfoSchema } from '$lib/schemas/personal-info';
	import { safeParse } from 'valibot';
	import { goto } from '$app/navigation';
	import { ApiError } from '$lib/api/error/common_error';
	import { ExclamationTriangle, CheckCircle, Icon } from 'svelte-hero-icons';
	import * as m from '../../../../paraglide/messages';
	import { enhance } from '$app/forms';
	import type { ActionData } from './$types';

	let handle = $state('');
	let validationError = $state<string | undefined>();
	let verificationState = $state<'unverified' | 'checking' | 'verified' | 'unavailable'>('unverified');
	let proceeding = $state(false);
	let proceedError = $state<string | undefined>();

	let { data, form }: { data: any; form: ActionData } = $props();

	const characterCount = $derived(handle.length);
	const canCheck = $derived(handle.trim() !== '' && !validationError);
	const canProceed = $derived(handle.trim() !== '' && verificationState === 'verified' && !validationError);

	function validateHandle(value: string): string | undefined {
		const schema = createPersonalInfoSchema();
		const result = safeParse(schema.entries.handle, value.trim());
		return result.success ? undefined : result.issues?.[0]?.message;
	}

	function handleInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		handle = value;
		validationError = validateHandle(value);

		// Reset verification state when handle changes
		verificationState = 'unverified';
	}

	async function checkHandle() {
		if (!canCheck) return;

		verificationState = 'checking';
		try {
			const result = await checkHandleAvailability(handle.trim());
			verificationState = result.is_available ? 'verified' : 'unavailable';
		} catch (error) {
			console.error('Handle availability check failed:', error);
			verificationState = 'unverified';
		}
	}

	// Form submission is handled by server action

	function goBack() {
		history.back();
	}
</script>

<div class="text-mofu-dark-200 bg-mofu-dark-900 flex min-h-screen items-center justify-center">
	<div class="w-full max-w-md space-y-8 p-8">
		<div class="text-center">
			<div class="mb-6 flex items-center justify-center">
				<!-- Google 아이콘 -->
				<div class="bg-mofu-dark-800 mr-4 rounded-full p-3">
					<svg viewBox="0 0 24 24" aria-hidden="true" class="h-8 w-8">
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
				</div>
			</div>
			<h1 class="text-3xl font-bold">{m.oauth_signup_google_title()}</h1>
			<p class="text-mofu-dark-300 mt-2">{m.oauth_enter_handle()}</p>
		</div>

		<div class="space-y-6">
			<div class="space-y-2">
				<div class="relative flex">
					<span class="dark:bg-mofu-dark-800/50 text-mofu-dark-200 inline-flex items-center rounded-l-md px-3 text-sm"
						>@</span
					>
					<div class="relative flex-1">
						<Input
							id="handle"
							placeholder="mofumofu"
							class="dark:bg-mofu-dark-800 text-mofu-dark-200 placeholder:text-mofu-dark-300 rounded-l-none rounded-r-none border-r-0 pr-12 {validationError
								? 'border-red-500'
								: ''}"
							value={handle}
							oninput={handleInput}
						/>
						<div class="text-mofu-dark-400 absolute top-1/2 right-3 -translate-y-1/2 text-xs">
							{characterCount}/20
						</div>
					</div>
					<button
						onclick={checkHandle}
						disabled={verificationState === 'checking' || !canCheck}
						class="dark:bg-mofu-dark-800/50 text-mofu-dark-200 disabled:dark:bg-mofu-dark-800/50 hover:bg-mofu-dark-700 inline-flex min-w-20 items-center justify-center rounded-r-md px-3 py-2 text-sm font-medium disabled:cursor-not-allowed disabled:opacity-50"
					>
						{#if verificationState === 'checking'}
							<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path
									class="opacity-75"
									fill="currentColor"
									d="m4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
								></path>
							</svg>
						{:else}
							{m.oauth_check_button()}
						{/if}
					</button>
				</div>

				{#if validationError}
					<p class="flex items-center gap-1 text-xs text-rose-400">
						<Icon src={ExclamationTriangle} size="14" />
						{validationError}
					</p>
				{:else if verificationState === 'verified'}
					<p class="flex items-center gap-1 text-xs text-green-400">
						<Icon src={CheckCircle} size="14" />
						{m.oauth_handle_available()}
					</p>
				{:else if verificationState === 'unavailable'}
					<p class="flex items-center gap-1 text-xs text-rose-400">
						<Icon src={ExclamationTriangle} size="14" />
						{m.oauth_handle_unavailable()}
					</p>
				{:else if handle.trim() !== ''}
					<p class="text-xs text-orange-400">{m.oauth_handle_check_required()}</p>
				{:else}
					<p class="text-mofu-dark-400 text-xs">{m.oauth_handle_requirements()}</p>
				{/if}
			</div>

			{#if form?.error}
				<div
					class="flex items-center gap-2 rounded-md border border-rose-400/20 bg-rose-400/10 p-3 text-sm text-rose-400"
				>
					<Icon src={ExclamationTriangle} size="16" />
					{form.error}
				</div>
			{/if}

			<div class="space-y-3">
				<form method="POST" action="?/proceedWithGoogle" use:enhance>
					<input type="hidden" name="handle" value={handle} />
					<Button
						type="submit"
						disabled={!canProceed}
						class="flex w-full items-center justify-center gap-3 bg-[#4285F4] text-white hover:bg-[#4285F4]/80 disabled:opacity-50"
					>
						<!-- Google 아이콘 -->
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
						{m.oauth_continue_with_google()}
					</Button>
				</form>

				<Button
					onclick={goBack}
					variant="ghost"
					class="text-mofu-dark-300 hover:text-mofu-dark-200 hover:bg-mofu-dark-800 w-full"
				>
					← {m.common_back()}
				</Button>
			</div>
		</div>
	</div>
</div>
