use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub image: ImageConfig,
    pub scene: SceneConfig,
    pub camera: CameraConfig,
}

impl Config {
    pub fn parse(toml_str: String) -> Result<Config, toml::de::Error> {
        toml::from_str(&toml_str)
    }

    /// Parse untrusted renderer input and apply request dimensions and sampling.
    pub(crate) fn parse_for_render(
        scene: &str,
        width: usize,
        height: usize,
        samples: Option<usize>,
    ) -> Result<Self, String> {
        if width == 0 || height == 0 || width > 1024 || height > 1024 {
            return Err("Resolution must be between 1 and 1024 pixels per side".into());
        }
        if scene.len() > 64 * 1024 {
            return Err("Scene is too large".into());
        }
        let mut cfg = Self::parse(scene.to_owned()).map_err(|e| e.to_string())?;
        if let Some(samples) = samples {
            cfg.image.samples = samples;
        }
        if ![1, 4, 16].contains(&cfg.image.samples)
            || !cfg.scene.exposure.is_finite()
            || !(0.1..=8.0).contains(&cfg.scene.exposure)
            || !cfg.scene.ambient.is_finite()
            || !(0.0..=2.0).contains(&cfg.scene.ambient)
        {
            return Err("Invalid sample count, exposure or ambient light".into());
        }
        if cfg.scene.spheres.len() > 32 || cfg.scene.lights.len() > 8 {
            return Err("Use at most 32 spheres and 8 lights".into());
        }
        if !valid_camera(&cfg.camera)
            || !cfg.scene.lights.iter().all(valid_light)
            || !cfg.scene.spheres.iter().all(valid_sphere)
        {
            return Err("Invalid camera, geometry or material values".into());
        }
        cfg.image.width = width;
        cfg.image.height = height;
        Ok(cfg)
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
    #[serde(default = "default_samples")]
    pub samples: usize,
}

fn default_samples() -> usize {
    4
}
fn default_exposure() -> f32 {
    1.5
}
fn default_ambient() -> f32 {
    0.25
}
fn default_background() -> [u8; 3] {
    [45, 65, 110]
}
fn default_power() -> f32 {
    60.0
}
fn default_white() -> [u8; 3] {
    [255, 255, 255]
}

#[derive(Debug, Deserialize)]
pub struct SceneConfig {
    pub spheres: Vec<SphereConfig>,
    pub lights: Vec<LightConfig>,
    #[serde(default = "default_exposure")]
    pub exposure: f32,
    #[serde(default = "default_ambient")]
    pub ambient: f32,
    #[serde(default = "default_background")]
    pub background: [u8; 3],
    pub horizon: Option<[u8; 3]>,
}

#[derive(Debug, Deserialize)]
pub struct SphereConfig {
    pub center: [f32; 3],
    pub radius: f32,
    pub properties: ObjProperties,
}

#[derive(Debug, Deserialize)]
pub struct ObjProperties {
    pub color: [u8; 3],
    pub diffuse: Option<f32>,
    pub reflection: Option<f32>,
    pub checker: Option<[u8; 3]>,
    pub checker_scale: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct LightConfig {
    pub origin: [f32; 3],
    #[serde(default = "default_power")]
    pub power: f32,
    #[serde(default = "default_white")]
    pub color: [u8; 3],
}

fn finite_vector(vector: &[f32; 3]) -> bool {
    vector.iter().all(|value| value.is_finite())
}

fn valid_camera(camera: &CameraConfig) -> bool {
    let view = camera.view;
    let up = camera.up;
    let cross = [
        view[1] * up[2] - view[2] * up[1],
        view[2] * up[0] - view[0] * up[2],
        view[0] * up[1] - view[1] * up[0],
    ];
    finite_vector(&camera.origin)
        && finite_vector(&view)
        && finite_vector(&up)
        && cross
            .iter()
            .map(|value| value * value)
            .sum::<f32>()
            .is_normal()
}

fn valid_light(light: &LightConfig) -> bool {
    finite_vector(&light.origin) && light.power.is_finite() && (0.0..=1000.0).contains(&light.power)
}

fn valid_sphere(sphere: &SphereConfig) -> bool {
    let material = &sphere.properties;
    finite_vector(&sphere.center)
        && sphere.radius.is_finite()
        && sphere.radius > 0.0
        && material
            .checker_scale
            .is_none_or(|value| value.is_finite() && (0.1..=100.0).contains(&value))
        && material
            .reflection
            .is_none_or(|value| value.is_finite() && (0.0..=1.0).contains(&value))
        && material
            .diffuse
            .is_none_or(|value| value.is_finite() && (0.0..=1.0).contains(&value))
}
