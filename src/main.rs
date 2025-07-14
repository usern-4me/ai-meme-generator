mod cli;
mod logger;
mod commands;
mod image;
mod render;





fn main() {
    logger::init_logger();
    cli::run_cli();




    let img_path = "assets/templates/pobrane.jpg";
    let text = "Text text text text text text text text text text text text text";
    let font_path = "assets/fonts/High Empathy.ttf";
    let font_size: f32 = 64.0;
    let position = Some((400, 100));
    let center_alignment = true;

    match render::render_text_on_image(img_path, text, font_path, font_size, position, center_alignment) {
        Ok(img) => println!("Image rendered successfully!"),
        Err(e) => eprintln!("Error rendering image: {}", e),
    }
    
        
}



