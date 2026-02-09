<script lang="ts">
	import { Textarea } from '../../../ui/textarea';
	import * as v from 'valibot';
	import { createPersonalInfoSchema } from '$lib/schemas/personal-info';
	import * as m from '../../../../../paraglide/messages';

	interface Props {
		bio: string | null;
		onUpdate: (bio: string) => void;
		onValidationChange: (error?: string) => void;
		isVerified: boolean;
	}

	let { bio, onUpdate, onValidationChange, isVerified }: Props = $props();

	let localError = $state<string | undefined>();

	function validateBio(value: string): string | undefined {
		const schema = createPersonalInfoSchema();
		const result = v.safeParse(schema.entries.bio, value.trim());
		return result.success ? undefined : result.issues?.[0]?.message;
	}

	function handleInput(e: Event) {
		if (!isVerified) {
			alert('이메일 인증이 필요합니다. 이메일을 확인해주세요.');
			return;
		}

		const value = (e.target as HTMLTextAreaElement).value;
		onUpdate(value);

		const error = validateBio(value);
		localError = error;
		onValidationChange(error);
	}

	const characterCount = $derived((bio || '').length);
	const isOverLimit = $derived(characterCount > 200);
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_bio()}</h2>
	<div class="space-y-2">
		<div class="relative">
			<Textarea
				id="bio"
				placeholder={m.settings_bio_placeholder()}
				class="dark:bg-mofu-dark-800 text-mofu-dark-200 placeholder:text-mofu-dark-300 min-h-[100px]  {localError
					? 'border-red-500'
					: ''} {!isVerified ? 'cursor-not-allowed opacity-50' : ''}"
				value={bio || ''}
				oninput={handleInput}
				disabled={!isVerified}
			/>
			<div class="absolute right-2 bottom-2 text-xs {isOverLimit ? 'text-red-400' : 'text-mofu-dark-400'}">
				{characterCount}/200
			</div>
		</div>
		{#if !isVerified}
			<p class="text-xs text-gray-500">이메일 인증이 필요합니다. 자기소개 변경을 위해 이메일을 인증해주세요.</p>
		{:else if localError}
			<p class="text-xs text-rose-400">{localError}</p>
		{:else}
			<p class="text-mofu-dark-400 text-xs">{m.settings_bio_description()}</p>
		{/if}
	</div>
</div>
