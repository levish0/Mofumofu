<script lang="ts">
	import { Icon, Photo, LockClosed } from 'svelte-hero-icons';
	import { useImageCrop } from './useImageCrop';
	import * as m from '../../../../../paraglide/messages';

	interface Props {
		bannerImage: string | null;
		onUpdate: (data: { bannerImageFile: Blob; bannerImage: string }) => void;
		openImageCrop: (
			imageSrc: string,
			aspectRatio?: number,
			shape?: 'rect' | 'round',
			onComplete?: (data: any) => void
		) => void;
		isVerified: boolean;
	}

	let { bannerImage, onUpdate, openImageCrop, isVerified }: Props = $props();

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
			// 파일 크기 체크 (8MB 제한)
			const fileSizeMB = file.size / (1024 * 1024);
			if (fileSizeMB > 8) {
				alert(`파일 크기가 ${fileSizeMB.toFixed(2)}MB로 8MB 제한을 초과합니다.`);
				target.value = '';
				return;
			}

			try {
				// GIF 파일은 crop 없이 바로 업로드
				if (file.type === 'image/gif') {
					const url = URL.createObjectURL(file);
					onUpdate({
						bannerImageFile: file,
						bannerImage: url
					});
					target.value = '';
					return;
				}

				// 다른 이미지 형식은 crop 모달 열기
				const tempImageSrc = await handleFileRead(file);

				openImageCrop(
					tempImageSrc,
					4,
					'rect',
					async (data: { croppedAreaPixels: { x: number; y: number; width: number; height: number } }) => {
						try {
							const { blob, url } = await cropImage(tempImageSrc, data, {
								maxFileSizeMB: 8
							});

							onUpdate({
								bannerImageFile: blob,
								bannerImage: url
							});
							cleanupTempImage(tempImageSrc);
						} catch (error) {
							console.error('Error cropping banner image:', error);
							if (error instanceof Error) {
								alert(`Banner crop failed: ${error.message}`);
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
		if (bannerImage) {
			imageLoading = true;
		}
	});
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_banner_image()}</h2>
	<div class="group relative transition-all">
		<div
			class="bg-mofu-light-800 dark:bg-mofu-dark-800 relative aspect-[4/1] w-full overflow-hidden rounded-lg {isVerified
				? 'group-hover:opacity-75'
				: 'opacity-50'}"
		>
			{#if bannerImage}
				<!-- Skeleton shimmer while loading -->
				{#if imageLoading}
					<div class="shimmer absolute inset-0 rounded-lg"></div>
				{/if}
				<img
					src={bannerImage}
					alt="Banner preview"
					class="h-full w-full object-cover {imageLoading
						? 'opacity-0'
						: 'opacity-100'} transition-opacity duration-200"
					onload={handleImageLoad}
				/>
			{/if}

			{#if bannerImage}
				<label
					for="banner-upload"
					class="dark:text-mofu-dark-300 absolute inset-0 flex {isVerified
						? 'cursor-pointer'
						: 'cursor-not-allowed'} items-center justify-center hover:text-gray-300"
				>
					{#if !isVerified}
						<div class="flex flex-col items-center justify-center space-y-2 text-gray-400">
							<Icon src={LockClosed} class="h-8 w-8" />
							<span class="text-xs">이메일 인증 필요</span>
						</div>
					{/if}
				</label>
			{:else}
				<label
					for="banner-upload"
					class="text-mofu-dark-300 flex h-full {isVerified
						? 'cursor-pointer'
						: 'cursor-not-allowed'} flex-col items-center justify-center space-y-2"
				>
					{#if isVerified}
						<Icon src={Photo} class="h-10 w-10" />
						<span class="text-sm">{m.settings_banner_image_upload()}</span>
						<span class="text-xs">{m.settings_banner_image_recommended()}</span>
					{:else}
						<Icon src={LockClosed} class="h-10 w-10 text-gray-400" />
						<span class="text-sm text-gray-400">이메일 인증 필요</span>
						<span class="text-xs text-gray-500">배너 이미지 업로드를 위해 이메일을 인증해주세요</span>
					{/if}
				</label>
			{/if}
		</div>
		<input
			id="banner-upload"
			type="file"
			accept="image/*"
			class="hidden"
			onchange={handleImageChange}
			disabled={!isVerified}
		/>
	</div>
</div>
