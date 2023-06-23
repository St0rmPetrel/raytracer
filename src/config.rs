use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub image: ImageConfig,
    pub scene: SceneConfig,
    pub camera: CameraConfig,
}

impl Config {
    pub fn parse(toml_str: &str) -> Result<Config, toml::de::Error> {
        toml::from_str(toml_str)
    }
}

#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    pub origin: [f32; 3],
    pub view: [f32; 3],
    pub up: [f32; 3],
}

#[derive(Debug, Deserialize)]
pub struct ImageConfig {
    pub name: String,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Deserialize)]
pub struct SceneConfig {
    pub spheres: Vec<SphereConfig>,
    pub lights: Vec<LightConfig>,
}

#[derive(Debug, Deserialize)]
pub struct SphereConfig {
    pub center: [f32; 3],
    pub radius: f32,
    pub properies: ObjProperties,
}

#[derive(Debug, Deserialize)]
pub struct ObjProperties {
    pub color: [u8; 3],
    pub diffuse: Option<f32>,
    pub reflection: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct LightConfig {
    pub origin: [f32; 3],
}
