<!-- src/routes/account/reset-password/+page.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { resetPassword } from '$lib/api/auth/authApi';
	import { goto } from '$app/navigation';
	import { ApiError } from '$lib/api/error/common_error';
	import { LockClosed, ExclamationTriangle, Icon } from 'svelte-hero-icons';

	let token = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let isSubmitting = $state(false);
	let error = $state<string | undefined>();
	let passwordValidationError = $state<string | undefined>();
	let confirmPasswordError = $state<string | undefined>();

	onMount(() => {
		const urlToken = page.url.searchParams.get('token');
		if (!urlToken) {
			error = '유효하지 않은 링크입니다. 비밀번호 재설정을 다시 요청해주세요.';
		} else {
			token = urlToken;
		}
	});

	function validatePassword(password: string): string | undefined {
		if (!password) {
			return '새 비밀번호를 입력해주세요';
		}
		if (password.length < 8) {
			return '비밀번호는 8자 이상이어야 합니다';
		}
		if (password.length > 128) {
			return '비밀번호는 128자 이하여야 합니다';
		}
		if (!/(?=.*[a-z])/.test(password)) {
			return '소문자를 포함해야 합니다';
		}
		if (!/(?=.*[A-Z])/.test(password)) {
			return '대문자를 포함해야 합니다';
		}
		if (!/(?=.*\d)/.test(password)) {
			return '숫자를 포함해야 합니다';
		}
		if (!/(?=.*[!@#$%^&*(),.?":{}|<>])/.test(password)) {
			return '특수문자를 포함해야 합니다';
		}
		return undefined;
	}

	function handlePasswordInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		password = value;
		passwordValidationError = validatePassword(value);

		// Revalidate confirm password if it's been entered
		if (confirmPassword) {
			confirmPasswordError = password !== confirmPassword ? '비밀번호가 일치하지 않습니다' : undefined;
		}
	}

	function handleConfirmPasswordInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		confirmPassword = value;
		confirmPasswordError = password !== value ? '비밀번호가 일치하지 않습니다' : undefined;
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();

		if (!token) {
			error = '유효하지 않은 토큰입니다';
			return;
		}

		const passwordError = validatePassword(password);
		if (passwordError) {
			passwordValidationError = passwordError;
			return;
		}

		if (password !== confirmPassword) {
			confirmPasswordError = '비밀번호가 일치하지 않습니다';
			return;
		}

		isSubmitting = true;
		error = undefined;

		try {
			await resetPassword(token, password);
			await goto('/account/password-reset-success');
		} catch (err) {
			console.error('Reset password error:', err);
			if (err instanceof ApiError) {
				error = err.message;
			} else {
				error = '비밀번호 재설정 중 오류가 발생했습니다';
			}
		} finally {
			isSubmitting = false;
		}
	}

	const canSubmit = $derived(
		token &&
			password &&
			confirmPassword &&
			!passwordValidationError &&
			!confirmPasswordError &&
			password === confirmPassword
	);
</script>

<div class="text-mofu-dark-200 bg-mofu-dark-900 flex min-h-screen items-center justify-center">
	<div class="w-full max-w-md space-y-8 p-8">
		<div class="text-center">
			<div class="mb-6 flex items-center justify-center">
				<div class="bg-mofu-dark-800 rounded-full p-3">
					<Icon src={LockClosed} size="32" class="text-mofu" />
				</div>
			</div>
			<h1 class="text-3xl font-bold">새 비밀번호 설정</h1>
			<p class="text-mofu-dark-300 mt-2">새로운 비밀번호를 입력해주세요.</p>
		</div>

		{#if !token}
			<div
				class="flex items-center gap-2 rounded-md border border-rose-400/20 bg-rose-400/10 p-3 text-sm text-rose-400"
			>
				<Icon src={ExclamationTriangle} size="16" />
				유효하지 않은 링크입니다. 비밀번호 재설정을 다시 요청해주세요.
			</div>
			<div class="space-y-3">
				<Button
					onclick={() => goto('/account/forgot-password')}
					class="bg-mofu text-mofu-dark-900 hover:bg-mofu/80 w-full"
				>
					비밀번호 재설정 다시 요청하기
				</Button>
			</div>
		{:else}
			<form onsubmit={handleSubmit} class="space-y-6">
				<div class="space-y-2">
					<label for="password" class="block text-sm font-medium">새 비밀번호</label>
					<Input
						id="password"
						type="password"
						placeholder="새 비밀번호"
						class="dark:bg-mofu-dark-800 text-mofu-dark-200 placeholder:text-mofu-dark-300 {passwordValidationError
							? 'border-red-500'
							: ''}"
						value={password}
						oninput={handlePasswordInput}
						autocomplete="new-password"
					/>

					{#if passwordValidationError}
						<p class="flex items-center gap-1 text-xs text-rose-400">
							<Icon src={ExclamationTriangle} size="14" />
							{passwordValidationError}
						</p>
					{:else}
						<p class="text-mofu-dark-400 text-xs">8자 이상, 대소문자, 숫자, 특수문자 포함</p>
					{/if}
				</div>

				<div class="space-y-2">
					<label for="confirmPassword" class="block text-sm font-medium">비밀번호 확인</label>
					<Input
						id="confirmPassword"
						type="password"
						placeholder="비밀번호 확인"
						class="dark:bg-mofu-dark-800 text-mofu-dark-200 placeholder:text-mofu-dark-300 {confirmPasswordError
							? 'border-red-500'
							: ''}"
						value={confirmPassword}
						oninput={handleConfirmPasswordInput}
						autocomplete="new-password"
					/>

					{#if confirmPasswordError}
						<p class="flex items-center gap-1 text-xs text-rose-400">
							<Icon src={ExclamationTriangle} size="14" />
							{confirmPasswordError}
						</p>
					{/if}
				</div>

				{#if error}
					<div
						class="flex items-center gap-2 rounded-md border border-rose-400/20 bg-rose-400/10 p-3 text-sm text-rose-400"
					>
						<Icon src={ExclamationTriangle} size="16" />
						{error}
					</div>
				{/if}

				<div class="space-y-3">
					<Button
						type="submit"
						disabled={!canSubmit || isSubmitting}
						class="bg-mofu text-mofu-dark-900 hover:bg-mofu/80 w-full disabled:opacity-50"
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
							설정 중...
						{:else}
							비밀번호 재설정
						{/if}
					</Button>
				</div>
			</form>
		{/if}
	</div>
</div>
