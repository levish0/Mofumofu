<!-- src/routes/account/forgot-password/+page.svelte -->
<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { forgotPassword } from '$lib/api/auth/authApi';
	import { goto } from '$app/navigation';
	import { ApiError } from '$lib/api/error/common_error';
	import { EnvelopeOpen, ExclamationTriangle, Icon } from 'svelte-hero-icons';
	import * as m from '../../../paraglide/messages';

	let email = $state('');
	let isSubmitting = $state(false);
	let error = $state<string | undefined>();
	let emailValidationError = $state<string | undefined>();

	function validateEmail(email: string): string | undefined {
		if (!email.trim()) {
			return '이메일을 입력해주세요';
		}
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!emailRegex.test(email.trim())) {
			return '올바른 이메일 형식이 아닙니다';
		}
		return undefined;
	}

	function handleEmailInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		email = value;
		emailValidationError = validateEmail(value);
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();

		const validationError = validateEmail(email);
		if (validationError) {
			emailValidationError = validationError;
			return;
		}

		isSubmitting = true;
		error = undefined;

		try {
			await forgotPassword(email.trim());
			await goto('/account/password-reset-sent');
		} catch (err) {
			console.error('Forgot password error:', err);
			if (err instanceof ApiError) {
				error = err.message;
			} else {
				error = '비밀번호 재설정 요청 중 오류가 발생했습니다';
			}
		} finally {
			isSubmitting = false;
		}
	}

	function goBack() {
		history.back();
	}

	const canSubmit = $derived(email.trim() !== '' && !emailValidationError);
</script>

<div class="text-mofu-dark-200 bg-mofu-dark-900 flex min-h-screen items-center justify-center">
	<div class="w-full max-w-md space-y-8 p-8">
		<div class="text-center">
			<div class="mb-6 flex items-center justify-center">
				<div class="bg-mofu-dark-800 rounded-full p-3">
					<Icon src={EnvelopeOpen} size="32" class="text-mofu" />
				</div>
			</div>
			<h1 class="text-3xl font-bold">비밀번호 찾기</h1>
			<p class="text-mofu-dark-300 mt-2">가입하신 이메일 주소를 입력해주세요. 비밀번호 재설정 링크를 보내드립니다.</p>
		</div>

		<form onsubmit={handleSubmit} class="space-y-6">
			<div class="space-y-2">
				<Input
					id="email"
					type="email"
					placeholder="email@mofu.com"
					class="dark:bg-mofu-dark-800 text-mofu-dark-200 placeholder:text-mofu-dark-300 {emailValidationError
						? 'border-red-500'
						: ''}"
					value={email}
					oninput={handleEmailInput}
					autocomplete="email"
				/>

				{#if emailValidationError}
					<p class="flex items-center gap-1 text-xs text-rose-400">
						<Icon src={ExclamationTriangle} size="14" />
						{emailValidationError}
					</p>
				{:else if email.trim() !== ''}
					<p class="text-mofu-dark-400 text-xs">입력하신 이메일로 재설정 링크를 보내드립니다</p>
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
						전송 중...
					{:else}
						재설정 링크 보내기
					{/if}
				</Button>

				<Button
					onclick={goBack}
					variant="ghost"
					class="text-mofu-dark-300 hover:text-mofu-dark-200 hover:bg-mofu-dark-800 w-full"
				>
					← 돌아가기
				</Button>
			</div>
		</form>

		<div class="text-center">
			<p class="text-mofu-dark-300 text-sm">
				계정이 기억나셨나요?
				<a href="/account/signin" class="text-mofu rounded-lg font-semibold hover:opacity-70">로그인하기</a>
			</p>
		</div>
	</div>
</div>
