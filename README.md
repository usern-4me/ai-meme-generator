# 🖼️ Simple Text Renderer

A minimal Rust-based CLI tool for rendering custom text onto image templates — useful for posters, memes, or vote-worthy banners.

---

## 🚀 Features

- Add custom text to an image template.
- Supports automatic positioning and optional center alignment.
- Automatically wraps text to fit image width.
- Output saved as `output/Output.png`.

---

## 📦 Requirements

- Rust (edition 2021 or newer)

---

## 🛠️ Usage

```bash
cargo run -- "Text here" [font_size] [align_center] [x] [y] [r] [g] [b] [template_path] [font_path]
```

- `x`, `y` – optional text position (default: `50 50`)
- `r`, `g`, `b` – optional text color (default: white `255 255 255`)
- `align_center` - text align center (default: false (left))
- `font_size` - size of font (default: 64)
- `template_path`, `font_path` - paths to image and font

### Examples:

Render centered green text:
```bash
cargo run -- "Vote for me!" 72 false 100 200 0 255 0
```

Use defaults:
```bash
cargo run -- "Hello, world!"
```

---

## 🗂️ Project Structure

```
.
├── assets/
│   ├── fonts/
│   │   └── High Empathy.ttf
│   └── templates/
│       └── default.jpg
├── src/
│   ├── main.rs
│   └── render.rs
├── Cargo.toml
└── output/
    └── out_name.png
```

---

## 📄 License

MIT or similar — feel free to reuse, fork, or adapt.
