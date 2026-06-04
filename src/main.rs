use env_logger::Env;
use log::info;
use ray_tracer::{
    colour::{Colour, write_colour},
    vec3::Vec3,
};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let image_width: i32 = 256;
    let image_height: i32 = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        info!("/rScanlines remaining: {} ", (image_height - j));

        for i in 0..image_width {
            let pixel_colour = Colour::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64),
                0.0,
            );
            write_colour(&pixel_colour);
        }
    }

    info!("\rDone.");
}
