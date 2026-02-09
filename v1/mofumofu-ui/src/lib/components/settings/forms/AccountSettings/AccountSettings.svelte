<script lang="ts">
	import { userStore } from '$lib/stores/user.svelte';
	import EmailVerificationInput from './EmailVerificationInput.svelte';
	import PasswordManagement from './PasswordManagement.svelte';
	import OAuthConnections from './OAuthConnections.svelte';

	const user = $derived(userStore.user);
</script>

<div class="dark:text-mofu-dark-200 text-mofu-light-200 min-h-screen">
	<div class="space-y-8">
		{#if user?.email}
			<EmailVerificationInput email={user.email} />
		{/if}

		<PasswordManagement />

		<OAuthConnections />
	</div>
</div>
