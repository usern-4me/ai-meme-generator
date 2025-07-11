use image::{RgbaImage, DynamicImage, ImageReader, ImageError, ImageFormat};
use ab_glyph::FontArc;
use std::fs::read;



pub fn render_text_on_image(
    img_path: &str,
    text: &str,
    font_path: &str,
    font_size: u32,
    position: Option<(u32, u32)>
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let mut img = load_image(img_path)?.to_rgba8();


    save_image(img.clone())?;
    Ok(DynamicImage::ImageRgba8(img))
}


pub fn load_image(
    img_path: &str
) -> Result<DynamicImage, ImageError> {
    let img = ImageReader::open(img_path)?.decode()?;
    Ok(img)
}

pub fn save_image(img: RgbaImage) -> Result<(), ImageError> {
    let dyn_img = DynamicImage::ImageRgba8(img);
    dyn_img.save_with_format("output/out_name.png", ImageFormat::Png)
}
