mod vec3;


fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;
    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        dbg!("Scanlines remaining: ", image_height - j);
        for i in 0..image_width {
            let r = (i as f64)  / (image_width as f64 - 1.0);
            let g = (j as f64) / (image_height as f64 - 1.0);
            let b = 0.0;
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            println!("{} {} {}", ir, ig, ib);

        }
    }
    dbg!("Done.           \n");
}
