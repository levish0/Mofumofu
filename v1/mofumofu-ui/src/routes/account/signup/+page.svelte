<script lang="ts">
	import { checkHandleAvailability } from '$lib/api/user/userApi';
	import { signup } from '$lib/api/auth/authApi';
	import { createSignupSchema } from '$lib/schemas/signup';
	import { safeParse } from 'valibot';
	import { ExclamationTriangle, CheckCircle, Icon } from 'svelte-hero-icons';
	import { goto } from '$app/navigation';
	import { ApiError } from '$lib/api/error/common_error';
	import * as m from '../../../paraglide/messages';

	let name = $state('');
	let email = $state('');
	let handle = $state('');
	let password = $state('');

	let nameValidationError = $state<string | undefined>();
	let emailValidationError = $state<string | undefined>();
	let handleValidationError = $state<string | undefined>();
	let passwordValidationError = $state<string | undefined>();
	let handleVerificationState = $state<'unverified' | 'checking' | 'verified' | 'unavailable'>('unverified');
	let isSubmitting = $state(false);
	let submitError = $state<string | undefined>();

	const characterCount = $derived(handle.length);
	const canCheckHandle = $derived(handle.trim() !== '' && !handleValidationError);

	function validateField(field: 'name' | 'email' | 'handle' | 'password', value: string): string | undefined {
		const schema = createSignupSchema();
		const result = safeParse(schema.entries[field], value.trim());
		return result.success ? undefined : result.issues?.[0]?.message;
	}

	function handleNameInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		name = value;
		nameValidationError = validateField('name', value);
	}

	function handleEmailInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		email = value;
		emailValidationError = validateField('email', value);
	}

	function handleHandleInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		handle = value;
		handleValidationError = validateField('handle', value);

		// Reset verification state when handle changes
		handleVerificationState = 'unverified';
	}

	function handlePasswordInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		password = value;
		passwordValidationError = validateField('password', value);
	}

	async function checkHandle() {
		if (!canCheckHandle) return;

		handleVerificationState = 'checking';
		try {
			const result = await checkHandleAvailability(handle.trim());
			handleVerificationState = result.is_available ? 'verified' : 'unavailable';
		} catch (error) {
			console.error('Handle availability check failed:', error);
			handleVerificationState = 'unverified';
		}
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();

		if (handleVerificationState !== 'verified') {
			submitError = m.signup_handle_check_required();
			return;
		}

		const schema = createSignupSchema();
		const result = safeParse(schema, { name: name.trim(), email: email.trim(), handle: handle.trim(), password });

		if (!result.success) {
			submitError = m.signup_input_validation_error();
			return;
		}

		isSubmitting = true;
		submitError = undefined;

		try {
			await signup({
				name: name.trim(),
				email: email.trim(),
				handle: handle.trim(),
				password
			});

			await goto('/account/email-sent');
		} catch (error) {
			console.error('Signup error:', error);
			if (error instanceof ApiError) {
				submitError = error.message;
			} else {
				submitError = m.signup_error_occurred();
			}
		} finally {
			isSubmitting = false;
		}
	}

	function goBack() {
		history.back();
	}
</script>

<div class="flex min-h-full">
	<!-- 왼쪽 이미지 영역 -->
	<div class="relative hidden w-0 flex-1 items-center justify-center overflow-hidden lg:flex">
		<a href="/" class="block">
			<img src="/mofumofu_kawaii_mini.svg" alt="Background" class="h-auto w-[500px] object-contain" />
		</a>
	</div>
	<!-- 오른쪽 로그인 폼 영역 -->
	<div
		class="bg-mofu-dark-900 relative flex flex-1 flex-col justify-center px-4 py-12 shadow-lg sm:px-6 lg:flex-none lg:px-20 xl:px-24"
	>
		<!-- 로그인 폼 콘텐츠 -->
		<div class="mx-auto w-full max-w-sm lg:w-96">
			<div class="flex justify-start text-center">
				<button onclick={goBack} class="text-mofu-dark-300 text-sm hover:opacity-70">{m.auth_go_back()}</button>
			</div>
			<div>
				<h2 class="mt-6 text-2xl/9 font-bold tracking-tight text-white">{m.auth_sign_up()}</h2>
				<p class="text-mofu-dark-300 mt-2 text-sm/6">
					{m.auth_already_have_account()}
					<a href="/account/signin" class="text-mofu rounded-lg font-semibold hover:opacity-70">{m.auth_sign_in()}</a>
				</p>
			</div>

			<div class="mt-6">
				<form onsubmit={handleSubmit} class="space-y-4">
					<div>
						<label for="name" class="block text-sm/6 font-medium">{m.signup_name_label()}</label>
						<div class="mt-2 space-y-2">
							<input
								id="name"
								type="text"
								name="name"
								required
								placeholder={m.signup_name_placeholder()}
								value={name}
								oninput={handleNameInput}
								autocomplete="name"
								class="bg-mofu-dark-800 placeholder:text-mofu-dark-300 block w-full rounded-lg px-3 py-1.5 text-base outline-none sm:text-sm/6 {nameValidationError
									? 'border-red-500'
									: ''}"
							/>
							{#if nameValidationError}
								<p class="flex items-center gap-1 text-xs text-rose-400">
									<Icon src={ExclamationTriangle} size="14" />
									{nameValidationError}
								</p>
							{/if}
						</div>
					</div>

					<div>
						<label for="email" class="block text-sm/6 font-medium">{m.auth_email_address()}</label>
						<div class="mt-2 space-y-2">
							<input
								id="email"
								type="email"
								name="email"
								required
								placeholder={m.auth_email_placeholder()}
								value={email}
								oninput={handleEmailInput}
								autocomplete="email"
								class="bg-mofu-dark-800 placeholder:text-mofu-dark-300 block w-full rounded-lg px-3 py-1.5 text-base outline-none sm:text-sm/6 {emailValidationError
									? 'border-red-500'
									: ''}"
							/>
							{#if emailValidationError}
								<p class="flex items-center gap-1 text-xs text-rose-400">
									<Icon src={ExclamationTriangle} size="14" />
									{emailValidationError}
								</p>
							{/if}
						</div>
					</div>

					<div>
						<label for="handle" class="block text-sm/6 font-medium">{m.settings_handle()}</label>
						<div class="mt-2 space-y-2">
							<div class="relative flex">
								<span class="bg-mofu-dark-800 inline-flex items-center rounded-l-lg border-r-0 px-3 text-sm">@</span>
								<div class="relative flex-1">
									<input
										id="handle"
										type="text"
										name="handle"
										required
										placeholder={m.settings_handle_placeholder()}
										value={handle}
										oninput={handleHandleInput}
										class="bg-mofu-dark-800 placeholder:text-mofu-dark-300 block w-full rounded-l-none rounded-r-none border-r-0 px-3 py-1.5 pr-12 text-base outline-none sm:text-sm/6 {handleValidationError
											? 'border-red-500'
											: ''}"
									/>
									<div class="text-mofu-dark-400 absolute top-1/2 right-3 -translate-y-1/2 text-xs">
										{characterCount}/20
									</div>
								</div>
								<button
									type="button"
									onclick={checkHandle}
									disabled={handleVerificationState === 'checking' || !canCheckHandle}
									class="bg-mofu-dark-800 text-mofu-dark-200 disabled:bg-mofu-dark-800/50 hover:bg-mofu-dark-700 inline-flex min-w-20 items-center justify-center rounded-r-lg px-3 py-1.5 text-sm font-medium disabled:cursor-not-allowed disabled:opacity-50"
								>
									{#if handleVerificationState === 'checking'}
										<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
											<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
											<path
												class="opacity-75"
												fill="currentColor"
												d="m4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
											></path>
										</svg>
									{:else}
										{m.settings_handle_check()}
									{/if}
								</button>
							</div>

							{#if handleValidationError}
								<p class="flex items-center gap-1 text-xs text-rose-400">
									<Icon src={ExclamationTriangle} size="14" />
									{handleValidationError}
								</p>
							{:else if handleVerificationState === 'verified'}
								<p class="flex items-center gap-1 text-xs text-green-400">
									<Icon src={CheckCircle} size="14" />
									{m.settings_handle_available()}
								</p>
							{:else if handleVerificationState === 'unavailable'}
								<p class="flex items-center gap-1 text-xs text-rose-400">
									<Icon src={ExclamationTriangle} size="14" />
									{m.settings_handle_taken()}
								</p>
							{:else if handle.trim() !== ''}
								<p class="text-xs text-orange-400">{m.settings_handle_verification_required()}</p>
							{:else}
								<p class="text-mofu-dark-400 text-xs">{m.settings_handle_description()}</p>
							{/if}
						</div>
					</div>

					<div>
						<label for="password" class="block text-sm/6 font-medium">{m.auth_password()}</label>
						<div class="mt-2 space-y-2">
							<input
								id="password"
								type="password"
								name="password"
								placeholder={m.auth_password_placeholder()}
								value={password}
								oninput={handlePasswordInput}
								required
								autocomplete="new-password"
								class="bg-mofu-dark-800 placeholder:text-mofu-dark-300 block w-full rounded-lg px-3 py-1.5 text-base outline-none sm:text-sm/6 {passwordValidationError
									? 'border-red-500'
									: ''}"
							/>
							{#if passwordValidationError}
								<p class="flex items-center gap-1 text-xs text-rose-400">
									<Icon src={ExclamationTriangle} size="14" />
									{passwordValidationError}
								</p>
							{:else}
								<p class="text-mofu-dark-400 text-xs">{m.auth_password_requirements()}</p>
							{/if}
						</div>
					</div>
					{#if submitError}
						<div class="rounded-lg border border-red-500/20 bg-red-900/20 p-3">
							<p class="flex items-center gap-1 text-xs text-rose-400">
								<Icon src={ExclamationTriangle} size="14" />
								{submitError}
							</p>
						</div>
					{/if}

					<div>
						<button
							type="submit"
							disabled={isSubmitting || handleVerificationState !== 'verified'}
							class="bg-mofu text-mofu-dark-900 disabled:bg-mofu/50 mt-6 flex w-full justify-center rounded-lg px-3 py-1.5 text-sm/6 font-semibold shadow-xs outline-none hover:opacity-70 disabled:cursor-not-allowed disabled:hover:opacity-100"
						>
							{#if isSubmitting}
								<svg class="mr-2 h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
									<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
									<path
										class="opacity-75"
										fill="currentColor"
										d="m4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
									></path>
								</svg>
								{m.signup_signing_up()}
							{:else}
								{m.auth_sign_up_button()}
							{/if}
						</button>
					</div>
				</form>
			</div>

			<div class="mt-4">
				<div class="relative">
					<div aria-hidden="true" class="absolute inset-0 flex items-center">
						<div class="border-mofu-dark-700 w-full border-t"></div>
					</div>
					<div class="relative flex justify-center text-sm/6 font-medium">
						<span class="bg-mofu-dark-900 px-6">{m.auth_or()}</span>
					</div>
				</div>

				<div class="mt-6 grid grid-cols-2 gap-4">
					<a
						href="/account/set-handle/google"
						class="bg-mofu-dark-800 flex w-full items-center justify-center gap-3 rounded-lg px-3 py-2 text-sm font-semibold hover:opacity-70"
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
						<span class="text-sm/6 font-semibold">Google</span>
					</a>

					<a
						href="/account/set-handle/github"
						class="bg-mofu-dark-800 flex w-full items-center justify-center gap-3 rounded-lg px-3 py-2 text-sm font-semibold hover:opacity-70"
					>
						<!-- GitHub 아이콘 -->
						<svg viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" class="size-5 fill-[#FFFFFF]">
							<path
								d="M10 0C4.477 0 0 4.484 0 10.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0110 4.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.942.359.31.678.921.678 1.856 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0020 10.017C20 4.484 15.522 0 10 0z"
								clip-rule="evenodd"
								fill-rule="evenodd"
							/>
						</svg>
						<span class="text-sm/6 font-semibold">GitHub</span>
					</a>
				</div>
			</div>
		</div>
	</div>
</div>
