use clap::Parser;
use image::ImageFormat;
use std::path::Path;
use std::process;

#[derive(Parser)]
struct Cli {
    input: String,
    output: String,
}

fn main() {
    let args = Cli::parse();

    let img = match image::open(&args.input) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    let output_path = Path::new(&args.output);

    if let Err(e) = img.save_with_format(&output_path, ImageFormat::Png) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    println!("Image saved to {}", output_path.display());
}