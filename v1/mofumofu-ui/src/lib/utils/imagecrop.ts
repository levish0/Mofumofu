export interface CroppedArea {
	x: number;
	y: number;
	width: number;
	height: number;
}

export interface CropOptions {
	maxFileSizeMB?: number;
}

/**
 * Creates a cropped image from the original image and crop area
 */
export async function getCroppedImg(
	imageSrc: string,
	pixelCrop: CroppedArea,
	options: CropOptions = {}
): Promise<{ blob: Blob; url: string; cleanup: () => void }> {
	const image = await createImage(imageSrc);
	const { maxFileSizeMB = 4 } = options;

	let sourceCanvas: HTMLCanvasElement | null = null;
	let objectUrl: string | null = null;

	try {
		// Create source canvas with cropped area
		sourceCanvas = document.createElement('canvas');
		const sourceCtx = sourceCanvas.getContext('2d');

		if (!sourceCtx) {
			throw new Error('No 2d context');
		}

		// Set source canvas size to the crop area
		sourceCanvas.width = pixelCrop.width;
		sourceCanvas.height = pixelCrop.height;

		// Draw the cropped image onto the source canvas
		sourceCtx.drawImage(
			image,
			pixelCrop.x,
			pixelCrop.y,
			pixelCrop.width,
			pixelCrop.height,
			0,
			0,
			pixelCrop.width,
			pixelCrop.height
		);

		// Convert canvas to blob without compression (server will handle compression)
		const result = await new Promise<{ blob: Blob; url: string }>((resolve, reject) => {
			sourceCanvas!.toBlob(
				(blob) => {
					if (!blob) {
						reject(new Error('Canvas is empty'));
						return;
					}

					// Check file size limit
					const fileSizeMB = blob.size / (1024 * 1024);
					if (fileSizeMB > maxFileSizeMB) {
						reject(new Error(`File size ${fileSizeMB.toFixed(2)}MB exceeds limit of ${maxFileSizeMB}MB`));
						return;
					}

					objectUrl = URL.createObjectURL(blob);
					resolve({ blob, url: objectUrl });
				},
				'image/png' // Use PNG to preserve quality for server-side compression
			);
		});

		// Cleanup function to revoke object URL
		const cleanup = () => {
			if (objectUrl) {
				URL.revokeObjectURL(objectUrl);
				objectUrl = null;
			}
		};

		return { ...result, cleanup };
	} finally {
		// Clean up source canvas immediately
		if (sourceCanvas) {
			sourceCanvas.width = 0;
			sourceCanvas.height = 0;
		}
	}
}

/**
 * Creates an Image object from a source URL
 */
function createImage(url: string): Promise<HTMLImageElement> {
	return new Promise((resolve, reject) => {
		const image = new Image();
		image.addEventListener('load', () => resolve(image));
		image.addEventListener('error', (error) => reject(error));
		image.setAttribute('crossOrigin', 'anonymous');
		image.src = url;
	});
}
