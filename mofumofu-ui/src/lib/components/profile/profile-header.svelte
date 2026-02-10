<script lang="ts">
	import { getContext } from 'svelte';
	import { invalidateAll } from '$app/navigation';
	import { toast } from 'svelte-sonner';
	import { Icon, Photo } from 'svelte-hero-icons';
	import PillButton from '$lib/components/pill-button/pill-button.svelte';
	import * as FileDropZone from '$lib/components/ui/file-drop-zone';
	import * as ImageCropper from '$lib/components/ui/image-cropper';
	import { uploadBannerImage, uploadProfileImage } from '$lib/api/user';
	import { isApiError } from '$lib/api';
	import FollowButton from './follow-button.svelte';
	import type { PublicUserProfile } from '$lib/api/types';

	interface Props {
		profile: PublicUserProfile;
		isOwnProfile: boolean;
		isAuthenticated: boolean;
	}

	const { profile, isOwnProfile, isAuthenticated }: Props = $props();

	const openSettings = getContext<() => void>('openSettings');

	// --- Banner upload ---
	async function handleBannerUpload(files: File[]) {
		const file = files[0];
		if (!file) return;

		try {
			await uploadBannerImage(file);
			toast.success('Banner updated.');
			await invalidateAll();
		} catch {
			toast.error('Failed to upload banner.');
		}
	}

	// --- Avatar crop + upload ---
	let cropperSrc = $state(profile.profile_image ?? '');
	let avatarUploading = $state(false);

	$effect(() => {
		cropperSrc = profile.profile_image ?? '';
	});

	async function handleAvatarCropped(url: string) {
		avatarUploading = true;
		try {
			const file = await ImageCropper.getFileFromUrl(url, 'profile.png');
			if (file.size > 4 * 1024 * 1024) {
				toast.error('Image must be under 4MB.');
				return;
			}
			await uploadProfileImage(file);
			await invalidateAll();
			toast.success('Profile image updated.');
		} catch (err) {
			if (isApiError(err)) toast.error(err.details ?? 'Upload failed.');
			else toast.error('Upload failed.');
		} finally {
			avatarUploading = false;
		}
	}
</script>

<div>
	<FileDropZone.Root
		onUpload={handleBannerUpload}
		disabled={!isOwnProfile}
		accept="image/*"
		maxFileSize={8 * FileDropZone.MEGABYTE}
		maxFiles={1}
		onFileRejected={({ reason }) => toast.error(reason)}
	>
		<div class="relative aspect-[3/1] w-full">
			<!-- Banner trigger -->
			<FileDropZone.Trigger class="absolute inset-0 z-0 block overflow-hidden rounded-xl">
				{#snippet children()}
					{#if profile.banner_image}
						<img
							src={profile.banner_image}
							alt="Banner"
							class="h-full w-full object-cover"
							loading="lazy"
						/>
					{:else}
						<div
							class="h-full w-full bg-gradient-to-r from-blue-400 to-purple-500"
						></div>
					{/if}

					{#if isOwnProfile}
						<div
							class="absolute inset-0 flex cursor-pointer flex-col items-center justify-center bg-black/50 opacity-0 transition-opacity hover:opacity-100"
						>
							<Icon src={Photo} class="h-8 w-8 text-white" />
							<span class="mt-2 text-sm font-medium text-white">Change banner</span>
						</div>
					{/if}
				{/snippet}
			</FileDropZone.Trigger>

			<!-- Action button -->
			<div class="absolute right-4 -bottom-12 z-20">
				{#if isOwnProfile}
					<PillButton variant="outline" onclick={openSettings} class="px-3">
						Edit Profile
					</PillButton>
				{:else}
					<FollowButton followeeId={profile.id} {isAuthenticated} />
				{/if}
			</div>

			<!-- Profile image with crop -->
			<div class="absolute -bottom-12 left-4 z-10">
				{#if isOwnProfile}
					<ImageCropper.Root
						bind:src={cropperSrc}
						onCropped={handleAvatarCropped}
						accept="image/png, image/jpeg, image/webp"
						onUnsupportedFile={() => toast.error('Unsupported file type.')}
					>
						<ImageCropper.UploadTrigger>
							<div class="group relative h-24 w-24 cursor-pointer">
								{#if profile.profile_image}
									<img
										src={profile.profile_image}
										alt={profile.display_name}
										class="h-24 w-24 rounded-full border-4 border-mofu-light-900 bg-mofu-light-900 object-cover dark:border-mofu-dark-900 dark:bg-mofu-dark-900"
										loading="lazy"
									/>
								{:else}
									<div
										class="flex h-24 w-24 items-center justify-center rounded-full border-4 border-mofu-light-900 bg-mofu-light-700 dark:border-mofu-dark-900 dark:bg-mofu-dark-700"
									>
										<span class="text-2xl font-medium text-black dark:text-white">
											{profile.display_name.charAt(0).toUpperCase()}
										</span>
									</div>
								{/if}
								<!-- Hover overlay -->
								<div
									class="absolute inset-0 flex items-center justify-center rounded-full bg-black/50 opacity-0 transition-opacity group-hover:opacity-100"
								>
									<Icon src={Photo} class="h-6 w-6 text-white" />
								</div>
							</div>
						</ImageCropper.UploadTrigger>
						<ImageCropper.Dialog>
							<ImageCropper.Cropper />
							<ImageCropper.Controls>
								<ImageCropper.Cancel />
								<ImageCropper.Crop />
							</ImageCropper.Controls>
						</ImageCropper.Dialog>
					</ImageCropper.Root>
				{:else}
					<div class="relative h-24 w-24">
						{#if profile.profile_image}
							<img
								src={profile.profile_image}
								alt={profile.display_name}
								class="h-24 w-24 rounded-full border-4 border-mofu-light-900 bg-mofu-light-900 object-cover dark:border-mofu-dark-900 dark:bg-mofu-dark-900"
								loading="lazy"
							/>
						{:else}
							<div
								class="flex h-24 w-24 items-center justify-center rounded-full border-4 border-mofu-light-900 bg-mofu-light-700 dark:border-mofu-dark-900 dark:bg-mofu-dark-700"
							>
								<span class="text-2xl font-medium text-black dark:text-white">
									{profile.display_name.charAt(0).toUpperCase()}
								</span>
							</div>
						{/if}
					</div>
				{/if}
			</div>
		</div>
	</FileDropZone.Root>
</div>
