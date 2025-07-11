use image::{DynamicImage, ImageFormat, Rgba};
use imageproc::drawing::draw_text_mut;
use ab_glyph::{FontArc, PxScale};
use std::fs;


pub fn render_text_on_image(
    img_path: &str,
    text: &str,
    font_path: &str,
    font_size: f32,
    position: Option<(i32, i32)>,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let mut img = image::open(img_path)?
        .to_rgba8();

    let font_data = fs::read(font_path)?;
    let font = FontArc::try_from_vec(font_data)?;

    let scale = PxScale::from(font_size);

    let (x, y) = position.unwrap_or((10, 10));

    draw_text_mut(
        &mut img,
        Rgba([0, 0, 0, 255]),     // black
        x, y,
        scale,
        &font,
        text,
    );

    let (w, h) = imageproc::drawing::text_size(scale, &font, text);
    println!("Rendered text size: {}x{}", w, h);

    fs::create_dir_all("output")?;
    let out = DynamicImage::ImageRgba8(img);
    out.save_with_format("output/out_name.png", ImageFormat::Png)?;
    Ok(out)
}



//TODO- next line when reached border, chose starting point(start from same width), font size