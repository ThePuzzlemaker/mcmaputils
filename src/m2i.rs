use std::fs::File;
use std::path::Path;
use std::process;

use image::imageops::FilterType;
use image::{imageops, ColorType, ImageFormat, Rgba, RgbaImage};

use clap::ArgMatches;

use crate::colors;
use crate::map_data::MapRoot;

pub fn map2img(matches: &ArgMatches) {
    let map_file_path = matches.value_of("map_#.dat").unwrap();
    let output_file_path = matches.value_of("output").unwrap();
    let format = matches.value_of("format").unwrap();
    let scale_factor = matches.value_of("scale").unwrap();

    let scale_factor = match scale_factor.parse::<u32>() {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "error: failed to parse scaling factor `{}` as an integer: {}",
                scale_factor, e
            );
            process::exit(1);
        }
    };

    if scale_factor == 0 {
        eprintln!("error: scaling factor cannot be 0");
        process::exit(1);
    }

    let format = match format {
        "png" => ImageFormat::Png,
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "bmp" => ImageFormat::Bmp,
        "detect" => crate::img::detect(output_file_path),
        _ => unreachable!(),
    };

    let map_file_path = Path::new(map_file_path);
    if !map_file_path.exists() {
        eprintln!("error: file does not exist: `{}`", map_file_path.display());
        process::exit(1);
    }

    let map_file = match File::open(map_file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!(
                "error: could not open file `{}`: {}",
                map_file_path.display(),
                e
            );
            process::exit(1)
        }
    };

    let map_data: MapRoot = match nbt::from_gzip_reader(map_file) {
        Ok(data) => data,
        Err(e) => {
            eprintln!(
                "error: failed to parse file `{}`: {}",
                map_file_path.display(),
                e
            );
            process::exit(1)
        }
    };

    let pixels: Vec<Rgba<u8>> = map_data
        .data
        .colors
        .into_iter()
        .map(|x| x as u8)
        .map(colors::to_rgba_full)
        .collect();

    let mut img = RgbaImage::new(128, 128);
    img.enumerate_pixels_mut()
        .for_each(|(x, y, pix)| *pix = pixels[(x + y * 128) as usize]);

    if scale_factor != 1 {
        img = imageops::resize(
            &img,
            128 * scale_factor,
            128 * scale_factor,
            FilterType::Nearest,
        );
    }

    match image::save_buffer_with_format(
        output_file_path,
        img.as_raw(),
        128 * scale_factor,
        128 * scale_factor,
        ColorType::Rgba8,
        format,
    ) {
        Ok(_) => (),
        Err(e) => {
            eprintln!(
                "error: failed to save output image `{}`: {}",
                output_file_path, e
            );
            process::exit(1)
        }
    };
}
