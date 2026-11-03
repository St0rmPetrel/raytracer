//! Shared native/browser renderer. The browser API never reads or writes files.
pub mod config;
pub mod image;
pub mod raytracer;

#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(target_arch = "wasm32")]
pub use browser::{render_scene, render_scene_quality, BrowserRenderer};

/// Render a TOML scene to RGBA. Bounds limit memory and work on mobile devices.
/// Diffusion is stochastic, matching the native renderer.
pub fn render_rgba(scene: &str, width: usize, height: usize) -> Result<Vec<u8>, String> {
    render_rgba_quality(scene, width, height, None)
}

pub fn render_rgba_quality(
    scene: &str,
    width: usize,
    height: usize,
    samples: Option<usize>,
) -> Result<Vec<u8>, String> {
    let cfg = config::Config::parse_for_render(scene, width, height, samples)?;
    let mut image = image::RasterImage::new(cfg.image);
    raytracer::Raytracer::new(cfg.camera, cfg.scene).fill_image(&mut image);
    Ok(image.rgba())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SCENE: &str = include_str!("../config/reflect.toml");
    const LIT_SPHERE: &str = r#"
[image]
name = "test"
width = 32
height = 32
[camera]
origin = [0, 0, 5]
view = [0, 0, -1]
up = [0, 1, 0]
[scene]
ambient = 0
exposure = 1.5
[[scene.lights]]
origin = [0, 3, 5]
power = 80
[[scene.spheres]]
center = [0, 0, 0]
radius = 1
[scene.spheres.properties]
color = [255, 255, 255]
reflection = 0
diffuse = 0
"#;

    #[test]
    fn light_power_and_exposure_brighten_the_lit_surface() {
        let center = (16 * 32 + 16) * 4;
        let lit = render_rgba(LIT_SPHERE, 32, 32).unwrap();
        let unlit = render_rgba(&LIT_SPHERE.replace("power = 80", "power = 0"), 32, 32).unwrap();
        let low_exposure = render_rgba(
            &LIT_SPHERE.replace("exposure = 1.5", "exposure = 0.3"),
            32,
            32,
        )
        .unwrap();
        assert!(lit[center] > 150);
        assert_eq!(unlit[center], 0);
        assert!(lit[center] > low_exposure[center]);
    }

    #[test]
    fn supersampling_preserves_surface_brightness_and_changes_edges() {
        let draft = render_rgba_quality(LIT_SPHERE, 32, 32, Some(1)).unwrap();
        let detailed = render_rgba_quality(LIT_SPHERE, 32, 32, Some(16)).unwrap();
        let center = (16 * 32 + 16) * 4;
        assert!(draft[center].abs_diff(detailed[center]) <= 3);
        assert_ne!(draft, detailed);
        for samples in [0, 2, 17, usize::MAX] {
            assert!(render_rgba_quality(LIT_SPHERE, 32, 32, Some(samples)).is_err());
        }
    }

    #[test]
    fn an_occluder_still_casts_a_shadow_with_brighter_lighting() {
        let blocked = format!(
            r#"{LIT_SPHERE}
[[scene.spheres]]
center = [0, 1.5, 3]
radius = 0.45
[scene.spheres.properties]
color = [255, 255, 255]
reflection = 0
"#
        );
        let center = (16 * 32 + 16) * 4;
        assert_eq!(render_rgba(&blocked, 32, 32).unwrap()[center], 0);
    }

    #[test]
    fn rejects_invalid_lighting_before_rendering() {
        for (from, to) in [
            ("power = 80", "power = nan"),
            ("power = 80", "power = -1"),
            ("exposure = 1.5", "exposure = inf"),
            ("ambient = 0", "ambient = -1"),
        ] {
            assert!(render_rgba(&LIT_SPHERE.replace(from, to), 32, 32).is_err());
        }
    }

    #[test]
    fn incremental_rows_match_full_render_for_square_and_rectangular_images() {
        for (width, height) in [(32, 32), (12, 8), (7, 11)] {
            let cfg = config::Config::parse_for_render(LIT_SPHERE, width, height, Some(4)).unwrap();
            let mut renderer =
                raytracer::Raytracer::new(cfg.camera, cfg.scene).into_rows(width, height, 4);
            let mut strips = Vec::new();
            for start in (0..height).step_by(3) {
                strips.extend(renderer.render_rows(start, (start + 3).min(height)));
            }
            assert_eq!(
                strips,
                render_rgba_quality(LIT_SPHERE, width, height, Some(4)).unwrap()
            );
        }
    }
    #[test]
    fn produces_rgba_for_square_and_rectangular_images() {
        for (w, h) in [(1, 1), (16, 16), (12, 8), (7, 11)] {
            let bytes = render_rgba(SCENE, w, h).unwrap();
            assert_eq!(bytes.len(), w * h * 4);
            assert!(bytes.as_chunks::<4>().0.iter().all(|p| p[3] == 255));
        }
    }
    #[test]
    fn rejects_invalid_input_before_allocating_image() {
        for (w, h) in [(0, 16), (16, 0), (1025, 16), (usize::MAX, 16)] {
            assert!(render_rgba(SCENE, w, h).is_err());
        }
        assert!(render_rgba("not a scene", 16, 16).is_err());
        assert!(render_rgba(&"x".repeat(65537), 16, 16).is_err());
    }
}
