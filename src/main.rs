use std::rc::Rc;

use camera::Camera;
use hittable::HittableList;
use sphere::Sphere;
use vec3::Point3;

mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // world
    let mut world = HittableList::default();

    world.add(Rc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.render(&world);
}
