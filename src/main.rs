use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;
use which::which;

fn main() {
    let input_folder = "./videos";
    let output_folder = "./gifs";

    if which("ffmpeg").is_err() {
        eprintln!("❌ ffmpeg not found in PATH. Install it first.");
        return;
    }

    std::fs::create_dir_all(output_folder).unwrap();
    let allowed_exts = ["mp4", "mov", "avi", "mkv"];

    for entry in WalkDir::new(input_folder)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        println!("Found file: {}", path.display());
        let file_name = path.file_stem().unwrap().to_string_lossy();
        let extension = path.extension().unwrap_or_default().to_string_lossy();

        if allowed_exts.contains(&extension.to_lowercase().as_str()) {
            let output_path = format!("{}/{}.gif", output_folder, file_name);
            let palette_path = format!("{}/{}_palette.png", output_folder, file_name);
            println!("🎬 Converting {} -> {}", path.display(), output_path);

            // Step 1: Generate palette
            let palette_status = Command::new("ffmpeg")
                .args(&[
                    "-y",
                    "-i",
                    path.to_str().unwrap(),
                    "-vf",
                    "fps=10,scale=1024:-1:flags=lanczos,palettegen",
                    &palette_path,
                ])
                .status()
                .expect("failed to generate palette");

            if !palette_status.success() {
                eprintln!("❌ Failed to generate palette for {}", path.display());
                continue;
            }

            // Step 2: Create optimized GIF using palette
            let gif_status = Command::new("ffmpeg")
                .args(&[
                    "-y",
                    "-i",
                    path.to_str().unwrap(),
                    "-i",
                    &palette_path,
                    "-filter_complex",
                    "fps=10,scale=1024:-1:flags=lanczos[x];[x][1:v]paletteuse",
                    &output_path,
                ])
                .status()
                .expect("failed to create gif");

            let palette_path = Path::new(&palette_path);
            if palette_path.exists() {
                if let Err(e) = std::fs::remove_file(palette_path) {
                    eprintln!("⚠️ Failed to delete palette.png: {}", e);
                } else {
                    println!("🧹 Deleted palette.png");
                }
            }
            if gif_status.success() {
                println!("✅ Saved optimized GIF: {}", output_path);
            } else {
                eprintln!("⚠️ Failed to create optimized GIF: {}", path.display());
            }
        }
    }
}
