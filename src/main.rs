use image::{GenericImageView, DynamicImage, Rgba, GenericImage};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input_image: PathBuf,

    #[structopt(parse(from_os_str))]
    color_file: PathBuf,

    #[structopt(parse(from_os_str))]
    output_image: PathBuf,
}

// Function to convert hex color to RGB
fn hex_to_rgb(hex_color: &str) -> [u8; 3] {
    [
        u8::from_str_radix(&hex_color[0..2], 16).unwrap(),
        u8::from_str_radix(&hex_color[2..4], 16).unwrap(),
        u8::from_str_radix(&hex_color[4..6], 16).unwrap(),
    ]
}

// Function to find the closest color in the color list
fn closest_color(target_color: [u8; 4], color_list: &Vec<[u8; 4]>) -> [u8; 4] {
    let closest_idx = color_list
        .iter()
        .position(|&color| color == *color_list.iter().min_by_key(|&c| {
            let squared_distance = ((c[0] as i32 - target_color[0] as i32).pow(2)
                + (c[1] as i32 - target_color[1] as i32).pow(2)
                + (c[2] as i32 - target_color[2] as i32).pow(2)
                + (c[3] as i32 - target_color[3] as i32).pow(2)) as f64;
            squared_distance.sqrt() as u8
        }).unwrap())
        .unwrap();
    color_list[closest_idx]
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let opt = Opt::from_args();

    // Configure logging (not as sophisticated as Python's logging module)
    println!("Loading color data from hex file...");

    // Read color data from hex file
    let file = File::open(&opt.color_file)?;
    let reader = io::BufReader::new(file);

    // Convert hex values to RGBA
    let color_list: Vec<[u8; 4]> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let rgb = hex_to_rgb(&line);
            [rgb[0], rgb[1], rgb[2], 255] // Add alpha channel (255)
        })
        .collect();

    // Load input image
    let input_image = image::open(&opt.input_image)?;

    // Process image
    println!("Processing image...");

    // Find the closest colors for all pixels in the input image
    let closest_colors: Vec<[u8; 4]> = input_image
        .pixels()
        .map(|(_, _, pixel)| {
            let rgba = pixel.0;
            closest_color(rgba, &color_list)
        })
        .collect();

    // Reshape the resulting array back to the shape of the input image
    let width = input_image.width() as usize;
    let height = input_image.height() as usize;
    let closest_colors = closest_colors.chunks_exact(width).take(height);

    // Create an output image from the closest colors
    let mut output_image = DynamicImage::new_rgba8(width as u32, height as u32);
    for (y, row) in closest_colors.enumerate() {
        for (x, color) in row.iter().enumerate() {
            output_image.put_pixel(x as u32, y as u32, Rgba(*color));
        }
    }

    // Save the output image
    output_image.save(&opt.output_image)?;

    println!(
        "Image processing complete. Output saved as '{:?}'",
        opt.output_image
    );
    Ok(())
}
