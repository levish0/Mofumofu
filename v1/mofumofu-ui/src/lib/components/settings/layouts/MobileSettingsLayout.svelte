<script lang="ts">
	import { CheckCircle, ArrowUturnLeft, Icon } from 'svelte-hero-icons';
	import { authStore } from '$lib/stores/auth.svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { PersonalInfoSettings } from '../forms/PersonalInfoSettings';
	import AccountSettings from '../forms/AccountSettings/AccountSettings.svelte';
	import WritingSettings from '../forms/WritingSettings.svelte';
	import PrivacySettings from '../forms/PrivacySettings.svelte';
	import NotificationSettings from '../forms/NotificationSettings.svelte';
	import * as Accordion from '$lib/components/ui/accordion';
	import { Button } from '$lib/components/ui/button';
	import * as m from '../../../../paraglide/messages';
	import DisplaySettings from '../forms/DisplaySettings/DisplaySettings.svelte';

	type Props = {
		sections: Array<{
			id: string;
			label: () => string;
			icon: any;
			description: () => string;
			requiresAuth: boolean;
		}>;
		handleSave: () => Promise<void>;
		saveSuccess: boolean;
		openImageCrop: (
			imageSrc: string,
			aspectRatio?: number,
			shape?: 'rect' | 'round',
			onComplete?: (data: any) => void
		) => void;
		handleReset: () => void;
	};

	const { sections, handleSave, saveSuccess, openImageCrop, handleReset }: Props = $props();

	// 모바일에서 accordion의 기본 열린 섹션
	let accordionValue = $state(authStore.isAuthenticated ? 'personal' : 'display');
</script>

<!-- 모바일 레이아웃 (Accordion) -->
<div class="text-mofu-dark-200 min-h-screen w-full pb-10 lg:hidden">
	<Accordion.Root type="single" bind:value={accordionValue} class="space-y-2">
		{#each sections.filter((s) => !s.requiresAuth || authStore.isAuthenticated) as section}
			<Accordion.Item value={section.id}>
				<Accordion.Trigger class="flex w-full items-center justify-between text-left">
					<div class="flex items-center gap-3">
						<div class="rounded-lg bg-white/10 p-2">
							<Icon src={section.icon} size="20" solid class="text-mofu" />
						</div>
						<div>
							<h3 class="text-mofu-dark-200 text-md font-bold">{section.label()}</h3>
							<p class="dark:text-mofu-dark-300 text-xs text-gray-400">{section.description()}</p>
						</div>
					</div>
				</Accordion.Trigger>
				<Accordion.Content class="pb-4">
					{#if section.id === 'personal'}
						<PersonalInfoSettings {openImageCrop} />
					{:else if section.id === 'account'}
						<AccountSettings />
					{:else if section.id === 'display'}
						<DisplaySettings />
					{:else if section.id === 'writing'}
						<WritingSettings />
					{:else if section.id === 'notifications'}
						<NotificationSettings />
					{:else if section.id === 'privacy'}
						<PrivacySettings />
					{/if}
				</Accordion.Content>
			</Accordion.Item>
		{/each}
	</Accordion.Root>

	<!-- Error Messages -->
	{#if settingsStore.errors.general}
		<div class="mt-4 rounded-lg border border-red-500/20 bg-red-500/10 p-3">
			<p class="text-xs text-red-400">{settingsStore.errors.general}</p>
		</div>
	{/if}

	<!-- Validation Errors -->
	{#if settingsStore.hasValidationErrors()}
		<div class="mt-4 rounded-lg border border-orange-500/20 bg-orange-500/10 p-3">
			<p class="text-xs font-medium text-orange-400">{m.settings_validation_errors()}</p>
			{#each Object.entries(settingsStore.validationErrors) as [section, sectionErrors]}
				{#if Object.keys(sectionErrors).length > 0}
					<div class="mt-2">
						{#each Object.entries(sectionErrors) as [field, error]}
							<p class="text-xs text-orange-400">• {field}: {error}</p>
						{/each}
					</div>
				{/if}
			{/each}
		</div>
	{/if}
</div>

<!-- 모바일용 하단 고정 Save Changes 버튼 -->
<div class="text-mofu-dark-200 dark:text-mofu-dark-200 fixed right-4 bottom-4 left-4 z-50 space-y-2 lg:hidden">
	<!-- Save Button -->
	<Button
		class="group bg-mofu-dark-800 dark:bg-mofu-dark-800 flex w-full cursor-pointer flex-col overflow-hidden rounded-xl border px-6 py-6 text-left transition-all duration-200 hover:opacity-75 hover:shadow-xl {!settingsStore.hasChanges
			? 'cursor-not-allowed '
			: ''}"
		onclick={handleSave}
		disabled={!settingsStore.hasChanges || settingsStore.isLoading}
	>
		<div class="flex items-center justify-center gap-2">
			{#if settingsStore.isLoading}
				<div class="border-mofu h-4 w-4 animate-spin rounded-full border-2 border-t-transparent"></div>
				<h3 class="text-md text-mofu-dark-200 font-bold">{m.settings_saving()}</h3>
			{:else if saveSuccess}
				<Icon src={CheckCircle} class="h-4 w-4 text-green-400" />
				<h3 class="text-md text-mofu-dark-200 font-bold">{m.settings_saved()}</h3>
			{:else}
				<h3 class="text-md text-mofu-dark-200 font-bold">{m.settings_save_changes()}</h3>
				{#if settingsStore.hasChanges}
					<span class="text-xs text-orange-400">•</span>
				{/if}
			{/if}
		</div>
	</Button>

	<!-- Reset Button (only show if there are changes) -->
	{#if settingsStore.hasChanges}
		<Button
			class="group bg-mofu-dark-800 dark:bg-mofu-dark-800 flex w-full cursor-pointer flex-col overflow-hidden rounded-xl border px-6 py-6 text-center transition-all duration-200 hover:opacity-75"
			onclick={handleReset}
		>
			<div class="flex items-center justify-center gap-2">
				<Icon src={ArrowUturnLeft} class="text-mofu-dark-200 dark:text-mofu-dark-200 h-4 w-4" />
				<span class="text-mofu-dark-200 dark:text-mofu-dark-200 text-sm">{m.settings_reset_changes()}</span>
			</div>
		</Button>
	{/if}
</div>
