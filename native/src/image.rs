//! Pure-Rust image-processing core.
//!
//! This module is intentionally independent of JNI, Android, and Skia so it can
//! be tested quickly and deterministically (Layer A). Pixels are stored as
//! RGBA8 (4 bytes per pixel, row stride = width * 4).

/// Errors returned by image-processing operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageError {
    ZeroDimension,
    NegativeDimension,
    BufferTooSmall { needed: usize, provided: usize },
    Overflow,
    InvalidPixelFormat,
}

impl core::fmt::Display for ImageError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ImageError::ZeroDimension => write!(f, "image dimension is zero"),
            ImageError::NegativeDimension => write!(f, "image dimension is negative"),
            ImageError::BufferTooSmall { needed, provided } => {
                write!(f, "buffer too small: needed {needed} bytes, provided {provided}")
            }
            ImageError::Overflow => write!(f, "integer overflow in size calculation"),
            ImageError::InvalidPixelFormat => write!(f, "invalid pixel format"),
        }
    }
}

impl core::error::Error for ImageError {}

/// Number of bytes per pixel for the RGBA8 format.
pub const BYTES_PER_PIXEL: usize = 4;

/// Required buffer length in bytes for an RGBA8 image of the given dimensions.
///
/// Returns `None` if the computation would overflow.
pub fn required_buffer_len(width: u32, height: u32) -> Option<usize> {
    let width = usize::try_from(width).ok()?;
    let height = usize::try_from(height).ok()?;
    width.checked_mul(height)?.checked_mul(BYTES_PER_PIXEL)
}

/// Validate dimensions and buffer length for an RGBA8 image.
fn validate(width: i32, height: i32, buffer_len: usize) -> Result<(usize, usize), ImageError> {
    if width < 0 || height < 0 {
        return Err(ImageError::NegativeDimension);
    }
    let width = u32::try_from(width).map_err(|_| ImageError::NegativeDimension)?;
    let height = u32::try_from(height).map_err(|_| ImageError::NegativeDimension)?;
    if width == 0 || height == 0 {
        return Err(ImageError::ZeroDimension);
    }
    let needed = required_buffer_len(width, height).ok_or(ImageError::Overflow)?;
    if buffer_len < needed {
        return Err(ImageError::BufferTooSmall { needed, provided: buffer_len });
    }
    Ok((width as usize, height as usize))
}

/// Invert the RGB channels of every pixel in place, preserving the alpha channel.
///
/// `buffer` must hold at least `width * height * 4` bytes in RGBA8 order.
pub fn invert_colors(buffer: &mut [u8], width: i32, height: i32) -> Result<(), ImageError> {
    let (width, height) = validate(width, height, buffer.len())?;
    let stride = width * BYTES_PER_PIXEL;
    for row in 0..height {
        let row_start = row * stride;
        for col in 0..width {
            let base = row_start + col * BYTES_PER_PIXEL;
            buffer[base] = 255u8.wrapping_sub(buffer[base]);     // R
            buffer[base + 1] = 255u8.wrapping_sub(buffer[base + 1]); // G
            buffer[base + 2] = 255u8.wrapping_sub(buffer[base + 2]); // B
            // alpha (base + 3) is preserved
        }
    }
    Ok(())
}

/// Fill every pixel with a single RGBA8 value (useful for fixtures).
pub fn fill(buffer: &mut [u8], width: i32, height: i32, rgba: [u8; 4]) -> Result<(), ImageError> {
    let (width, height) = validate(width, height, buffer.len())?;
    let stride = width * BYTES_PER_PIXEL;
    for row in 0..height {
        let row_start = row * stride;
        for col in 0..width {
            let base = row_start + col * BYTES_PER_PIXEL;
            buffer[base..base + 4].copy_from_slice(&rgba);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgba(r: u8, g: u8, b: u8, a: u8) -> [u8; 4] {
        [r, g, b, a]
    }

    fn buf_for(w: i32, h: i32) -> Vec<u8> {
        vec![0u8; (w as usize) * (h as usize) * BYTES_PER_PIXEL]
    }

    #[test]
    fn required_buffer_len_matches_width_height_times_four() {
        assert_eq!(required_buffer_len(2, 3), Some(2 * 3 * 4));
        assert_eq!(required_buffer_len(1, 1), Some(4));
    }

    #[test]
    fn required_buffer_len_overflows_for_huge_dimensions() {
        // u32::MAX * u32::MAX * 4 overflows usize on 64-bit and 32-bit targets.
        assert_eq!(required_buffer_len(u32::MAX, u32::MAX), None);
    }

    #[test]
    fn invert_colors_inverts_rgb_preserves_alpha() {
        let mut buffer = buf_for(2, 2);
        fill(&mut buffer, 2, 2, rgba(10, 20, 30, 200)).unwrap();

        invert_colors(&mut buffer, 2, 2).unwrap();

        // Row-major RGBA8: (R,G,B,A) per pixel.
        assert_eq!(&buffer[0..4], &[245, 235, 225, 200]);
        assert_eq!(&buffer[4..8], &[245, 235, 225, 200]);
        assert_eq!(&buffer[8..12], &[245, 235, 225, 200]);
        assert_eq!(&buffer[12..16], &[245, 235, 225, 200]);
    }

    #[test]
    fn invert_colors_preserves_alpha_differently_per_pixel() {
        let mut buffer = buf_for(2, 1);
        // pixel 0: alpha 5, pixel 1: alpha 250
        buffer.copy_from_slice(&[1, 2, 3, 5, 4, 5, 6, 250]);

        invert_colors(&mut buffer, 2, 1).unwrap();

        assert_eq!(&buffer[0..4], &[254, 253, 252, 5]);
        assert_eq!(&buffer[4..8], &[251, 250, 249, 250]);
    }

    #[test]
    fn invert_colors_handles_full_range_values() {
        let mut buffer = buf_for(1, 1);
        buffer.copy_from_slice(&[0, 0, 0, 0]);
        invert_colors(&mut buffer, 1, 1).unwrap();
        assert_eq!(&buffer[0..4], &[255, 255, 255, 0]);

        buffer.copy_from_slice(&[255, 255, 255, 255]);
        invert_colors(&mut buffer, 1, 1).unwrap();
        assert_eq!(&buffer[0..4], &[0, 0, 0, 255]);
    }

    #[test]
    fn invert_colors_rejects_zero_width() {
        let mut buffer = buf_for(0, 0);
        assert_eq!(
            invert_colors(&mut buffer, 0, 5),
            Err(ImageError::ZeroDimension)
        );
    }

    #[test]
    fn invert_colors_rejects_zero_height() {
        let mut buffer = buf_for(0, 0);
        assert_eq!(
            invert_colors(&mut buffer, 5, 0),
            Err(ImageError::ZeroDimension)
        );
    }

    #[test]
    fn invert_colors_rejects_negative_dimensions() {
        let mut buffer = buf_for(1, 1);
        assert_eq!(
            invert_colors(&mut buffer, -1, 5),
            Err(ImageError::NegativeDimension)
        );
        assert_eq!(
            invert_colors(&mut buffer, 5, -5),
            Err(ImageError::NegativeDimension)
        );
    }

    #[test]
    fn invert_colors_rejects_buffer_too_small() {
        // 1x1 needs 4 bytes; provide only 2.
        let mut buffer = vec![0u8; 2];
        assert_eq!(
            invert_colors(&mut buffer, 1, 1),
            Err(ImageError::BufferTooSmall {
                needed: 4,
                provided: 2
            })
        );
    }

    #[test]
    fn invert_colors_accepts_exact_size_buffer() {
        let mut buffer = vec![0u8; 4];
        assert!(invert_colors(&mut buffer, 1, 1).is_ok());
    }

    #[test]
    fn invert_colors_accepts_larger_buffer() {
        // Extra trailing bytes are ignored.
        let mut buffer = vec![0u8; 100];
        assert!(invert_colors(&mut buffer, 1, 1).is_ok());
    }

    #[test]
    fn invert_colors_handles_row_stride_layout() {
        // 2 pixels wide x 2 tall = stride 8 bytes per row.
        let mut buffer = buf_for(2, 2);
        fill(&mut buffer, 2, 2, rgba(0, 0, 0, 255)).unwrap();
        invert_colors(&mut buffer, 2, 2).unwrap();
        let expected: &[u8] = &[
            255, 255, 255, 255, // row 0 pixel 0
            255, 255, 255, 255, // row 0 pixel 1
            255, 255, 255, 255, // row 1 pixel 0
            255, 255, 255, 255, // row 1 pixel 1
        ];
        assert_eq!(&buffer, expected);
    }

    #[test]
    fn empty_buffer_rejects_any_nonzero_dimensions() {
        assert_eq!(
            invert_colors(&mut [], 1, 1),
            Err(ImageError::BufferTooSmall {
                needed: 4,
                provided: 0
            })
        );
    }

    #[test]
    fn fill_writes_uniform_rgba() {
        let mut buffer = buf_for(2, 2);
        fill(&mut buffer, 2, 2, rgba(7, 8, 9, 10)).unwrap();
        for chunk in buffer.chunks(4) {
            assert_eq!(chunk, &[7, 8, 9, 10]);
        }
    }
}
