use env_logger::Env;
use log::info;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let image_width: i32 = 256;
    let image_height: i32 = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        info!("/rScanlines remaining: {} ", (image_height - j));

        for i in 0..image_width {
            let r: f64 = (i as f64) / ((image_width - 1) as f64);
            let g: f64 = (j as f64) / ((image_height - 1) as f64);
            let b: f64 = 0.0;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib)
        }
    }

    info!("\rDone.");
}
