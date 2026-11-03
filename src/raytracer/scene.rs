use crate::config::{ObjProperties, SceneConfig};
use crate::raytracer::radiance::Radiance;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector::Vector;

mod light;
mod shape;

use light::Light;
use shape::Shape;

struct Object {
    shape: Box<dyn Shape>,
    properties: Properties,
}

struct Properties {
    color: Radiance,
    checker: Option<Radiance>,
    checker_scale: f32,
    diffuse: Option<f32>,
    reflection: Option<f32>,
}

impl Properties {
    fn new(cfg: &ObjProperties) -> Properties {
        Properties {
            color: Radiance::from_rgb(cfg.color),
            checker: cfg.checker.map(Radiance::from_rgb),
            checker_scale: cfg.checker_scale.unwrap_or(1.0),
            diffuse: cfg.diffuse,
            reflection: cfg.reflection,
        }
    }
}

pub struct Scene {
    objects: Vec<Object>,
    lights: Vec<Light>,
    exposure: f32,
    ambient: f32,
    background: Radiance,
    horizon: Radiance,
}

impl Scene {
    pub fn new(cfg: SceneConfig) -> Scene {
        let mut scene = Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            exposure: cfg.exposure,
            ambient: cfg.ambient,
            background: Radiance::from_rgb(cfg.background),
            horizon: cfg.horizon.map(Radiance::from_rgb).unwrap_or_else(|| {
                Radiance::from_rgb(cfg.background).mix(Radiance::from_rgb([95, 130, 190]), 0.75)
            }),
        };
        // objects
        for s in cfg.spheres.iter() {
            let sphere = shape::new_sphere(Vector::new_from_arr(&s.center), s.radius);
            let prop = Properties::new(&s.properties);

            scene.push_object(sphere, prop)
        }

        // light
        for l in cfg.lights.iter() {
            let light = Light::new(
                Vector::new_from_arr(&l.origin),
                l.power,
                Radiance::from_rgb(l.color),
            );
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

    const MAX_REFLECTION_DEPTH: u8 = 5;

    pub fn exposure(&self) -> f32 {
        self.exposure
    }

    fn environment(&self, direction: &Vector) -> Radiance {
        let elevation = direction.dot(&Vector::new(0.0, 1.0, 0.0));
        self.horizon.mix(self.background, elevation.max(0.0).sqrt())
    }

    pub fn get_ray_color(&self, ray: &Ray, depth: u8) -> Radiance {
        let Some(hit) = self.intersec(ray) else {
            return self.environment(ray.get_dir());
        };
        let properties = hit.obj_properties;
        let normal = match properties.diffuse {
            Some(diff) if diff > 0.0 => (&hit.norm + &(&Vector::new_rand() * diff)).norm(),
            _ => hit.norm.clone(),
        };
        let mut albedo = properties.color;
        if let Some(checker) = properties.checker {
            let x = hit.point.dot(&Vector::new(1.0, 0.0, 0.0)) / properties.checker_scale;
            let z = hit.point.dot(&Vector::new(0.0, 0.0, 1.0)) / properties.checker_scale;
            if (x.floor() + z.floor()).rem_euclid(2.0) >= 1.0 {
                albedo = checker;
            }
        }
        let mut diffuse = self.environment(&hit.norm).tint(albedo).scale(self.ambient);
        let mut specular = Radiance::default();
        for light in &self.lights {
            let to_light = light.get_orig() - &hit.point;
            let distance_squared = to_light.dot2().max(0.01);
            let direction = to_light.norm();
            let cosine = normal.dot(&direction).max(0.0);
            if cosine <= 0.0 || self.is_shadow(light, &hit.point, &hit.norm) {
                continue;
            }
            let incoming = light.incoming(distance_squared);
            diffuse = diffuse.add(incoming.tint(albedo).scale(cosine));
            let half = (&direction - ray.get_dir()).norm();
            let gloss = 96.0 / (1.0 + properties.diffuse.unwrap_or(0.0) * 30.0);
            specular = specular.add(incoming.scale(normal.dot(&half).max(0.0).powf(gloss) * 0.45));
        }
        let reflection = properties.reflection.unwrap_or(0.0);
        let mut result = diffuse;
        if reflection > 0.0 && depth < Self::MAX_REFLECTION_DEPTH {
            if let Some(reflected) = ray.new_reflect(&hit.point.step_away(&hit.norm), &normal) {
                let cosine = (-ray.get_dir().dot(&normal)).clamp(0.0, 1.0);
                let fresnel = reflection + (1.0 - reflection) * (1.0 - cosine).powi(5);
                // Trace reflection once per hit, independent of the number of lights.
                result = result.mix(self.get_ray_color(&reflected, depth + 1), fresnel);
            }
        }
        result.add(specular)
    }

    fn is_shadow(&self, light: &Light, point: &Vector, normal: &Vector) -> bool {
        let offset = point.step_away(normal);
        let to_light = light.get_orig() - &offset;
        let distance = to_light.size();
        let ray = Ray::new(offset, to_light.norm());
        self.intersec_obj(&ray)
            .is_some_and(|hit| hit.distance < distance)
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
    obj: &'a Object,
    distance: f32,
}

impl Scene {
    fn intersec_obj(&self, ray: &Ray) -> Option<IntersecObj<'_>> {
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

    fn intersec(&self, ray: &Ray) -> Option<Intersection<'_>> {
        self.intersec_obj(ray)
            .map(|obj| Intersection::new(obj, ray))
    }
}
