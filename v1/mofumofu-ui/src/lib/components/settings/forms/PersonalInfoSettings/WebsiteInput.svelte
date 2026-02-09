<script lang="ts">
	import { Input } from '../../../ui/input';
	import * as v from 'valibot';
	import { createPersonalInfoSchema } from '$lib/schemas/personal-info';
	import * as m from '../../../../../paraglide/messages';

	interface Props {
		website: string | null;
		onUpdate: (website: string) => void;
		onValidationChange: (error?: string) => void;
		isVerified: boolean;
	}

	let { website, onUpdate, onValidationChange, isVerified }: Props = $props();

	let localError = $state<string | undefined>();

	function validateWebsite(value: string): string | undefined {
		const schema = createPersonalInfoSchema();
		const result = v.safeParse(schema.entries.website, value.trim());
		return result.success ? undefined : result.issues?.[0]?.message;
	}

	function handleInput(e: Event) {
		if (!isVerified) {
			alert('이메일 인증이 필요합니다. 이메일을 확인해주세요.');
			return;
		}

		const value = (e.target as HTMLInputElement).value;
		onUpdate(value);

		const error = validateWebsite(value);
		localError = error;
		onValidationChange(error);
	}

	const characterCount = $derived((website || '').length);
	const isOverLimit = $derived(characterCount > 50);
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_website()}</h2>
	<div class="space-y-2">
		<div class="relative">
			<Input
				id="website"
				placeholder={m.settings_website_placeholder()}
				class="dark:bg-mofu-dark-800 dark:text-mofu-dark-200 text-mofu-light-800 placeholder:text-mofu-dark-300 pr-12 {localError
					? 'border-red-500'
					: ''} {!isVerified ? 'cursor-not-allowed opacity-50' : ''}"
				value={website || ''}
				oninput={handleInput}
				disabled={!isVerified}
			/>
			<div
				class="absolute top-1/2 right-3 -translate-y-1/2 text-xs {isOverLimit ? 'text-red-400' : 'text-mofu-dark-400'}"
			>
				{characterCount}/50
			</div>
		</div>
		{#if !isVerified}
			<p class="text-xs text-gray-500">이메일 인증이 필요합니다. 웹사이트 변경을 위해 이메일을 인증해주세요.</p>
		{:else if localError}
			<p class="text-xs text-rose-400">{localError}</p>
		{:else}
			<p class="text-mofu-dark-400 text-xs">{m.settings_website_description()}</p>
		{/if}
	</div>
</div>
