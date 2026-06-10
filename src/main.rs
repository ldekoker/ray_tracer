use env_logger::Env;
use log::info;
use ray_tracer::{
    colour::{Colour, write_colour},
    ray::Ray,
    vec3::{Point3, Vec3},
};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let base_height = (image_width as f64 / aspect_ratio) as u32;
    let image_height = if base_height < 1 { 1 } else { base_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        info!("/rScanlines remaining: {} ", (image_height - j));

        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_colour = ray_colour(&ray);
            write_colour(pixel_colour);
        }
    }

    info!("\rDone.");
}

fn ray_colour(ray: &Ray) -> Colour {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Colour::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Colour::new(1.0, 1.0, 1.0) * (1.0 - a) + Colour::new(0.5, 0.7, 1.0) * a
}

/// The Ray has hit the sphere if the discriminant (distance to center)
/// is greater than or equal to 0 (i.e there are one or more solutions)
fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    // oc = (C - Q)
    let oc = *center - ray.origin();

    // a = d ⋅ d
    let a = ray.direction().dot(ray.direction());
    // b = -2d ⋅ (C - Q)
    let b = -2.0 * ray.direction().dot(oc);
    // c = (C - Q) ⋅ (C - Q) - r^2
    let c = oc.dot(oc) - (radius * radius);

    // b^2 - 4ac
    let discriminant = (b * b) - (4.0 * a * c);

    discriminant >= 0.0
}
