use super::light::Light;
use super::ray::Ray;
use super::shape::Shape;
use super::vector::Vector;

use crate::image::Color;

pub struct Object {
    shape: Box<dyn Shape>,
    properties: Properties,
}

pub struct Properties {
    pub color: Color,
}

pub struct Scene {
    objects: Vec<Object>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn push_object<T: Shape + 'static>(&mut self, shape: T, color: Color) {
        let obj = Object {
            properties: Properties { color },
            shape: Box::new(shape),
        };
        self.objects.push(obj)
    }
    pub fn push_light(&mut self, orig: Vector) {
        let light = Light::new(orig);
        self.lights.push(light)
    }

    pub fn get_ray_color(&self, ray: &Ray) -> Color {
        let intersec = match self.intersec(ray) {
            None => return Color::new(0, 0, 0),
            Some(intersec) => intersec,
        };
        self.illuminate(intersec)
    }
}

struct Intersection<'a> {
    point: Vector,
    norm: Vector,
    obj_properties: &'a Properties,
}

impl Scene {
    fn intersec(&self, ray: &Ray) -> Option<Intersection> {
        for obj in self.objects.iter() {
            let distance = match obj.shape.intersec(ray) {
                Some(distance) => distance,
                None => continue,
            };
            // TODO
            let point = ray.point_on_ray(distance);
            let properties = &obj.properties;
            return Some(Intersection {
                norm: obj.shape.norm(&point),
                obj_properties: properties,
                point,
            });
        }
        None
    }

    fn illuminate(&self, intersec: Intersection) -> Color {
        let mut color = Color::new(0, 0, 0);
        color.set(&intersec.obj_properties.color);

        let mut intensity = 0.0;
        for light in self.lights.iter() {
            intensity +=
                light.intensity(&intersec.point, &intersec.norm) / (self.lights.len() as f32);
        }
        color *= intensity;
        color
    }
}
