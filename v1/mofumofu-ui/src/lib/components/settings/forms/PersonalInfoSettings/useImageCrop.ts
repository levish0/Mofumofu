import { getCroppedImg, type CropOptions } from '$lib/utils/imagecrop';

interface CropData {
	croppedAreaPixels: { x: number; y: number; width: number; height: number };
}

export function useImageCrop() {
	let cleanup: (() => void) | null = null;

	async function cropImage(
		imageSrc: string,
		cropData: CropData,
		options: CropOptions
	): Promise<{ blob: Blob; url: string }> {
		if (cleanup) {
			cleanup();
		}

		const { blob, url, cleanup: newCleanup } = await getCroppedImg(imageSrc, cropData.croppedAreaPixels, options);

		cleanup = newCleanup;

		return { blob, url };
	}

	function cleanupTempImage(imageSrc: string) {
		if (imageSrc && imageSrc.startsWith('blob:')) {
			URL.revokeObjectURL(imageSrc);
		}
	}

	function handleFileRead(file: File): Promise<string> {
		return new Promise((resolve, reject) => {
			const reader = new FileReader();
			reader.onload = (e) => {
				const result = e.target?.result;
				if (typeof result === 'string') {
					resolve(result);
				} else {
					reject(new Error('Failed to read file as string'));
				}
			};
			reader.onerror = () => {
				reject(new Error('Failed to read image file'));
			};
			reader.readAsDataURL(file);
		});
	}

	return {
		cropImage,
		cleanupTempImage,
		handleFileRead
	};
}
