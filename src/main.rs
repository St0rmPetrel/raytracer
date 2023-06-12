pub mod canvas;
pub mod config;
pub mod image;
pub mod ray;
pub mod scene;
pub mod vector;

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
    let scene = scene::Scene::new(&cfg.scene);

    const RESOLUTION: usize = 1280 + 1;

    const HD_W: usize = 1280;
    const HD_H: usize = 720;

    let mut convas = canvas::Canvas::new(15.0, RESOLUTION);

    convas.fill_canvas(scene, (0, 1281), (280, 1001));

    for j in 0..HD_H {
        for i in 0..HD_W {
            let pixel = image.get_pixel(i, j).expect("pixel in image not found");
            let color_ul = convas
                .get_canvas_pixel(i, j + 280)
                .expect("pixel in convas not found");
            let color_ur = convas
                .get_canvas_pixel(i + 1, j + 280)
                .expect("pixel in convas not found");
            let color_bl = convas
                .get_canvas_pixel(i, j + 281)
                .expect("pixel in convas not found");
            let color_br = convas
                .get_canvas_pixel(i + 1, j + 281)
                .expect("pixel in convas not found");
            let color_u = color_ul + color_ur;
            let color_b = color_bl + color_br;
            let color = &color_b + &color_u;
            pixel.set(&color);
        }
    }

    image.save_ppm().expect("can't save ruster image");

    ExitCode::SUCCESS
}
