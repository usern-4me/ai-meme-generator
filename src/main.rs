// Import the `env` module from the Rust standard library to work with command-line arguments
use std::env;

// Declare a module named `render`, assumed to contain the rendering functionality
mod render;

// Generic function to parse a command-line argument at a given index into a specified type T
// If the argument is missing or cannot be parsed, it falls back to the provided default value
fn parse_arg<T: std::str::FromStr>(args: &[String], index: usize, default: T) -> T {
    args.get(index)                            // Try to get the argument at the given index
        .and_then(|val| val.parse::<T>().ok()) // Attempt to parse it into type T
        .unwrap_or(default)                    // Use default value if parsing fails or value is absent
}

fn main() {
    // Collect all command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Get the text to render from the first argument, or default to "Hello, world!"
    let text = args.get(1).cloned().unwrap_or_else(|| "Hello, world!".to_string());

    // Parse the font size from argument 2 or use default value 64.0
    let font_size: f32 = parse_arg(&args, 2, 64.0);

    // Parse the alignment flag (whether to center the text) from argument 3, default to false
    let align_center: bool = parse_arg(&args, 3, false);

    // Parse the x-coordinate from argument 4, default to 50
    let x = parse_arg(&args, 4, 50);

    // Parse the y-coordinate from argument 5, default to 50
    let y = parse_arg(&args, 5, 50);

    // Parse the red color value from argument 6, default to 255
    let r = parse_arg(&args, 6, 255);

    // Parse the green color value from argument 7, default to 255
    let g = parse_arg(&args, 7, 255);

    // Parse the blue color value from argument 8, default to 255
    let b = parse_arg(&args, 8, 255);

    // Get the template image path from argument 9 or use the default image path
    let template_path = args.get(9).unwrap_or(&"assets/templates/default.jpg".to_string()).clone();

    // Get the font file path from argument 10 or use the default font path
    let font_path = args.get(10).unwrap_or(&"assets/fonts/High Empathy.ttf".to_string()).clone();

    // Call the `render_text_on_image` function from the `render` module with the provided or default parameters
    let result = render::render_text_on_image(
        &template_path,  // Path to the image template
        &text,           // Text to render
        &font_path,      // Font file path
        font_size,       // Font size
        Some((x, y)),    // Coordinates for text placement
        r, g, b,         // Text color (RGB)
        align_center,    // Whether to center the text
    );

    // Check whether rendering succeeded or failed
    match result {
        Ok(_) => println!("Saved output/out_name.png with your text!"), // Success message
        Err(e) => eprintln!("Error: {}", e),                            // Print error if rendering failed
    }
}
