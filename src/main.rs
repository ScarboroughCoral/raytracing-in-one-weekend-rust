use color::{write_color, Color};
use vec3::Vec3;

mod color;
mod vec3;
mod ray;

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;
    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        dbg!("Scanlines remaining: ", image_height - j);
        for i in 0..image_width {
            let pixel_color: Color = Vec3::new(
                (i as f64) / (image_width as f64 - 1.0),
                (j as f64) / (image_height as f64 - 1.0),
                0.0,
            );
            write_color(&pixel_color);
        }
    }
    dbg!("Done.           \n");
}
