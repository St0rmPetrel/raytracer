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
    const STEP_FROM_SHAPE: f32 = 100.0 * f32::EPSILON;

    pub fn get_ray_color(&self, ray: &Ray, deep: u8) -> Color {
        let mut c = Color::new(0, 0, 0);
        for l in self.lights.iter() {
            let ray_color_by_l = self.get_ray_color_by_light(ray, &l, deep);
            c = &c + &ray_color_by_l;
        }
        c
    }

    fn get_ray_color_by_light(&self, ray: &Ray, l: &Light, deep: u8) -> Color {
        let intersec = match self.intersec(ray) {
            None => return Color::new(0, 0, 0),
            Some(intersec) => intersec,
        };
        let norm = match intersec.obj_properties.diffuse {
            None => intersec.norm,
            Some(diff) => (&intersec.norm + &(&Vector::new_rand() * diff)).norm(),
        };

        // shadow
        {
            let pl = l.get_orig() - &intersec.point;
            let pl_size = pl.size();

            let point = &intersec.point + &(Self::STEP_FROM_SHAPE * &norm);
            let sh_ray = Ray::new(point, pl.norm());

            let distance = match self.intersec_obj(&sh_ray) {
                None => f32::MAX,
                Some(intersec) => intersec.distance,
            };
            if distance <= pl_size {
                return match intersec.obj_properties.reflection {
                    None => Color::new(0, 0, 0),
                    Some(rfl) => {
                        if deep >= Self::REFLECT_DEEP {
                            return Color::new(0, 0, 0);
                        }

                        let point = &intersec.point + &(Self::STEP_FROM_SHAPE * &norm);
                        let rfl_ray = match ray.new_reflect(&point, &norm) {
                            None => return Color::new(0, 0, 0),
                            Some(r) => r,
                        };
                        rfl * self.get_ray_color(&rfl_ray, deep + 1)
                    }
                };
            }
        }
        // make light visible
        {
            let sh_dir = l.get_orig() - ray.get_orig();
            // TODO check 1.0/size
            let sh_dir = sh_dir.norm();
            let sh_dot = sh_dir.dot(ray.get_dir());
            if sh_dot > (1.0 - Self::STEP_FROM_SHAPE) {
                return Color::new(255, 255, 255) * sh_dot;
            }
        }

        let dist_from_l = (&intersec.point - l.get_orig()).size().sqrt();
        let l_intens = l.intensity(&intersec.point, &norm) / (dist_from_l);

        let color = l_intens * intersec.obj_properties.color.clone();

        match intersec.obj_properties.reflection {
            None => color,
            Some(rfl) => {
                if deep >= Self::REFLECT_DEEP {
                    return color;
                }

                let point = &intersec.point + &(Self::STEP_FROM_SHAPE * &norm);
                let rfl_ray = match ray.new_reflect(&point, &norm) {
                    None => return color,
                    Some(r) => r,
                };
                &color + &(rfl * self.get_ray_color(&rfl_ray, deep + 1))
            }
        }
    }
}

struct Intersection<'a> {
    point: Vector,
    norm: Vector,
    obj_properties: &'a Properties,
}

impl<'a> Intersection<'a> {
    fn new(obj: IntersecObj<'a>, ray: &Ray) -> Intersection<'a> {
        let point = ray.point_on_ray(obj.distance);
        let obj = obj.obj;
        Intersection {
            norm: obj.shape.norm(&point),
            obj_properties: &obj.properties,
            point,
        }
    }
}

struct IntersecObj<'a> {
    pub obj: &'a Object,
    pub distance: f32,
}

impl Scene {
    fn intersec_obj(&self, ray: &Ray) -> Option<IntersecObj> {
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
        nearest_obj
    }

    fn intersec(&self, ray: &Ray) -> Option<Intersection> {
        match self.intersec_obj(ray) {
            None => None,
            Some(obj) => Some(Intersection::new(obj, ray)),
        }
    }
}
