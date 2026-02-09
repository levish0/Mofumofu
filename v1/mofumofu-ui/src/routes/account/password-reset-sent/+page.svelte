<!-- src/routes/account/password-reset-sent/+page.svelte -->
<script lang="ts">
	import { EnvelopeOpen, Home, Icon } from 'svelte-hero-icons';
	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';
</script>

<div class="text-mofu-dark-200 bg-mofu-dark-900 flex min-h-screen items-center justify-center">
	<div class="w-full max-w-md space-y-8 p-8">
		<div class="text-center">
			<div class="space-y-4">
				<div class="text-mofu">
					<Icon src={EnvelopeOpen} solid size="40" class="inline-block" />
				</div>
				<h2 class="text-xl font-semibold">이메일을 확인해주세요</h2>
				<p class="text-mofu-dark-300">
					비밀번호 재설정 링크를 이메일로 보내드렸습니다. 이메일을 확인하여 비밀번호를 재설정해주세요.
				</p>
				<div class="pt-4">
					<Button
						onclick={() => goto('/')}
						variant="ghost"
						class="text-mofu-dark-300 rounded-md text-sm hover:opacity-70"
					>
						<Icon src={Home} size="16" class="mr-2" />
						홈으로 가기
					</Button>
				</div>
			</div>
		</div>
	</div>
</div>
