<script lang="ts">
	import Cropper from 'svelte-easy-crop';
	import { Button } from '../ui/button';

	interface CropArea {
		x: number;
		y: number;
		width: number;
		height: number;
	}

	interface CropData {
		croppedAreaPixels: CropArea;
		croppedAreaPercentage: CropArea;
	}

	let {
		isOpen = $bindable(false),
		imageSrc,
		aspectRatio = 1,
		cropShape = 'rect' as 'rect' | 'round',
		onCrop,
		onCancel
	} = $props<{
		isOpen: boolean;
		imageSrc: string;
		aspectRatio?: number;
		cropShape?: 'rect' | 'round';
		onCrop: (data: CropData) => void;
		onCancel: () => void;
	}>();

	let crop = $state({ x: 0, y: 0 });
	let zoom = $state(1);
	let croppedAreaPixels = $state<CropArea | null>(null);
	let croppedAreaPercentage = $state<CropArea | null>(null);

	function handleSave() {
		if (croppedAreaPixels && croppedAreaPercentage) {
			onCrop({ croppedAreaPixels, croppedAreaPercentage });
			isOpen = false;
		}
	}

	function handleCancel() {
		onCancel();
		isOpen = false;
	}
</script>

{#if isOpen}
	<div class="fixed inset-0 z-[60] flex items-center justify-center bg-black/80 p-4">
		<div class="bg-mofu-dark-800 w-full max-w-2xl rounded-lg p-4">
			<div class="relative h-96 w-full">
				<Cropper
					image={imageSrc}
					bind:crop
					bind:zoom
					aspect={aspectRatio}
					{cropShape}
					oncropcomplete={(e) => {
						// oncropcomplete인데 변할 때마다 호출됨
						croppedAreaPixels = e.pixels;
						croppedAreaPercentage = e.percent;
					}}
				/>
			</div>

			<div class="mt-4 flex justify-between">
				<Button
					class="dark:bg-mofu-dark-700 dark:text-mofu-dark-200 hover:dark:bg-mofu-dark-600 rounded border-none px-4 py-2 transition-colors"
					onclick={handleCancel}
				>
					Cancel
				</Button>
				<Button
					class="dark:bg-mofu hover:bg-mofu/80 rounded border-none px-4 py-2 transition-colors dark:text-white"
					onclick={handleSave}
				>
					Crop
				</Button>
			</div>
		</div>
	</div>
{/if}
