<!-- src/routes/account/set-handle/github/+page.svelte -->
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

	function goBack() {
		history.back();
	}
</script>

<div class="text-mofu-dark-200 bg-mofu-dark-900 flex min-h-screen items-center justify-center">
	<div class="w-full max-w-md space-y-8 p-8">
		<div class="text-center">
			<div class="mb-6 flex items-center justify-center">
				<!-- GitHub 아이콘 -->
				<div class="bg-mofu-dark-800 mr-4 rounded-full p-3">
					<svg viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" class="size-8 fill-white">
						<path
							d="M10 0C4.477 0 0 4.484 0 10.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0110 4.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.942.359.31.678.921.678 1.856 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0020 10.017C20 4.484 15.522 0 10 0z"
							clip-rule="evenodd"
							fill-rule="evenodd"
						/>
					</svg>
				</div>
			</div>
			<h1 class="text-3xl font-bold">{m.oauth_signup_github_title()}</h1>
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
				<form method="POST" action="?/proceedWithGitHub" use:enhance>
					<input type="hidden" name="handle" value={handle} />
					<Button
						type="submit"
						disabled={!canProceed}
						class="flex w-full items-center justify-center gap-3 bg-[#24292e] text-white hover:bg-[#24292e]/80 disabled:opacity-50"
					>
						<!-- GitHub 아이콘 -->
						<svg viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" class="size-5 fill-white">
							<path
								d="M10 0C4.477 0 0 4.484 0 10.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0110 4.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.942.359.31.678.921.678 1.856 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0020 10.017C20 4.484 15.522 0 10 0z"
								clip-rule="evenodd"
								fill-rule="evenodd"
							/>
						</svg>
						{m.oauth_continue_with_github()}
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
