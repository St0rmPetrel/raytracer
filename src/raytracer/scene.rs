use crate::config::SceneConfig;
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
}

pub struct Scene {
    objects: Vec<Object>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new(cfg: &SceneConfig) -> Scene {
        let mut scene = Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        };
        // objects
        for s in cfg.spheres.iter() {
            let sphere = shape::new_sphere(Vector::new_from_arr(&s.center), s.radius);
            let prop = Properties {
                color: Color::new_from_arr(&s.color),
                diffuse: s.diffuse,
            };

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

    fn illuminate(&self, intersec: Intersection) -> Color {
        let mut color = Color::new(0, 0, 0);
        color.set(&intersec.obj_properties.color);

        let norm = match intersec.obj_properties.diffuse {
            None => intersec.norm,
            Some(diff) => (&intersec.norm + &(&Vector::new_rand() * diff)).norm(),
        };

        let mut intensity = 0.0;
        for light in self.lights.iter() {
            intensity += light.intensity(&intersec.point, &norm) / (self.lights.len() as f32);
        }
        color *= intensity;
        color
    }
}
