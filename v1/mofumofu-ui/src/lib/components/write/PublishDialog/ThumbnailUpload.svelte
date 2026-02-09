<script lang="ts">
	import { Icon, Photo } from 'svelte-hero-icons';

	interface Props {
		thumbnail: string | null;
		onUpdate: (data: { thumbnailFile: Blob; thumbnail: string } | null) => void;
	}

	let { thumbnail, onUpdate }: Props = $props();

	let imageLoading = $state(true);

	async function handleImageChange(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file && file.type.startsWith('image/')) {
			// 파일 크기 체크 (4MB 제한)
			const fileSizeMB = file.size / (1024 * 1024);
			if (fileSizeMB > 4) {
				alert(`파일 크기가 ${fileSizeMB.toFixed(2)}MB로 4MB 제한을 초과합니다.`);
				return;
			}

			try {
				const url = URL.createObjectURL(file);
				onUpdate({
					thumbnailFile: file,
					thumbnail: url
				});
			} catch (error) {
				console.error('Error uploading thumbnail:', error);
				if (error instanceof Error) {
					alert(`썸네일 업로드 실패: ${error.message}`);
				}
			}
		}
		target.value = '';
	}

	function handleImageLoad() {
		imageLoading = false;
	}

	function handleImageError() {
		imageLoading = false;
	}

	function removeThumbnail() {
		if (thumbnail && thumbnail.startsWith('blob:')) {
			URL.revokeObjectURL(thumbnail);
		}
		onUpdate(null);
	}

	// Reset loading state when image URL changes
	$effect(() => {
		if (thumbnail && !thumbnail.startsWith('blob:')) {
			imageLoading = true;
		} else if (thumbnail && thumbnail.startsWith('blob:')) {
			imageLoading = false;
		}
	});
</script>

<div class="space-y-4">
	<h2 class="text-mofu-light-100 dark:text-mofu-dark-100 text-lg font-medium">썸네일 (선택사항)</h2>
	<div class="group relative transition-all">
		<div
			class="dark:bg-mofu-dark-800 bg-mofu-light-800 relative aspect-video w-full overflow-hidden rounded-lg group-hover:opacity-75"
		>
			{#if thumbnail}
				<!-- Loading shimmer -->
				{#if imageLoading && !thumbnail.startsWith('blob:')}
					<div class="shimmer absolute inset-0 rounded-lg"></div>
				{/if}
				<img
					src={thumbnail}
					alt="썸네일 미리보기"
					class="h-full w-full object-cover {imageLoading && !thumbnail.startsWith('blob:')
						? 'opacity-0'
						: 'opacity-100'} transition-opacity duration-200"
					onload={handleImageLoad}
					onerror={handleImageError}
				/>
				<label
					for="thumbnail-upload"
					class="dark:text-mofu-dark-300 text-mofu-light-300 hover:text-mofu-light-400 dark:hover:text-mofu-dark-400 absolute inset-0 flex cursor-pointer items-center justify-center"
				>
				</label>
			{:else}
				<label
					for="thumbnail-upload"
					class="dark:text-mofu-dark-300 text-mofu-light-300 flex h-full cursor-pointer flex-col items-center justify-center space-y-2"
				>
					<Icon src={Photo} class="h-10 w-10" />
					<span class="text-sm">썸네일 이미지 업로드</span>
					<span class="text-xs">16:9 비율 권장, 최대 4MB</span>
				</label>
			{/if}
		</div>
		<input id="thumbnail-upload" type="file" accept="image/*" class="hidden" onchange={handleImageChange} />
	</div>
	{#if thumbnail}
		<button onclick={removeThumbnail} class="text-xs text-rose-500 underline hover:text-rose-400"> 썸네일 제거 </button>
	{/if}
</div>
