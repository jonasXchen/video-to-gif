# 🎞️ Video to GIF Converter (Rust + FFmpeg)

This project recursively converts video files from a folder into optimized animated GIFs using [FFmpeg](https://ffmpeg.org/) and Rust.

## 📦 Features

- Supports common video formats: `.mp4`, `.mov`, `.avi`, `.mkv`
- Generates high-quality, color-optimized GIFs
- Automatically creates and deletes color palettes (`palette.png`) for better compression
- Outputs all GIFs into a specified folder
- Respects original video resolution (up to a configured scale)

## 🛠️ Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [FFmpeg](https://ffmpeg.org/) installed and available in your system `PATH`

## 📁 Folder Structure

```
project/
├── videos/         # Input videos go here
├── gifs/           # Output GIFs will be saved here
├── src/
│   └── main.rs     # Rust source code
├── Cargo.toml
└── README.md
```

## 🚀 Usage

1. **Place your videos** into the `./videos` folder.
2. **Build and run** the project:

```bash
cargo run
```

3. **GIFs will appear** in the `./gifs` folder.

## 🧪 Example Output

A `video.mp4` in `./videos` will be converted to:

```
gifs/video.gif
```

## 🧹 Clean-up

Intermediate `*_palette.png` files are automatically deleted after each GIF is created.

## 🧩 Customization

You can adjust the GIF scale, FPS, or other FFmpeg filters inside the `main.rs` file:

```rust
"fps=20,scale=1024:-1:flags=lanczos,palettegen" // For palette
"fps=20,scale=1024:-1:flags=lanczos[x];[x][1:v]paletteuse" // For gif generation
```

## 🔒 License

This project is open source under the [MIT License](LICENSE).
