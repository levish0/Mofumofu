<!-- src/routes/account/verify-email/+page.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { verifyEmail } from '$lib/api/auth/authApi';
	import { goto } from '$app/navigation';
	import { ApiError } from '$lib/api/error/common_error';
	import { ExclamationTriangle, CheckCircle, Icon } from 'svelte-hero-icons';
	import * as m from '../../../paraglide/messages';
	import { Button } from '$lib/components/ui/button';

	let loading = $state(true);
	let success = $state(false);
	let error = $state<string | null>(null);

	onMount(async () => {
		try {
			const token = page.url.searchParams.get('token');

			if (!token) {
				throw new Error(m.auth_token_missing());
			}

			await verifyEmail(token);
			success = true;
		} catch (err) {
			console.error('Email verification error:', err);

			if (err instanceof ApiError) {
				error = m.verify_email_failed_message({ message: err.message });
			} else if (err instanceof Error) {
				error = err.message;
			} else {
				error = m.verify_email_unexpected_error();
			}
		} finally {
			loading = false;
		}
	});
</script>

<div class="text-mofu-dark-200 bg-mofu-dark-900 flex min-h-screen items-center justify-center">
	<div class="w-full max-w-md space-y-8 p-8">
		<div class="text-center">
			{#if loading}
				<div class="space-y-4">
					<div class="border-mofu-dark-100 mx-auto h-12 w-12 animate-spin rounded-full border-b-2"></div>
					<h2 class="text-xl font-semibold">{m.verify_email_in_progress()}</h2>
					<p>{m.verify_email_please_wait()}</p>
				</div>
			{:else if success}
				<div class="space-y-4">
					<div class="text-green-600">
						<Icon src={CheckCircle} solid size="40" class="inline-block" />
					</div>
					<h2 class="text-xl font-semibold">{m.verify_email_completed()}</h2>
					<p class="text-mofu-dark-300">{m.verify_email_success_description()}</p>
					<div class="pt-4">
						<Button
							onclick={() => goto('/account/signin')}
							class="bg-mofu text-mofu-dark-900 rounded-md px-4 py-2 text-sm font-semibold hover:opacity-70"
						>
							{m.verify_email_sign_in()}
						</Button>
					</div>
				</div>
			{:else if error}
				<div class="space-y-4">
					<div class="text-rose-600">
						<Icon src={ExclamationTriangle} solid size="40" class="inline-block" />
					</div>
					<h2 class="text-xl font-semibold">{m.verify_email_failed()}</h2>
					<p class="text-mofu-dark-300">{error}</p>
					<div class="flex gap-3 pt-4">
						<Button
							onclick={() => goto('/account/signup')}
							variant="ghost"
							class="text-mofu-dark-300 rounded-md text-sm hover:opacity-70"
						>
							{m.verify_email_signup_again()}
						</Button>
						<Button
							onclick={() => goto('/account/signin')}
							variant="ghost"
							class="text-mofu-dark-300 rounded-md text-sm hover:opacity-70"
						>
							{m.verify_email_sign_in()}
						</Button>
					</div>
				</div>
			{/if}
		</div>
	</div>
</div>
