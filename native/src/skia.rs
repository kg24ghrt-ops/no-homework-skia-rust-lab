//! Skia-backed image generators.
//!
//! This module is behind the `skia` feature because it pulls in the large
//! skia-safe dependency graph. It is tested separately (Layer B).

use crate::image::ImageError;
use skia_safe::{Color4f, EncodedImageFormat, ISize, Paint, PaintStyle, Point, Rect, Surface};

/// B5 paper width in millimeters (common notebook size in Asia).
pub const B5_WIDTH_MM: f32 = 176.0;

/// B5 paper height in millimeters (common notebook size in Asia).
pub const B5_HEIGHT_MM: f32 = 250.0;

/// Default DPI for notebook paper rendering.
pub const DEFAULT_DPI: f32 = 150.0;

/// Convert millimeters to pixels at the given DPI.
fn mm_to_px(mm: f32, dpi: f32) -> i32 {
    (mm / 25.4 * dpi).round() as i32
}

/// Dimensions for B5 paper at a given DPI.
pub struct B5Dimensions {
    pub width: i32,
    pub height: i32,
}

impl B5Dimensions {
    pub fn at_dpi(dpi: f32) -> Self {
        Self {
            width: mm_to_px(B5_WIDTH_MM, dpi),
            height: mm_to_px(B5_HEIGHT_MM, dpi),
        }
    }
}

/// Generate lined notebook paper as PNG-encoded RGBA8 bytes.
///
/// White background with light gray ruled horizontal lines and a red
/// margin line on the left.
///
/// # Errors
///
/// Returns [`ImageError::ZeroDimension`] if width or height is not positive.
/// Returns [`ImageError::InvalidPixelFormat`] if line spacing is not positive
/// or if Skia fails to produce the image.
pub fn generate_notebook_paper_png(
    width: i32,
    height: i32,
    line_spacing: f32,
    margin_width: f32,
) -> Result<Vec<u8>, ImageError> {
    if width <= 0 || height <= 0 {
        return Err(ImageError::ZeroDimension);
    }
    if line_spacing <= 0.0 {
        return Err(ImageError::InvalidPixelFormat);
    }

    let mut surface = Surface::new_raster_n32_premul(ISize::new(width, height))
        .ok_or(ImageError::InvalidPixelFormat)?;
    let canvas = surface.canvas();

    // White background.
    let mut bg_paint = Paint::new(Color4f::new(1.0, 1.0, 1.0, 1.0), None);
    bg_paint.set_style(PaintStyle::Fill);
    canvas.draw_rect(Rect::from_wh(width as f32, height as f32), &bg_paint);

    // Ruled lines in light gray (0xd0 = 208/255).
    let mut line_paint = Paint::new(Color4f::new(0.816, 0.816, 0.816, 1.0), None);
    line_paint.set_style(PaintStyle::Stroke);
    line_paint.set_stroke_width(1.0);

    let mut y = line_spacing;
    while y < height as f32 {
        canvas.draw_line(Point::new(0.0, y), Point::new(width as f32, y), &line_paint);
        y += line_spacing;
    }

    // Red margin line.
    let mut margin_paint = Paint::new(Color4f::new(1.0, 0.0, 0.0, 1.0), None);
    margin_paint.set_style(PaintStyle::Stroke);
    margin_paint.set_stroke_width(2.0);

    canvas.draw_line(
        Point::new(margin_width, 0.0),
        Point::new(margin_width, height as f32),
        &margin_paint,
    );

    let image = surface.image_snapshot();
    let data = image
        .encode_to_data(EncodedImageFormat::PNG)
        .ok_or(ImageError::InvalidPixelFormat)?;
    Ok(data.as_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png_signature(bytes: &[u8]) -> bool {
        bytes.len() >= 8 && bytes[0..8] == [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]
    }

    #[test]
    fn notebook_paper_is_valid_png() {
        let png = generate_notebook_paper_png(100, 100, 10.0, 10.0).unwrap();
        assert!(png_signature(&png));
        assert!(!png.is_empty());
    }

    #[test]
    fn notebook_paper_rejects_zero_width() {
        assert_eq!(
            generate_notebook_paper_png(0, 100, 10.0, 10.0),
            Err(ImageError::ZeroDimension)
        );
    }

    #[test]
    fn notebook_paper_rejects_zero_height() {
        assert_eq!(
            generate_notebook_paper_png(100, 0, 10.0, 10.0),
            Err(ImageError::ZeroDimension)
        );
    }

    #[test]
    fn notebook_paper_rejects_negative_dimensions() {
        assert_eq!(
            generate_notebook_paper_png(-1, 100, 10.0, 10.0),
            Err(ImageError::ZeroDimension)
        );
    }

    #[test]
    fn notebook_paper_rejects_nonpositive_line_spacing() {
        assert_eq!(
            generate_notebook_paper_png(100, 100, 0.0, 10.0),
            Err(ImageError::InvalidPixelFormat)
        );
        assert_eq!(
            generate_notebook_paper_png(100, 100, -5.0, 10.0),
            Err(ImageError::InvalidPixelFormat)
        );
    }

    #[test]
    fn b5_dimensions_at_150dpi() {
        let dims = B5Dimensions::at_dpi(150.0);
        assert_eq!(dims.width, mm_to_px(B5_WIDTH_MM, 150.0));
        assert_eq!(dims.height, mm_to_px(B5_HEIGHT_MM, 150.0));
        assert!(dims.width > 0);
        assert!(dims.height > 0);
    }
}
