<script lang="ts">
	import { Icon, Photo } from 'svelte-hero-icons';
	import * as FileDropZone from '$lib/components/ui/file-drop-zone';
	import { toast } from 'svelte-sonner';

	interface Props {
		thumbnail: string | null;
		onUpdate: (data: { thumbnailFile: Blob; thumbnail: string } | null) => void;
	}

	let { thumbnail, onUpdate }: Props = $props();

	let imageLoading = $state(true);

	async function handleUpload(files: File[]) {
		const file = files[0];
		if (!file) return;

		const fileSizeMB = file.size / (1024 * 1024);
		if (fileSizeMB > 4) {
			toast.error(`File size (${fileSizeMB.toFixed(2)}MB) exceeds 4MB limit.`);
			return;
		}

		const url = URL.createObjectURL(file);
		onUpdate({ thumbnailFile: file, thumbnail: url });
	}

	function handleImageLoad() {
		imageLoading = false;
	}

	function handleImageError() {
		imageLoading = false;
	}

	function removeThumbnail() {
		if (thumbnail?.startsWith('blob:')) {
			URL.revokeObjectURL(thumbnail);
		}
		onUpdate(null);
	}

	$effect(() => {
		if (thumbnail && !thumbnail.startsWith('blob:')) {
			imageLoading = true;
		} else if (thumbnail) {
			imageLoading = false;
		}
	});
</script>

<div class="space-y-4">
	<h2 class="text-sm font-medium">Thumbnail (optional)</h2>

	{#if thumbnail}
		<div class="group relative">
			<div class="relative aspect-video w-full overflow-hidden rounded-lg bg-muted">
				{#if imageLoading && !thumbnail.startsWith('blob:')}
					<div class="shimmer absolute inset-0 rounded-lg"></div>
				{/if}
				<img
					src={thumbnail}
					alt="Thumbnail preview"
					class="h-full w-full object-cover transition-opacity duration-200 {imageLoading &&
					!thumbnail.startsWith('blob:')
						? 'opacity-0'
						: 'opacity-100'}"
					onload={handleImageLoad}
					onerror={handleImageError}
				/>
			</div>
		</div>
		<button
			onclick={removeThumbnail}
			class="text-xs text-destructive underline hover:text-destructive/80"
		>
			Remove thumbnail
		</button>
	{:else}
		<FileDropZone.Root
			accept={FileDropZone.ACCEPT_IMAGE}
			maxFiles={1}
			maxFileSize={4 * FileDropZone.MEGABYTE}
			onUpload={handleUpload}
			onFileRejected={({ reason }) => toast.error(reason)}
		>
			<FileDropZone.Trigger>
				<div
					class="flex aspect-video w-full cursor-pointer flex-col items-center justify-center space-y-2 rounded-lg border border-dashed border-border transition-colors hover:bg-accent/25"
				>
					<Icon src={Photo} class="h-10 w-10 text-muted-foreground" />
					<span class="text-sm text-muted-foreground">Upload thumbnail image</span>
					<span class="text-xs text-muted-foreground">16:9 recommended, max 4MB</span>
				</div>
			</FileDropZone.Trigger>
		</FileDropZone.Root>
	{/if}
</div>
