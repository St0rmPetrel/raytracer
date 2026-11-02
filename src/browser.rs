//! JavaScript bindings; scene validation and rendering stay in the shared library.
use crate::{config::Config, raytracer, render_rgba, render_rgba_quality};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render_scene(scene: &str, width: u32, height: u32) -> Result<Vec<u8>, JsValue> {
    render_rgba(scene, width as usize, height as usize).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn render_scene_quality(
    scene: &str,
    width: u32,
    height: u32,
    samples: u32,
) -> Result<Vec<u8>, JsValue> {
    render_rgba_quality(
        scene,
        width as usize,
        height as usize,
        Some(samples as usize),
    )
    .map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub struct BrowserRenderer {
    inner: raytracer::RowRenderer,
    height: usize,
}

#[wasm_bindgen]
impl BrowserRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        scene: &str,
        width: u32,
        height: u32,
        samples: u32,
    ) -> Result<BrowserRenderer, JsValue> {
        let cfg = Config::parse_for_render(
            scene,
            width as usize,
            height as usize,
            Some(samples as usize),
        )
        .map_err(|error| JsValue::from_str(&error))?;
        let inner = raytracer::Raytracer::new(cfg.camera, cfg.scene).into_rows(
            width as usize,
            height as usize,
            samples as usize,
        );
        Ok(Self {
            inner,
            height: height as usize,
        })
    }

    pub fn render_rows(&mut self, start: u32, rows: u32) -> Result<Vec<u8>, JsValue> {
        let start = start as usize;
        let rows = rows as usize;
        if rows == 0 || rows > 64 || start >= self.height || rows > self.height - start {
            return Err(JsValue::from_str("Invalid row range"));
        }
        Ok(self.inner.render_rows(start, start + rows))
    }
}
