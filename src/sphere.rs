use crate::{
    hittable::{HitRecord, Hittable}, interval::Interval, vec3::Point3, Ray
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot_with(&oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let hit_point = r.at(root);
        let (front_face, normal) = HitRecord::get_face_normal(r, &((hit_point - self.center) / self.radius));
        return Some(HitRecord {
            t: root,
            p: hit_point,
            normal,
            front_face
        });
    }
}
