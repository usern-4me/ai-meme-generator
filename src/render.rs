// Import types and functions from the `image` crate
use image::{DynamicImage, ImageFormat, Rgba};

// Import text drawing utilities from the `imageproc` crate
use imageproc::drawing::{draw_text_mut, text_size};

// Import font handling utilities from the `ab_glyph` crate
use ab_glyph::{FontArc, PxScale};

// Standard file system module for reading files and creating directories
use std::fs;

// Function to render text onto an image with given parameters
pub fn render_text_on_image(
    img_path: &str,           // Path to the background/template image
    text: &str,               // Text to be drawn
    font_path: &str,          // Path to the font file
    font_size: f32,           // Desired font size
    position: Option<(i32, i32)>, // Optional text starting position (x, y)
    der: u8,                  // Red component of the text color
    grass: u8,                // Green component of the text color
    bule: u8,                 // Blue component of the text color
    alignment: bool,          // Whether to center-align the text
) -> Result<DynamicImage, Box<dyn std::error::Error>> {

    // Open the image from the given path and convert it to an RGBA image
    let mut img = image::open(img_path)?.to_rgba8();

    // Get the width of the image; height is unused
    let (img_width, _) = img.dimensions();

    // Use the provided position or default to (10, 10)
    let (mut start_x, start_y) = position.unwrap_or((10, 10));

    // Read the font file into memory
    let font_data = fs::read(font_path)?;

    // Create a font object from the font data
    let font = FontArc::try_from_vec(font_data)?;

    // Set the pixel scale based on the provided font size
    let scale = PxScale::from(font_size);

    // Break the input text into individual words
    let words = separate_text_into_words(text);

    // Vector to store lines of text after wrapping
    let mut lines: Vec<String> = Vec::new();

    // Temporary string to build the current line
    let mut current_line = String::new();

    // Used to calculate width of text lines
    let (mut line_width , _) = (10, 10 as u32); // Dummy initial values

    // Loop through each word to build properly wrapped lines
    for word in words {
        // Try adding the word to the current line
        let test_line = if current_line.is_empty() {
            word.clone() // Start a new line
        } else {
            format!("{} {}", current_line, word) // Append word to current line
        };

        // Measure width of the test line
        (line_width , _) = text_size(scale, &font, &test_line);

        // If line is too wide to fit on the image, push current line and start new one
        if start_x as u32 + line_width > img_width {
            lines.push(current_line.clone());
            current_line = word;
        } else {
            // Otherwise, use the test line as the new current line
            current_line = test_line;
        }
    }

    // Push any remaining text as the final line
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    // Start vertical positioning at the Y coordinate
    let mut y = start_y;

    // Draw each line of text onto the image
    for line in lines {
        // Measure the width of the line to center-align if necessary
        let (line_width, _) = text_size(scale, &font, &line);

        // If alignment flag is set, center the line horizontally
        if alignment {
            start_x = ((img_width - line_width) / 2) as i32;
        }

        // Draw the text line onto the image with the specified color and font
        draw_text_mut(
            &mut img,                            // Image buffer
            Rgba([der, grass, bule, 255]),       // RGBA color (255 = full opacity)
            start_x,                             // X coordinate
            y,                                   // Y coordinate
            scale,                               // Font size
            &font,                               // Font to use
            &line,                               // Text to draw
        );

        // Move down for the next line of text
        y += font_size as i32 + 5; // Add spacing between lines
    }

    // Ensure the output directory exists
    fs::create_dir_all("output")?;

    // Convert the image buffer back to a `DynamicImage`
    let out = DynamicImage::ImageRgba8(img);

    // Save the image to disk as a PNG
    out.save_with_format("output/Output.png", ImageFormat::Png)?;

    // Return the final image
    Ok(out)
}

// Splits the input text into words using whitespace and returns them as a vector of strings
pub fn separate_text_into_words(text: &str) -> Vec<String> {
    text.split_whitespace()             // Split on spaces, tabs, etc.
        .map(str::to_string)            // Convert each &str to String
        .collect()                      // Collect into a Vec<String>
}
