<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar';
	import type { ImageCropperPreviewProps } from '$lib/components/ui/image-cropper/types';
	import { useImageCropperPreview } from '$lib/components/ui/image-cropper/image-cropper.svelte.js';
	import UploadIcon from '@lucide/svelte/icons/upload';
	import { cn } from '$lib/utils.js';

	let { child, class: className }: ImageCropperPreviewProps = $props();

	const previewState = useImageCropperPreview();
</script>

{#if child}
	{@render child({ src: previewState.rootState.src })}
{:else}
	<Avatar.Root
		class={cn('size-20 ring-2 ring-accent ring-offset-2 ring-offset-background', className)}
	>
		<Avatar.Image src={previewState.rootState.src} />
		<Avatar.Fallback>
			<UploadIcon class="size-4" />
			<span class="sr-only">Upload image</span>
		</Avatar.Fallback>
	</Avatar.Root>
{/if}
