use core::f32;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;
mod vec3;

fn main() -> std::io::Result<()> {
    // File creation
    let mut render = File::create("image.ppm")?;

    //Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    //Render

    render.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {} ", j);
        stdout().flush()?;
        for i in 0..image_width {
            let r_frac = (i as f32) / ((image_width - 1) as f32);
            let g_frac = (j as f32) / ((image_height - 1) as f32);
            let b_frac = 0.25 as f32;

            let r = (255.999 * r_frac) as i32;
            let g = (255.999 * g_frac) as i32;
            let b = (255.999 * b_frac) as i32;

            let line_to_write = format!("{} {} {}\n", r, g, b);
            render.write_all(line_to_write.as_bytes())?;
        }
    }
    println!("\nDone!");
    Ok(())
}
