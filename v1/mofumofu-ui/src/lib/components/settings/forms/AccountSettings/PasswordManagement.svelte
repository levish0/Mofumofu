<script lang="ts">
	import { Input } from '../../../ui/input';
	import { Button } from '../../../ui/button';
	import { setPassword } from '$lib/api/auth/authApi';
	import { updateProfile } from '$lib/api/user/userApi';
	import type { UpdateProfileRequest } from '$lib/api/user/types';
	import { toast } from 'svelte-sonner';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import * as v from 'valibot';

	let currentPassword = $state('');
	let newPassword = $state('');
	let confirmPassword = $state('');
	let isProcessing = $state(false);
	const isOAuthOnly = $derived(settingsStore.account.isOAuthOnly);
	let showPasswordFields = $state(false);

	const passwordSchema = v.pipe(
		v.string(),
		v.minLength(8, '비밀번호는 최소 8자 이상이어야 합니다.'),
		v.regex(/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/, '비밀번호는 대문자, 소문자, 숫자를 포함해야 합니다.')
	);

	const newPasswordError = $derived(() => {
		if (!newPassword) return undefined;
		const result = v.safeParse(passwordSchema, newPassword);
		return result.success ? undefined : result.issues?.[0]?.message;
	});

	const confirmPasswordError = $derived(() => {
		if (!confirmPassword) return undefined;
		return newPassword !== confirmPassword ? '비밀번호가 일치하지 않습니다.' : undefined;
	});

	const currentPasswordError = $derived(() => {
		if (!isOAuthOnly && !currentPassword && showPasswordFields) {
			return '현재 비밀번호를 입력해주세요.';
		}
		return undefined;
	});

	const isFormValid = $derived(
		newPassword && confirmPassword && !newPasswordError() && !confirmPasswordError() && (isOAuthOnly || currentPassword)
	);

	async function handlePasswordAction() {
		if (!isFormValid) return;

		isProcessing = true;

		try {
			if (isOAuthOnly) {
				// OAuth 전용 계정 - 새 비밀번호 설정
				await setPassword(newPassword);
				// isOAuthOnly 상태를 업데이트 (이제 OAuth 전용이 아님)
				settingsStore.updateAccountSilent({ isOAuthOnly: false });
				toast.success('비밀번호가 성공적으로 설정되었습니다.');
			} else {
				// 기존 계정 - 비밀번호 변경
				const updateData: UpdateProfileRequest = {
					password: newPassword,
					current_password: currentPassword
				};
				await updateProfile(updateData);
				toast.success('비밀번호가 성공적으로 변경되었습니다.');
			}

			// 폼 초기화
			currentPassword = '';
			newPassword = '';
			confirmPassword = '';
			showPasswordFields = false;
		} catch (error) {
			console.error('Failed to update password:', error);
			if (isOAuthOnly) {
				toast.error('비밀번호 설정에 실패했습니다. 다시 시도해주세요.');
			} else {
				toast.error('비밀번호 변경에 실패했습니다. 현재 비밀번호를 확인해주세요.');
			}
		} finally {
			isProcessing = false;
		}
	}

	function handleShowPasswordFields() {
		showPasswordFields = true;
	}

	function handleCancel() {
		showPasswordFields = false;
		currentPassword = '';
		newPassword = '';
		confirmPassword = '';
	}
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">
		{isOAuthOnly ? '비밀번호 설정' : '비밀번호 변경'}
	</h2>
	<div class="space-y-3">
		{#if !showPasswordFields}
			<div
				class="border-mofu-light-700 dark:border-mofu-dark-700 bg-mofu-light-800 dark:bg-mofu-dark-800 rounded-lg border p-4"
			>
				<div class="flex items-start space-x-3">
					<div class="flex-shrink-0">
						{#if isOAuthOnly}
							<svg class="h-5 w-5 text-orange-400" fill="currentColor" viewBox="0 0 20 20">
								<path
									fill-rule="evenodd"
									d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
									clip-rule="evenodd"
								/>
							</svg>
						{:else}
							<svg class="h-5 w-5 text-blue-400" fill="currentColor" viewBox="0 0 20 20">
								<path
									fill-rule="evenodd"
									d="M18 8a6 6 0 01-7.743 5.743L10 14l-1 1-1 1H6v2H2v-4l4.257-4.257A6 6 0 1118 8zm-6-4a1 1 0 100 2 2 2 0 012 2 1 1 0 102 0 4 4 0 00-4-4z"
									clip-rule="evenodd"
								/>
							</svg>
						{/if}
					</div>
					<div class="flex-1">
						<h3 class="text-sm font-medium {isOAuthOnly ? 'text-orange-400' : 'text-blue-400'}">
							{isOAuthOnly ? '소셜 로그인 전용 계정' : '비밀번호 변경'}
						</h3>
						<p class="text-mofu-light-400 dark:text-mofu-dark-400 mt-1 text-sm">
							{isOAuthOnly
								? '현재 소셜 로그인만으로 가입된 계정입니다. 비밀번호를 설정하면 이메일과 비밀번호로도 로그인할 수 있습니다.'
								: '보안을 위해 정기적으로 비밀번호를 변경하는 것을 권장합니다.'}
						</p>
						<div class="mt-3">
							<Button class="h-8 px-3 text-sm" onclick={handleShowPasswordFields}>
								{isOAuthOnly ? '비밀번호 설정하기' : '비밀번호 변경하기'}
							</Button>
						</div>
					</div>
				</div>
			</div>
		{:else}
			<div class="space-y-4">
				{#if !isOAuthOnly}
					<div class="space-y-2">
						<label for="currentPassword" class="text-mofu-light-200 dark:text-mofu-dark-200 text-sm font-medium">
							현재 비밀번호
						</label>
						<Input
							id="currentPassword"
							type="password"
							placeholder="현재 비밀번호를 입력하세요"
							class="dark:bg-mofu-dark-800 bg-mofu-light-800 text-mofu-light-200 dark:text-mofu-dark-200 placeholder:text-mofu-light-300 dark:placeholder:text-mofu-dark-300 {currentPasswordError()
								? 'border-rose-500'
								: ''}"
							bind:value={currentPassword}
						/>
						{#if currentPasswordError()}
							<p class="text-xs text-rose-400">{currentPasswordError()}</p>
						{/if}
					</div>
				{/if}

				<div class="space-y-2">
					<label for="newPassword" class="text-mofu-light-200 dark:text-mofu-dark-200 text-sm font-medium">
						{isOAuthOnly ? '새 비밀번호' : '새 비밀번호'}
					</label>
					<Input
						id="newPassword"
						type="password"
						placeholder="새 비밀번호를 입력하세요"
						class="dark:bg-mofu-dark-800 bg-mofu-light-800 text-mofu-light-200 dark:text-mofu-dark-200 placeholder:text-mofu-light-300 dark:placeholder:text-mofu-dark-300 {newPasswordError()
							? 'border-rose-500'
							: ''}"
						bind:value={newPassword}
					/>
					{#if newPasswordError()}
						<p class="text-xs text-rose-400">{newPasswordError()}</p>
					{/if}
				</div>

				<div class="space-y-2">
					<label for="confirmPassword" class="text-mofu-light-200 dark:text-mofu-dark-200 text-sm font-medium">
						비밀번호 확인
					</label>
					<Input
						id="confirmPassword"
						type="password"
						placeholder="비밀번호를 다시 입력하세요"
						class="dark:bg-mofu-dark-800 bg-mofu-light-800 text-mofu-light-200 dark:text-mofu-dark-200 placeholder:text-mofu-light-300 dark:placeholder:text-mofu-dark-300 {confirmPasswordError()
							? 'border-rose-500'
							: ''}"
						bind:value={confirmPassword}
					/>
					{#if confirmPasswordError()}
						<p class="text-xs text-rose-400">{confirmPasswordError()}</p>
					{/if}
				</div>

				<div class="flex space-x-3">
					<Button onclick={handlePasswordAction} disabled={!isFormValid || isProcessing}>
						{#if isProcessing}
							<div class="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent"></div>
						{:else}
							{isOAuthOnly ? '비밀번호 설정' : '비밀번호 변경'}
						{/if}
					</Button>
					<Button variant="outline" onclick={handleCancel}>취소</Button>
				</div>
			</div>
		{/if}
	</div>
</div>
