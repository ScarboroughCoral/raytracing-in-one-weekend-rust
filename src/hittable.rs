use std::rc::Rc;

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn get_face_normal(r: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        let front_face = r.direction().dot_with(outward_normal) < 0.;
        let normal = if front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
        return (front_face, normal);
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;
        self.objects.iter().for_each(|object| {
            if let Some(hit_record) = object.hit(r, ray_tmin, closest_so_far) {
                hit_anything = true;
                closest_so_far = hit_record.t;
                temp_rec = hit_record
            }
        });
        return if hit_anything { Some(temp_rec)} else { None };
    }
}
