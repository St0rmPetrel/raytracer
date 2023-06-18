pub mod config;
pub mod image;
pub mod raytracer;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage: raytracer <path_to_toml_config>");
        return ExitCode::FAILURE;
    }

    let config_path = &args[1];
    let config_str = match std::fs::read_to_string(config_path) {
        Ok(str) => str,
        Err(err) => {
            println!("fail to read file {}: {}", config_path, err.to_string());
            return ExitCode::FAILURE;
        }
    };

    let cfg = match config::Config::parse(&config_str) {
        Ok(cfg) => cfg,
        Err(err) => {
            println!("fail to parse file {}: {}", config_path, err.message());
            return ExitCode::FAILURE;
        }
    };

    let mut image = image::RasterImage::new(&cfg.image);

    raytracer::Raytracer::new(&cfg.camera, &cfg.scene).fill_image(&mut image);

    match image.save_ppm() {
        Ok(_) => {
            println!("ppm image successful save");
        }
        Err(err) => {
            println!("fail to read file {}: {}", config_path, err.to_string());
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
