<script lang="ts">
	import ThemeSettings from './ThemeSettings.svelte';
	import LanguageSettings from './LanguageSettings.svelte';
</script>

<div class="dark:text-mofu-dark-200 text-mofu-light-200 min-h-screen">
	<div class="space-y-8">
		<LanguageSettings />
		<ThemeSettings />
	</div>
</div>
