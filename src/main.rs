use clap::Parser;
use image::ImageFormat;
use std::path::Path;
use std::process;

#[derive(Parser)]
struct Cli {
    // 入力画像のパス
    input: String,
    // 出力画像のパス
    output: String,
}

fn main() {
    // CLIの引数を解析
    let args = Cli::parse();

    // 画像を読み込む
    let img = match image::open(&args.input) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // 出力先のパス
    let output_path = Path::new(&args.output);

    // 画像を保存
    if let Err(e) = img.save_with_format(&output_path, ImageFormat::Png) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    println!("Image saved to {}", output_path.display());
}