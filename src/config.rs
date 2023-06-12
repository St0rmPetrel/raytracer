use serde::Deserialize;
use std::fs::read_to_string;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub spheres: Vec<SphereConfig>,
    pub lights: Vec<LightConfig>,
}

impl Config {
    pub fn new(path: &str) -> Config {
        let toml_str = read_to_string(path).unwrap();
        let conf: Config = toml::from_str(&toml_str).unwrap();
        conf
    }
}

#[derive(Debug, Deserialize)]
pub struct SphereConfig {
    pub center: [f32; 3],
    pub radius: f32,
    pub color: [u8; 3],
}

#[derive(Debug, Deserialize)]
pub struct LightConfig {
    pub center: [f32; 3],
}
