<!-- src/routes/account/email-sent/+page.svelte -->
<script lang="ts">
	import { Envelope, Home, Icon } from 'svelte-hero-icons';
	import * as m from '../../../paraglide/messages';
	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';
</script>

<div class="text-mofu-dark-200 bg-mofu-dark-900 flex min-h-screen items-center justify-center">
	<div class="w-full max-w-md space-y-8 p-8">
		<div class="text-center">
			<div class="space-y-4">
				<div class="text-mofu">
					<Icon src={Envelope} solid size="40" class="inline-block" />
				</div>
				<h2 class="text-xl font-semibold">이메일이 발송되었습니다</h2>
				<p class="text-mofu-dark-300">인증 링크를 확인하여 회원가입을 완료해주세요</p>
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
