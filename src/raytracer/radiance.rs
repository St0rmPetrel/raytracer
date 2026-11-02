use crate::image::Color;

/// Linear, unclipped light. Quantize only after sampling and tone mapping.
#[derive(Clone, Copy, Debug, Default)]
pub struct Radiance([f32; 3]);

impl Radiance {
    pub fn from_rgb(rgb: [u8; 3]) -> Self {
        Self(rgb.map(|v| (v as f32 / 255.0).powf(2.2)))
    }
    pub fn add(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] + other.0[i]))
    }
    pub fn scale(self, factor: f32) -> Self {
        Self(self.0.map(|v| v * factor))
    }
    pub fn tint(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] * other.0[i]))
    }
    pub fn mix(self, other: Self, fraction: f32) -> Self {
        self.scale(1.0 - fraction).add(other.scale(fraction))
    }
    pub fn display(self, exposure: f32) -> Color {
        let rgb = self.0.map(|v| {
            let linear = (v * exposure).max(0.0);
            let mapped = linear / (1.0 + linear);
            (mapped.powf(1.0 / 2.2) * 255.0).round() as u8
        });
        Color::new_from_arr(&rgb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_low_light_and_highlights_until_display() {
        let dim = Radiance([0.002; 3]).display(1.0);
        assert!(dim.r > 0);
        let bright = Radiance([2.0; 3]).display(1.0);
        let brighter = Radiance([10.0; 3]).display(1.0);
        assert!(bright.r < brighter.r && brighter.r < 255);
        assert_eq!(Radiance::default().display(1.0).r, 0);
    }
}
