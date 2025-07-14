use image::{DynamicImage, ImageFormat, Rgba};
use imageproc::drawing::{draw_text_mut, text_size};
use ab_glyph::{FontArc, PxScale};
use std::fs;

pub fn render_text_on_image(
    img_path: &str,
    text: &str,
    font_path: &str,
    font_size: f32,
    position: Option<(i32, i32)>,
    alignment: bool,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let mut img = image::open(img_path)?.to_rgba8();
    let (img_width, _) = img.dimensions();
    let (mut start_x, start_y) = position.unwrap_or((10, 10));

    let font_data = fs::read(font_path)?;
    let font = FontArc::try_from_vec(font_data)?;
    let scale = PxScale::from(font_size);

    let words = separate_text_into_words(text);
    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();

    let (mut line_width ,  _)= (10, 10 as u32);
    for word in words {
        let test_line = if current_line.is_empty() {
            word.clone()
        } else {
            format!("{} {}", current_line, word)
        };
        (line_width , _) = text_size(scale, &font, &test_line);
        if start_x as u32 + line_width > img_width {
            lines.push(current_line.clone());
            current_line = word;
        } else {
            current_line = test_line;
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    let mut y = start_y;
    for line in lines {
        let (line_width, _) = text_size(scale, &font, &line);
        if alignment {
            start_x = ((img_width - line_width) / 2) as i32;
        }
        draw_text_mut(
            &mut img,
            Rgba([0, 0, 0, 255]),
            start_x,
            y,
            scale,
            &font,
            &line,
        );
        y += font_size as i32 + 5;
    }

    fs::create_dir_all("output")?;
    let out = DynamicImage::ImageRgba8(img);
    out.save_with_format("output/Output.png", ImageFormat::Png)?;
    Ok(out)
}

pub fn separate_text_into_words(text: &str) -> Vec<String> {
    text.split_whitespace().map(str::to_string).collect()
}