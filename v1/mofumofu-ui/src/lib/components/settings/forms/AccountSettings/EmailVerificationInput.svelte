<script lang="ts">
	import { Input } from '../../../ui/input';
	import { userStore } from '$lib/stores/user.svelte';
	import { resendVerification } from '$lib/api/auth/authApi';
	import { toast } from 'svelte-sonner';
	import * as m from '../../../../../paraglide/messages';

	interface Props {
		email: string;
	}

	let { email }: Props = $props();

	let isResending = $state(false);
	let cooldownRemaining = $state(0);
	let cooldownInterval: ReturnType<typeof setInterval> | undefined = $state();

	const isVerified = $derived(userStore.user?.is_verified ?? false);

	async function handleResendVerification() {
		if (isResending || cooldownRemaining > 0) return;

		try {
			isResending = true;
			await resendVerification(email);

			// 성공 시 1분 쿨다운 시작
			cooldownRemaining = 60;
			cooldownInterval = setInterval(() => {
				cooldownRemaining--;
				if (cooldownRemaining <= 0) {
					clearInterval(cooldownInterval);
					cooldownInterval = undefined;
				}
			}, 1000);

			toast.success('인증 이메일이 다시 전송되었습니다. 이메일함을 확인해주세요.');
		} catch (error) {
			console.error('Failed to resend verification email:', error);
			toast.error('인증 이메일 전송에 실패했습니다. 잠시 후 다시 시도해주세요.');
		} finally {
			isResending = false;
		}
	}

	// 컴포넌트 정리 시 interval 정리
	$effect(() => {
		return () => {
			if (cooldownInterval) {
				clearInterval(cooldownInterval);
			}
		};
	});
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">이메일 인증</h2>
	<div class="space-y-2">
		<div class="relative flex">
			<div class="relative flex-1">
				<Input
					id="email"
					placeholder="이메일 주소"
					class="dark:bg-mofu-dark-800 bg-mofu-light-800 text-mofu-light-200  dark:text-mofu-dark-200 placeholder:text-mofu-light-300 dark:placeholder:text-mofu-dark-300 rounded-r-none border-r-0"
					value={email}
					disabled={true}
				/>
			</div>
			<button
				onclick={handleResendVerification}
				disabled={isResending || cooldownRemaining > 0 || isVerified}
				class="dark:bg-mofu-dark-800/50 text-mofu-dark-200 disabled:dark:bg-mofu-dark-800/50 hover:bg-mofu-dark-700 inline-flex min-w-20 items-center justify-center rounded-r-md px-3 py-2 text-sm font-medium disabled:cursor-not-allowed disabled:opacity-50"
			>
				{#if isResending}
					<div class="border-mofu-dark-200 h-4 w-4 animate-spin rounded-full border-2 border-t-transparent"></div>
				{:else if cooldownRemaining > 0}
					재전송 ({cooldownRemaining}s)
				{:else}
					인증
				{/if}
			</button>
		</div>
		{#if isVerified}
			<p class="text-xs text-green-400">✓ 이메일 인증이 완료되었습니다</p>
		{:else}
			<p class="text-xs text-orange-400">이메일 인증이 필요합니다. 인증 버튼을 클릭하여 인증 이메일을 받아보세요.</p>
		{/if}
	</div>
</div>
