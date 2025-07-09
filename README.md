
# AI-Powered Meme Generator in Rust

![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![OpenAI](https://img.shields.io/badge/OpenAI-GPT-blue.svg)

A **local CLI application** to generate memes with AI-powered captions and customizable templates — designed to run on **Windows** (and cross-platform). No server needed; just clone, configure, and run!

---

## Features

- Generate meme captions using OpenAI GPT models  
- Render captions dynamically on meme templates  
- Support multiple text boxes, font styles, and text wrapping  
- Easy-to-use CLI with template listing and generation commands  
- Saves generated memes as PNG images locally  
- Fully offline except for OpenAI API calls  

---

## Getting Started (Windows)

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable) installed and on your PATH  
- OpenAI API Key (sign up [here](https://platform.openai.com/signup))  

### Setup

1. **Clone the repository**  
   ```powershell
   git clone https://github.com/yourusername/ai-meme-generator.git
   cd ai-meme-generator
   ```

2. **Create `.env` file** in project root:  
   ```env
   OPENAI_API_KEY=your_openai_api_key_here
   ```

3. **Build and run the app**  
   ```powershell
   cargo run --release -- generate --template "drake" --use-ai
   ```

4. **Result**  
   Check the `/output` folder for your generated meme image (`PNG` format).

---

## Usage

### List available meme templates

```powershell
cargo run -- list-templates
```

### Generate meme with AI caption

```powershell
cargo run -- generate --template "distracted-boyfriend" --use-ai
```

### Generate meme with your own caption text

```powershell
cargo run -- generate --template "drake" --text "Coding all night like a pro"
```

### Help

```powershell
cargo run -- --help
```

---

## Project Structure

```
ai-meme-generator/
├── Cargo.toml
├── README.md
├── .env.example
├── assets/
│   ├── templates/         # Meme images (PNG/JPEG)
│   └── fonts/             # Fonts used for rendering text
├── output/                # Generated meme images saved here
├── src/
│   ├── main.rs            # CLI entry point
│   ├── templates.rs       # Template loader and structs
│   ├── rendering.rs       # Image & text rendering
│   ├── ai_client.rs       # OpenAI API integration
│   ├── cli.rs             # CLI commands & argument parsing
│   └── config.rs          # Config & env loading
└── tests/
    └── (unit and integration tests)
```

---

## Contributing

Feel free to open issues or submit pull requests!  
Please keep the project lightweight and easy to run locally.

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*Built with ❤️ and Rust*
