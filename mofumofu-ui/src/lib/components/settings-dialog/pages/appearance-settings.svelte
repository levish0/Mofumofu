<script lang="ts">
	import { Icon, Sun, Moon, ComputerDesktop } from 'svelte-hero-icons';
	import { cn } from '$lib/utils';
	import { userPrefersMode, setMode } from 'mode-watcher';
	import * as FieldSet from '$lib/components/ui/field-set';

	type ThemeMode = 'light' | 'dark' | 'system';

	const themes: { id: ThemeMode; label: string; icon: typeof Sun }[] = [
		{ id: 'light', label: 'Light', icon: Sun },
		{ id: 'dark', label: 'Dark', icon: Moon },
		{ id: 'system', label: 'System', icon: ComputerDesktop }
	];

	let currentMode: ThemeMode = $derived((userPrefersMode.current as ThemeMode) || 'system');
</script>

<div class="mx-auto max-w-2xl space-y-6">
	<!-- Theme -->
	<section class="space-y-3">
		<h4 class="text-base font-semibold">Theme</h4>
		<FieldSet.Root>
			<FieldSet.Content>
				<div class="space-y-4">
					<p class="text-sm text-muted-foreground">Choose how mofumofu looks to you.</p>
					<div class="grid grid-cols-3 gap-3">
						{#each themes as theme (theme.id)}
							<button
								type="button"
								onclick={() => setMode(theme.id)}
								class={cn(
									'flex flex-col items-center gap-2 rounded-lg border-2 p-4 transition-colors',
									currentMode === theme.id ? 'border-mofu bg-mofu/5' : ''
								)}
							>
								<Icon
									src={theme.icon}
									class={cn(
										'size-6',
										currentMode === theme.id ? 'text-mofu' : 'text-muted-foreground'
									)}
								/>
								<span
									class={cn(
										'text-sm font-medium',
										currentMode === theme.id ? 'text-mofu' : 'text-muted-foreground'
									)}
								>
									{theme.label}
								</span>
							</button>
						{/each}
					</div>
				</div>
			</FieldSet.Content>
			<FieldSet.Footer>
				<span class="text-sm text-muted-foreground">Theme preference is stored locally.</span>
			</FieldSet.Footer>
		</FieldSet.Root>
	</section>
</div>
