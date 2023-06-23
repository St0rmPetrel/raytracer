use crate::config::{ObjProperties, SceneConfig};
use crate::image::Color;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector::Vector;

mod light;
mod shape;

use light::Light;
use shape::Shape;

pub struct Object {
    shape: Box<dyn Shape>,
    properties: Properties,
}

pub struct Properties {
    pub color: Color,
    pub diffuse: Option<f32>,
    pub reflection: Option<f32>,
}

impl Properties {
    fn new(cfg: &ObjProperties) -> Properties {
        Properties {
            color: Color::new_from_arr(&cfg.color),
            diffuse: cfg.diffuse,
            reflection: cfg.reflection,
        }
    }
}

pub struct Scene {
    objects: Vec<Object>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new(cfg: SceneConfig) -> Scene {
        let mut scene = Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        };
        // objects
        for s in cfg.spheres.iter() {
            let sphere = shape::new_sphere(Vector::new_from_arr(&s.center), s.radius);
            let prop = Properties::new(&s.properties);

            scene.push_object(sphere, prop)
        }

        // light
        for l in cfg.lights.iter() {
            let light = Light::new(Vector::new_from_arr(&l.origin));
            scene.push_light(light);
        }

        scene
    }

    fn push_object<T: Shape + 'static>(&mut self, shape: T, properties: Properties) {
        let obj = Object {
            properties,
            shape: Box::new(shape),
        };
        self.objects.push(obj)
    }
    fn push_light(&mut self, light: Light) {
        self.lights.push(light)
    }

    const REFLECT_DEEP: u8 = 5;
    const STEP_FROM_SHAPE: f32 = 0.0001;

    pub fn get_ray_color(&self, ray: &Ray, deep: u8) -> Color {
        let intersec = match self.intersec(ray) {
            None => return Color::new(0, 0, 0),
            Some(intersec) => intersec,
        };

        let norm = match intersec.obj_properties.diffuse {
            None => intersec.norm,
            Some(diff) => (&intersec.norm + &(&Vector::new_rand() * diff)).norm(),
        };

        let intensity: f32 = self
            .lights
            .iter()
            .map(|l| l.intensity(&intersec.point, &norm) / (self.lights.len() as f32))
            .sum();

        let color = intensity * intersec.obj_properties.color.clone();

        match intersec.obj_properties.reflection {
            None => color,
            Some(_) => {
                if deep >= Self::REFLECT_DEEP {
                    return color;
                }

                let point = &intersec.point + &(Self::STEP_FROM_SHAPE * &norm);
                let rfl_ray = match ray.new_reflect(&point, &norm) {
                    None => return color,
                    Some(r) => r,
                };
                // TODO better model of reflection
                //&((1.0 - rfl) * color) + &(rfl * self.get_ray_color(&rfl_ray, deep + 1))
                &color + &self.get_ray_color(&rfl_ray, deep + 1)
            }
        }
    }
}

struct Intersection<'a> {
    point: Vector,
    norm: Vector,
    obj_properties: &'a Properties,
}

impl Scene {
    fn intersec(&self, ray: &Ray) -> Option<Intersection> {
        struct IntersecObj<'a> {
            obj: &'a Object,
            distance: f32,
        }
        let mut nearest_obj = None;
        for obj in self.objects.iter() {
            let distance = match obj.shape.intersec(ray).get_closer() {
                Some(d) => d,
                None => continue,
            };

            nearest_obj = match nearest_obj {
                None => Some(IntersecObj { obj, distance }),
                Some(mut nearest_obj) => {
                    if distance < nearest_obj.distance {
                        nearest_obj.distance = distance;
                        nearest_obj.obj = obj;
                    }
                    Some(nearest_obj)
                }
            };
        }

        match nearest_obj {
            None => None,
            Some(obj) => {
                let point = ray.point_on_ray(obj.distance);
                let obj = obj.obj;
                Some(Intersection {
                    norm: obj.shape.norm(&point),
                    obj_properties: &obj.properties,
                    point,
                })
            }
        }
    }
}
