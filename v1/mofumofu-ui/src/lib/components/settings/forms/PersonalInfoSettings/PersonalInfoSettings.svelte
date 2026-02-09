<script lang="ts">
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import { useFieldValidation } from '$lib/hooks/settings/useFieldValidation.svelte';
	import { onMount } from 'svelte';
	import BannerImageUpload from './BannerImageUpload.svelte';
	import ProfileImageUpload from './ProfileImageUpload.svelte';
	import HandleInput from './HandleInput.svelte';
	import DisplayNameInput from './DisplayNameInput.svelte';
	import BioInput from './BioInput.svelte';
	import LocationInput from './LocationInput.svelte';
	import WebsiteInput from './WebsiteInput.svelte';

	type Props = {
		openImageCrop: (
			imageSrc: string,
			aspectRatio?: number,
			shape?: 'rect' | 'round',
			onComplete?: (data: any) => void
		) => void;
	};

	const { openImageCrop }: Props = $props();

	const personal = $derived(settingsStore.personal);
	const { handle, name, profileImage, bannerImage, bio, location, website } = $derived(personal);
	const isVerified = $derived(userStore.user?.is_verified ?? false);

	// 필드 검증 훅 사용
	type PersonalErrors = {
		handle?: string;
		name?: string;
		bio?: string;
		location?: string;
		website?: string;
	};

	function validateForm(): boolean {
		const hasErrors = Object.keys(localErrors).length > 0;

		if (hasErrors) {
			settingsStore.setValidationErrors('personal', localErrors);
		} else {
			settingsStore.clearValidationErrors('personal');
		}

		return !hasErrors;
	}

	const { localErrors, createValidationHandler } = useFieldValidation<PersonalErrors>({
		validateForm
	});

	function handleBannerUpdate(data: { bannerImageFile: Blob; bannerImage: string }) {
		settingsStore.updatePersonal(data);
	}

	function handleProfileUpdate(data: { profileImageFile: Blob; profileImage: string }) {
		settingsStore.updatePersonal(data);
	}

	function handleHandleUpdate(newHandle: string) {
		settingsStore.updatePersonal({ handle: newHandle });
		validateForm();
	}

	function handleNameUpdate(newName: string) {
		settingsStore.updatePersonal({ name: newName });
	}

	function handleBioUpdate(newBio: string) {
		settingsStore.updatePersonal({ bio: newBio });
	}

	function handleLocationUpdate(newLocation: string) {
		settingsStore.updatePersonal({ location: newLocation });
	}

	function handleWebsiteUpdate(newWebsite: string) {
		settingsStore.updatePersonal({ website: newWebsite });
	}

	// 제네릭 validation 핸들러들 생성
	const handleHandleValidation = createValidationHandler('handle');
	const handleNameValidation = createValidationHandler('name');
	const handleBioValidation = createValidationHandler('bio');
	const handleLocationValidation = createValidationHandler('location');
	const handleWebsiteValidation = createValidationHandler('website');

	onMount(() => {
		if (handle && name) {
			validateForm();
		}
	});
</script>

<div class="dark:text-mofu-dark-200 text-mofu-light-200 min-h-screen">
	<div class="space-y-4">
		<BannerImageUpload {bannerImage} onUpdate={handleBannerUpdate} {openImageCrop} {isVerified} />

		<ProfileImageUpload {profileImage} onUpdate={handleProfileUpdate} {openImageCrop} {isVerified} />

		<HandleInput {handle} onUpdate={handleHandleUpdate} onValidationChange={handleHandleValidation} {isVerified} />

		<DisplayNameInput {name} onUpdate={handleNameUpdate} onValidationChange={handleNameValidation} {isVerified} />

		<BioInput {bio} onUpdate={handleBioUpdate} onValidationChange={handleBioValidation} {isVerified} />

		<LocationInput
			{location}
			onUpdate={handleLocationUpdate}
			onValidationChange={handleLocationValidation}
			{isVerified}
		/>

		<WebsiteInput {website} onUpdate={handleWebsiteUpdate} onValidationChange={handleWebsiteValidation} {isVerified} />
	</div>
</div>
