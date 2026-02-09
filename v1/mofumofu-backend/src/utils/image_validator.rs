use crate::service::error::errors::Errors;
use crate::utils::image_processor::ImageProcessor;
use infer;

pub fn validate_and_get_image_info(
    data: &[u8],
    max_size: usize,
) -> Result<(String, &'static str), Errors> {
    if data.is_empty() {
        return Err(Errors::BadRequestError("Empty file".to_string()));
    }

    // Check file size
    if data.len() > max_size {
        return Err(Errors::FileTooLargeError(format!(
            "Image too large: {} bytes (max: {} bytes)",
            data.len(),
            max_size
        )));
    }

    // Use infer crate to detect file type
    match infer::get(data) {
        Some(kind) => match kind.mime_type() {
            "image/jpeg" => Ok((kind.mime_type().to_string(), "jpg")),
            "image/png" => Ok((kind.mime_type().to_string(), "png")),
            "image/gif" => Ok((kind.mime_type().to_string(), "gif")),
            "image/webp" => Ok((kind.mime_type().to_string(), "webp")),
            _ => Err(Errors::BadRequestError(format!(
                "Unsupported image format: {}",
                kind.mime_type()
            ))),
        },
        None => Err(Errors::BadRequestError(
            "Cannot determine file type or unsupported format".to_string(),
        )),
    }
}

pub fn process_image_for_upload(
    data: &[u8],
    max_size: usize,
    compress_to_webp: bool,
    max_dimensions: Option<(u32, u32)>,
) -> Result<(Vec<u8>, String, &'static str), Errors> {
    let (original_mime_type, original_extension) = validate_and_get_image_info(data, max_size)?;

    // Handle GIF files separately - keep them as GIF
    if original_mime_type == "image/gif" {
        let optimized_data = ImageProcessor::optimize_gif(data)?;
        return Ok((optimized_data, "image/gif".to_string(), "gif"));
    }

    // For other image types, compress and optionally convert to WebP
    if compress_to_webp {
        let compressed_data = ImageProcessor::compress_and_convert(
            data,
            image::ImageFormat::WebP,
            Some(90), // Default quality
            max_dimensions,
        )?;
        Ok((compressed_data, "image/webp".to_string(), "webp"))
    } else {
        // Keep original format but apply compression/resizing
        let format = match original_mime_type.as_str() {
            "image/jpeg" => image::ImageFormat::Jpeg,
            "image/png" => image::ImageFormat::Png,
            _ => return Ok((data.to_vec(), original_mime_type, original_extension)),
        };

        let compressed_data =
            ImageProcessor::compress_and_convert(data, format, Some(85), max_dimensions)?;
        Ok((compressed_data, original_mime_type, original_extension))
    }
}

pub fn generate_image_hash(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
