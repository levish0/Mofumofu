<script lang="ts">
	import { Icon, Camera, LockClosed } from 'svelte-hero-icons';
	import { useImageCrop } from './useImageCrop';
	import * as m from '../../../../../paraglide/messages';

	interface Props {
		profileImage: string | null;
		onUpdate: (data: { profileImageFile: Blob; profileImage: string }) => void;
		openImageCrop: (
			imageSrc: string,
			aspectRatio?: number,
			shape?: 'rect' | 'round',
			onComplete?: (data: any) => void
		) => void;
		isVerified: boolean;
	}

	let { profileImage, onUpdate, openImageCrop, isVerified }: Props = $props();

	// No cache-busting needed for blob URLs since they're already unique

	let imageLoading = $state(true);

	const { cropImage, cleanupTempImage, handleFileRead } = useImageCrop();

	async function handleImageChange(event: Event) {
		if (!isVerified) {
			alert('이메일 인증이 필요합니다. 이메일을 확인해주세요.');
			return;
		}

		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file && file.type.startsWith('image/')) {
			// 파일 크기 체크 (4MB 제한)
			const fileSizeMB = file.size / (1024 * 1024);
			if (fileSizeMB > 4) {
				alert(`파일 크기가 ${fileSizeMB.toFixed(2)}MB로 4MB 제한을 초과합니다.`);
				target.value = '';
				return;
			}

			try {
				// GIF 파일은 crop 없이 바로 업로드
				if (file.type === 'image/gif') {
					const url = URL.createObjectURL(file);
					onUpdate({
						profileImageFile: file,
						profileImage: url
					});
					target.value = '';
					return;
				}

				// 다른 이미지 형식은 crop 모달 열기
				const tempImageSrc = await handleFileRead(file);

				openImageCrop(
					tempImageSrc,
					1,
					'round',
					async (data: { croppedAreaPixels: { x: number; y: number; width: number; height: number } }) => {
						try {
							const { blob, url } = await cropImage(tempImageSrc, data, {
								maxFileSizeMB: 4
							});

							onUpdate({
								profileImageFile: blob,
								profileImage: url
							});
							cleanupTempImage(tempImageSrc);
						} catch (error) {
							console.error('Error cropping profile image:', error);
							if (error instanceof Error) {
								alert(`Profile crop failed: ${error.message}`);
							}
							cleanupTempImage(tempImageSrc);
						}
					}
				);
			} catch (error) {
				console.error('Failed to read image file:', error);
				alert('Failed to read image file. Please try again.');
			}
		}
		target.value = '';
	}

	function handleImageLoad() {
		imageLoading = false;
	}

	// Reset loading state when image URL changes
	$effect(() => {
		if (profileImage) {
			imageLoading = true;
		}
	});
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_profile_image()}</h2>
	<div class="flex items-center space-x-4">
		<div class="group relative transition-all">
			<div
				class="bg-mofu-dark-800 relative h-24 w-24 overflow-hidden rounded-full {isVerified
					? 'group-hover:opacity-75'
					: 'opacity-50'}"
			>
				{#if profileImage}
					<!-- Skeleton shimmer while loading -->
					{#if imageLoading}
						<div class="shimmer absolute inset-0 rounded-full"></div>
					{/if}
					<img
						src={profileImage}
						alt="Profile preview"
						class="h-full w-full object-cover {imageLoading
							? 'opacity-0'
							: 'opacity-100'} transition-opacity duration-200"
						onload={handleImageLoad}
					/>
				{/if}

				{#if profileImage}
					<label
						for="profile-upload"
						class="dark:text-mofu-dark-300 absolute inset-0 flex {isVerified
							? 'cursor-pointer'
							: 'cursor-not-allowed'} items-center justify-center"
					>
						{#if !isVerified}
							<Icon src={LockClosed} class="h-6 w-6 text-gray-400" />
						{/if}
					</label>
				{:else}
					<label
						for="profile-upload"
						class="dark:text-mofu-dark-300 flex h-full {isVerified
							? 'cursor-pointer'
							: 'cursor-not-allowed'} items-center justify-center"
					>
						<Icon src={isVerified ? Camera : LockClosed} class="h-6 w-6 {isVerified ? '' : 'text-gray-400'}" />
					</label>
				{/if}
			</div>
			<input
				id="profile-upload"
				type="file"
				accept="image/*"
				class="hidden"
				onchange={handleImageChange}
				disabled={!isVerified}
			/>
		</div>
		<div class="text-mofu-dark-400 text-sm">
			{#if isVerified}
				<p>{m.settings_profile_image_recommended()}</p>
				<p>{m.settings_profile_image_max_size()}</p>
			{:else}
				<p class="text-gray-500">이메일 인증이 필요합니다</p>
				<p class="text-gray-500">프로필 이미지 업로드를 위해 이메일을 인증해주세요</p>
			{/if}
		</div>
	</div>
</div>
