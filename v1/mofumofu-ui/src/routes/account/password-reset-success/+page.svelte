<!-- src/routes/account/password-reset-success/+page.svelte -->
<script lang="ts">
	import { CheckCircle, Icon } from 'svelte-hero-icons';
	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';
</script>

<div class="text-mofu-dark-200 bg-mofu-dark-900 flex min-h-screen items-center justify-center">
	<div class="w-full max-w-md space-y-8 p-8">
		<div class="text-center">
			<div class="space-y-4">
				<div class="text-green-600">
					<Icon src={CheckCircle} solid size="40" class="inline-block" />
				</div>
				<h2 class="text-xl font-semibold">비밀번호 재설정 완료</h2>
				<p class="text-mofu-dark-300">비밀번호가 성공적으로 변경되었습니다. 새로운 비밀번호로 로그인해주세요.</p>
				<div class="pt-4">
					<Button
						onclick={() => goto('/account/signin')}
						class="bg-mofu text-mofu-dark-900 rounded-md px-4 py-2 text-sm font-semibold hover:opacity-70"
					>
						로그인하기
					</Button>
				</div>
			</div>
		</div>
	</div>
</div>
