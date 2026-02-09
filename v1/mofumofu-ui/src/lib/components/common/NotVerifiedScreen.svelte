<script lang="ts">
	import { ShieldExclamation, Icon } from 'svelte-hero-icons';
	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';
	import * as m from '../../../paraglide/messages';

	let {
		isVisible,
		title = m.email_verification_required_title(),
		description = m.email_verification_required_description(),
		showHomeButton = true,
		showSettingsButton = true
	} = $props<{
		isVisible: boolean;
		title?: string;
		description?: string;
		showHomeButton?: boolean;
		showSettingsButton?: boolean;
	}>();
</script>

{#if isVisible}
	<div class="text-mofu-dark-200 bg-mofu-dark-900 flex min-h-screen items-center justify-center">
		<div class="w-full max-w-md space-y-8 p-8">
			<div class="text-center">
				<div class="space-y-4">
					<div class="text-amber-600">
						<Icon src={ShieldExclamation} solid size="40" class="inline-block" />
					</div>
					<h2 class="text-xl font-semibold">{title}</h2>
					<p class="text-sm text-gray-400">{description}</p>
					<div class="flex flex-col space-y-3 pt-4">
						{#if showSettingsButton}
							<Button onclick={() => goto('/settings#account')} class="w-full"
								>{m.email_verification_go_to_settings()}</Button
							>
						{/if}
						{#if showHomeButton}
							<Button
								onclick={() => goto('/')}
								variant="ghost"
								class="dark:text-mofu-dark-300 w-full rounded-md text-sm hover:opacity-70"
							>
								{m.auth_error_go_home()}
							</Button>
						{/if}
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
