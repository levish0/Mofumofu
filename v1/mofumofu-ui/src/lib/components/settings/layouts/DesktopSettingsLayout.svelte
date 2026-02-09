<script lang="ts">
	import { CheckCircle, ArrowUturnLeft, Icon } from 'svelte-hero-icons';
	import { authStore } from '$lib/stores/auth.svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { PersonalInfoSettings } from '../forms/PersonalInfoSettings';
	import AccountSettings from '../forms/AccountSettings/AccountSettings.svelte';
	import DisplaySettings from '../forms/DisplaySettings/DisplaySettings.svelte';
	import WritingSettings from '../forms/WritingSettings.svelte';
	import PrivacySettings from '../forms/PrivacySettings.svelte';
	import NotificationSettings from '../forms/NotificationSettings.svelte';
	import * as m from '../../../../paraglide/messages';

	type Props = {
		sections: Array<{
			id: string;
			label: () => string;
			icon: any;
			description: () => string;
			requiresAuth: boolean;
		}>;
		selectedSection: string;
		topPosition: string;
		handleSave: () => Promise<void>;
		saveSuccess: boolean;
		onSectionChange: (sectionId: string) => void;
		openImageCrop: (
			imageSrc: string,
			aspectRatio?: number,
			shape?: 'rect' | 'round',
			onComplete?: (data: any) => void
		) => void;
		handleReset: () => void;
	};

	const {
		sections,
		selectedSection,
		topPosition,
		handleSave,
		saveSuccess,
		onSectionChange,
		openImageCrop,
		handleReset
	}: Props = $props();
</script>

<!-- 데스크톱 레이아웃 -->
<div class="dark:text-mofu-dark-200 text-mofu-light-200 hidden min-h-screen w-full gap-4 lg:flex">
	<!-- Sidebar with Card Grid -->
	<div class="w-1/3">
		<div class="sticky space-y-4 transition-all duration-100 ease-out" style="top: {topPosition}">
			<div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
				{#each sections as section}
					<button
						class="dark:bg-mofu-dark-800 dark:border-mofu-dark-800 group border-mofu-light-800 bg-mofu-light-800 flex cursor-pointer flex-col overflow-hidden rounded-xl border p-4 text-left transition-all duration-200 hover:opacity-75 hover:shadow-xl {selectedSection ===
						section.id
							? 'opacity-70'
							: ''} {section.requiresAuth && !authStore.isAuthenticated ? 'cursor-not-allowed opacity-30' : ''}"
						onclick={() => {
							if (!section.requiresAuth || authStore.isAuthenticated) {
								onSectionChange(section.id);
								window.location.hash = section.id;
							}
						}}
					>
						<div class="mb-3 flex items-center gap-3">
							<div class="rounded-lg bg-white/10 p-2">
								<Icon src={section.icon} size="20" solid class="text-mofu" />
							</div>
							<div class="flex-1">
								<h3 class="dark:text-mofu-dark-200 text-mofu-light-200 text-md font-bold break-all">
									{section.label()}
								</h3>
							</div>
						</div>
						<p class="dark:text-mofu-dark-300 text-mofu-light-300 text-xs">{section.description()}</p>
					</button>
				{/each}
			</div>

			<!-- Save Button -->
			<button
				class="dark:bg-mofu-dark-800 dark:border-mofu-dark-800 group border-mofu-light-800 bg-mofu-light-800 flex w-full cursor-pointer flex-col overflow-hidden rounded-xl border p-4 text-left transition-all duration-200 hover:opacity-75 hover:shadow-xl {!settingsStore.hasChanges
					? 'cursor-not-allowed opacity-50'
					: ''}"
				onclick={handleSave}
				disabled={!settingsStore.hasChanges || settingsStore.isLoading}
			>
				<div class="flex items-center justify-center gap-2">
					{#if settingsStore.isLoading}
						<div class="border-mofu h-4 w-4 animate-spin rounded-full border-2 border-t-transparent"></div>
						<h3 class="dark:text-mofu-dark-200 text-mofu-light-200 text-md font-bold">{m.settings_saving()}</h3>
					{:else if saveSuccess}
						<Icon src={CheckCircle} class="h-4 w-4 text-green-400" />
						<h3 class="dark:text-mofu-dark-200 text-mofu-light-200 text-md font-bold">{m.settings_saved()}</h3>
					{:else}
						<h3 class="dark:text-mofu-dark-200 text-mofu-light-200 text-md font-bold">{m.settings_save_changes()}</h3>
						{#if settingsStore.hasChanges}
							<span class="text-xs text-orange-400">•</span>
						{/if}
					{/if}
				</div>
			</button>

			<!-- Reset Button (only show if there are changes) -->
			{#if settingsStore.hasChanges}
				<button
					class="dark:bg-mofu-dark-800/50 dark:border-mofu-dark-800 group flex w-full cursor-pointer flex-col overflow-hidden rounded-xl border border-white bg-white p-2 text-center transition-all duration-200 hover:opacity-75"
					onclick={handleReset}
				>
					<div class="flex items-center justify-center gap-2">
						<Icon src={ArrowUturnLeft} class="text-mofu-light-600 dark:text-mofu-dark-300 h-4 w-4" />
						<span class="dark:text-mofu-dark-300 text-mofu-light-600 text-sm">{m.settings_reset_changes()}</span>
					</div>
				</button>
			{/if}

			<!-- Error Messages -->
			{#if settingsStore.errors.general}
				<div class="rounded-lg border border-red-500/20 bg-red-500/10 p-3">
					<p class="text-xs text-red-400">{settingsStore.errors.general}</p>
				</div>
			{/if}

			<!-- Validation Errors -->
			{#if settingsStore.hasValidationErrors()}
				<div class="rounded-lg border border-orange-500/20 bg-orange-500/10 p-3">
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
	</div>

	<!-- Main Content -->
	<div class="flex-1">
		{#if selectedSection === 'personal'}
			<PersonalInfoSettings {openImageCrop} />
		{:else if selectedSection === 'account'}
			<AccountSettings />
		{:else if selectedSection === 'display'}
			<DisplaySettings />
		{:else if selectedSection === 'writing'}
			<WritingSettings />
		{:else if selectedSection === 'notifications'}
			<NotificationSettings />
		{:else if selectedSection === 'privacy'}
			<PrivacySettings />
		{/if}
	</div>
</div>
