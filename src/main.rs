use std::env;
use image::Rgba;
mod render;

fn parse_arg<T: std::str::FromStr>(args: &[String], index: usize, default: T) -> T {
    args.get(index)
        .and_then(|val| val.parse::<T>().ok())
        .unwrap_or(default)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let text = args.get(1).cloned().unwrap_or_else(|| "Hello, world!".to_string());
    let font_size: f32 = parse_arg(&args, 2, 64.0);
    let align_center: bool = parse_arg(&args, 3, false);
    let x = parse_arg(&args, 4, 50);
    let y = parse_arg(&args, 5, 50);
    let r = parse_arg(&args, 6, 255);
    let g = parse_arg(&args, 7, 255);
    let b = parse_arg(&args, 8, 255);
    let template_path = args.get(9).unwrap_or(&"assets/templates/default.jpg".to_string()).clone();
    let font_path = args.get(10).unwrap_or(&"assets/fonts/High Empathy.ttf".to_string()).clone();


    let result = render::render_text_on_image(
        &template_path,
        &text,
        &font_path,
        font_size,
        Some((x, y)),
        align_center,
    );


    match result {
        Ok(_) => println!("Saved output/out_name.png with your text!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

