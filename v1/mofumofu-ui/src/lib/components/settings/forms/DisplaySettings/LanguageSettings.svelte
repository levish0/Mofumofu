<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import * as m from '../../../../../paraglide/messages';
	import { setLocale, getLocale } from '../../../../../paraglide/runtime';
	import { onMount } from 'svelte';

	let selectedLanguage = $state('en');

	const languages = [
		{ value: 'en', label: m.language_english, flag: '🇺🇸' },
		{ value: 'ja', label: m.language_japanese, flag: '🇯🇵' },
		{ value: 'ko', label: m.language_korean, flag: '🇰🇷' },
		{ value: 'de', label: m.language_german, flag: '🇩🇪' },
		{ value: 'es', label: m.language_spanish, flag: '🇪🇸' },
		{ value: 'es-MX', label: m.language_spanish_mx, flag: '🇲🇽' },
		{ value: 'fr', label: m.language_french, flag: '🇫🇷' },
		{ value: 'ru', label: m.language_russian, flag: '🇷🇺' }
	];

	const selectedLang = $derived(languages.find((l) => l.value === selectedLanguage));
	const triggerContent = $derived(selectedLang ? `${selectedLang.flag} ${selectedLang.label()}` : 'Select language');

	onMount(() => {
		selectedLanguage = getLocale();
	});

	function handleLanguageChange(value: string | undefined) {
		if (value && isValidLocale(value)) {
			selectedLanguage = value;
			setLocale(value as 'en' | 'ko' | 'ja' | 'de' | 'es' | 'es-MX' | 'fr' | 'ru');
		}
	}

	function isValidLocale(value: string): value is 'en' | 'ko' | 'ja' | 'de' | 'es' | 'es-MX' | 'fr' | 'ru' {
		return ['en', 'ko', 'ja', 'de', 'es', 'es-MX', 'fr', 'ru'].includes(value);
	}
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_language()}</h2>
	<div class="space-y-2">
		<Select.Root type="single" bind:value={selectedLanguage} onValueChange={handleLanguageChange}>
			<Select.Trigger
				class="dark:bg-mofu-dark-800 bg-mofu-light-800 text-mofu-light-200 dark:text-mofu-dark-200 border-mofu-light-700 dark:border-mofu-dark-700 w-64"
			>
				{triggerContent}
			</Select.Trigger>
			<Select.Content class="dark:bg-mofu-dark-800 bg-mofu-light-800 border-mofu-light-700 dark:border-mofu-dark-700">
				{#each languages as language}
					<Select.Item
						value={language.value}
						class="dark:text-mofu-dark-200 text-mofu-light-200 hover:bg-mofu-light-700 dark:hover:bg-mofu-dark-700"
					>
						{language.flag}
						{language.label()}
					</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
		<p class="dark:text-mofu-dark-400 text-mofu-light-400 text-xs">{m.settings_language_description()}</p>
	</div>
</div>
