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

    fn get_rlf_ray(ray: &Ray, p: &Vector, n: &Vector, deep: u8) -> Option<Ray> {
        if deep >= Self::REFLECT_DEEP {
            return None;
        }

        // TODO make method step away
        let p = p + &(Self::STEP_FROM_SHAPE * n);
        ray.new_reflect(&p, n)
    }

    fn is_shadow(&self, l: &Light, p: &Vector, n: &Vector) -> bool {
        let pl = l.get_orig() - p;
        let pl_size = pl.size();

        let point = p + &(Self::STEP_FROM_SHAPE * n);
        let sh_ray = Ray::new(point, pl.norm());

        let distance = match self.intersec_obj(&sh_ray) {
            None => f32::MAX,
            Some(intersec) => intersec.distance,
        };

        distance <= pl_size
    }

    fn is_light(ray: &Ray, l: &Light) -> bool {
        let ol_dir = (l.get_orig() - ray.get_orig()).norm();
        let dot = ol_dir.dot(ray.get_dir());

        dot > (1.0 - Self::STEP_FROM_SHAPE)
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
        let rfl_handler = |c: Color| match intersec.obj_properties.reflection {
            None => c,
            Some(rfl) => {
                let rfl_ray = match Self::get_rlf_ray(ray, &intersec.point, &norm, deep) {
                    None => return c,
                    Some(r) => r,
                };
                c.add_refl(rfl, self.get_ray_color(&rfl_ray, deep + 1))
            }
        };

        // shadow
        if self.is_shadow(l, &intersec.point, &norm) {
            return rfl_handler(Color::new(0, 0, 0));
        }
        // make light visible
        if Self::is_light(ray, l) {
            return Color::new(255, 255, 255);
        }

        let dist_from_l = (&intersec.point - l.get_orig()).size();
        let l_intens = l.intensity(&intersec.point, &norm) / (dist_from_l);

        rfl_handler(l_intens * intersec.obj_properties.color.clone())
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
