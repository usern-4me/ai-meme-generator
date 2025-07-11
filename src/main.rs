mod cli;
mod logger;
mod commands;
mod template;
mod render;





fn main() {
    logger::init_logger();
    cli::run_cli();




    let img_path = "assets/templates/pobrane.jpg";
    let text = "Hello, world!";
    let font_path = "assets/fonts/arial.ttf";
    let font_size = 32;
    let position = Some((100, 100));

    match render::render_text_on_image(img_path, text, font_path, font_size, position) {
        Ok(img) => println!("Image rendered successfully!"),
        Err(e) => eprintln!("Error rendering image: {}", e),
    }
        
}



