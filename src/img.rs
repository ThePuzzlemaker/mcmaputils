use std::process;

use image::ImageFormat;

pub fn detect(file: &str) -> ImageFormat {
    match file.split('.').last().unwrap_or("none") {
        "png" => ImageFormat::Png,
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "bmp" => ImageFormat::Bmp,
        _ => {
            eprintln!(
                "error: could not detect image format from file name: `{}`",
                file
            );
            process::exit(1)
        }
    }
}
