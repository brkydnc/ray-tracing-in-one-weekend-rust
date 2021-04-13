use std::fs::File;
use std::io::Write;
use lib::{
    vec3::{Vec3, Color},
    world::World,
    camera::Camera,
    sphere::Sphere,
    material::{Lambertian, Metal}
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: u32 = 256;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 10;
const MAX_DEPTH: u32 = 4;

const ground: Lambertian = Lambertian::new(0.8, 0.8, 0.0);
const center: Lambertian = Lambertian::new(0.7, 0.3, 0.3);
const left: Metal = Metal::new(0.8, 0.8, 0.8, 0.0);
const right: Metal = Metal::new(0.8, 0.6, 0.2, 0.6);

fn main() {
    let camera = Camera::new()
        .look_from(Vec3::new(-2.0, 2.0, 1.0 ))
        .look_at(Vec3::new(0.0, 0.0, -1.0 ))
        .vfov(20.0)
        .aspect_ratio(ASPECT_RATIO)
        .build();

    let spheres = vec![
      Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground),
      Sphere::new(Vec3::new(0.0,    0.0, -1.0),   0.5, left),
      Sphere::new(Vec3::new(-1.0,   0.0, -1.0),   0.5, center),
      Sphere::new(Vec3::new(1.0,    0.0, -1.0),   0.5, right),
    ];

    let world = World::new()
        .camera(camera)
        .spheres(spheres)
        .build();

    let pixels = world.render(WIDTH, HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH);
    create_image_from(pixels);
}

fn create_image_from(pixels: Vec<Color>) {
    let mut file = File::create("world.ppm")
        .expect("Error creating file");

    let content = pixels
        .into_iter()
        .map(|c| {
            // gamma correction & scale to average color
            let scale = 1.0 / SAMPLES_PER_PIXEL as f64;
            let r = (c.x * scale).sqrt();
            let g = (c.y * scale).sqrt();
            let b = (c.z * scale).sqrt();

            let r = r.clamp(0.0, 1.0) * 255.0;
            let g = g.clamp(0.0, 1.0) * 255.0;
            let b = b.clamp(0.0, 1.0) * 255.0;

            format!("{} {} {}", r as u32, g as u32, b as u32)
        })
        .collect::<Vec<String>>()
        .join("\n");
    
    file.write_all(format!("P3\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes())
        .expect("Error writing file header");
    file.write_all(content.as_bytes()).expect("Error writing pixels");
}
