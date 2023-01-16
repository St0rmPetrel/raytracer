use super::light::Light;
use super::ray::Ray;
use super::shape::Shape;
use super::vector::Vector;

use crate::image;

pub struct Object {
    shape: Box<dyn Shape>,
    properties: Properties,
}

pub struct Properties {
    pub color: image::Color,
}

pub struct Intersection<'a> {
    pub point: Vector,
    pub norm: Vector,
    pub obj_properties: &'a Properties,
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

    pub fn push_object<T: Shape + 'static>(&mut self, shape: T, color: image::Color) {
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

    pub fn intersec(&self, ray: &Ray) -> Option<Intersection> {
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
}
