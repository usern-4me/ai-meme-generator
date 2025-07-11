use image::{RgbaImage, DynamicImage, ImageReader, ImageError, ImageFormat};
use ab_glyph::{FontArc, Font, PxScale, point};
use std::fs;

pub fn render_text_on_image(
    img_path: &str,
    text: &str,
    font_path: &str,
    font_size: u32,
    position: Option<(u32, u32)>,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let mut img = load_image(img_path)?.to_rgba8();
    let font = get_font(font_path)?;

    let scale = PxScale::from(font_size as f32);
    let (x, y) = position.unwrap_or((0, 0));
    let mut x_cursor = x as f32;

    for c in text.chars() {
        let glyph = font
            .glyph_id(c)
            .with_scale_and_position(scale, point(x_cursor, y as f32));

        if let Some(outlined) = font.outline_glyph(glyph.clone()) {
            outlined.draw(|gx, gy, alpha| {
                let px = (glyph.position.x as u32).saturating_add(gx);
                let py = (glyph.position.y as u32).saturating_add(gy);

                if px < img.width() && py < img.height() {
                    let pixel = img.get_pixel_mut(px, py);

                    let bg_r = pixel[0] as f32 / 255.0;
                    let bg_g = pixel[1] as f32 / 255.0;
                    let bg_b = pixel[2] as f32 / 255.0;
                    let bg_a = pixel[3] as f32 / 255.0;

                    let text_r = 0.1;
                    let text_g = 0.1;
                    let text_b = 0.1;
                    let text_a = alpha;

                    let out_a = text_a + bg_a * (1.0 - text_a);
                    let out_r = (text_r * text_a + bg_r * bg_a * (1.0 - text_a)) / out_a;
                    let out_g = (text_g * text_a + bg_g * bg_a * (1.0 - text_a)) / out_a;
                    let out_b = (text_b * text_a + bg_b * bg_a * (1.0 - text_a)) / out_a;

                    *pixel = image::Rgba([
                        (out_r * 255.0) as u8,
                        (out_g * 255.0) as u8,
                        (out_b * 255.0) as u8,
                        (out_a * 255.0) as u8,
                    ]);
                }
            });
        }

        let advance = font.h_advance_unscaled(glyph.id);
        x_cursor += advance * scale.x;
    }

    fs::create_dir_all("output")?;
    save_image(img.clone())?;
    Ok(DynamicImage::ImageRgba8(img))
}

fn load_image(img_path: &str) -> Result<DynamicImage, ImageError> {
    let img = ImageReader::open(img_path)?.decode()?;
    Ok(img)
}

fn save_image(img: RgbaImage) -> Result<(), ImageError> {
    let dyn_img = DynamicImage::ImageRgba8(img);
    dyn_img.save_with_format("output/out_name.png", ImageFormat::Png)
}

fn get_font(font_path: &str) -> Result<FontArc, std::io::Error> {
    let font_data = fs::read(font_path)?;
    let font = FontArc::try_from_vec(font_data)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid font data"))?;
    Ok(font)
}
