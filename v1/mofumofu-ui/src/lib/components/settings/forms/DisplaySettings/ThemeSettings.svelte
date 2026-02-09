<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import * as m from '../../../../../paraglide/messages';
	import { mode, setMode } from 'mode-watcher';

	let selectedTheme = $state(mode.current || 'system');

	const themes = [
		{ value: 'system', label: () => m.theme_system() },
		{ value: 'light', label: () => m.theme_light() },
		{ value: 'dark', label: () => m.theme_dark() }
	];

	$effect(() => {
		selectedTheme = mode.current || 'system';
	});

	function handleThemeChange(value: string | undefined) {
		if (value && isValidTheme(value)) {
			selectedTheme = value;
			setMode(value as 'system' | 'light' | 'dark');
		}
	}

	function isValidTheme(value: string): value is 'system' | 'light' | 'dark' {
		return ['system', 'light', 'dark'].includes(value);
	}
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_theme()}</h2>
	<div class="space-y-2">
		<Select.Root type="single" bind:value={selectedTheme} onValueChange={handleThemeChange}>
			<Select.Trigger
				class="dark:bg-mofu-dark-800 bg-mofu-800  text-mofu-light-200 dark:text-mofu-dark-200 border-mofu-700 dark:border-mofu-dark-700 w-64"
			>
				{themes.find((t) => t.value === selectedTheme)?.label() || m.select_theme()}
			</Select.Trigger>
			<Select.Content class="dark:bg-mofu-dark-800 bg-mofu-light-800 border-mofu-700  dark:border-mofu-dark-700">
				{#each themes as theme}
					<Select.Item
						value={theme.value}
						class="text-mofu-light-200 dark:text-mofu-dark-200 hover:bg-mofu-light-700 dark:hover:bg-mofu-dark-700"
					>
						{theme.label()}
					</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</div>
</div>
