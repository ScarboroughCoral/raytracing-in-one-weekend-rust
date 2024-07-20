use color::{write_color, Color};
use ray::Ray;
use vec3::{Point3, Vec3};

mod color;
mod vec3;
mod ray;

fn ray_color(r: &Ray) -> Color {
    let direction = r.direction().unit_vector();
    let a = 0.5 * (direction.y() + 1.0);
    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}
fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width as f64 / ASPECT_RATIO) as i32;
    let image_height = if (image_height < 1) { 1 } else { image_height };

    // Camera
    const FOCAL_LENGTH: f64 = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Point3 = Vec3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_location = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        dbg!("Scanlines remaining: ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_location + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            write_color(&pixel_color);
        }
    }
    dbg!("Done.           \n");
}
