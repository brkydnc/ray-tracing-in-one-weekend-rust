use crate::camera::Camera;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Color;
use crossbeam_utils::thread;
use std::ops::Range;
use std::ptr;
use std::sync::{Arc, Mutex};

const BLACK: Color = Color {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const WHITE: Color = Color {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};
const SKY_BLUE: Color = Color {
    x: 0.5,
    y: 0.7,
    z: 1.0,
};

pub struct World<'a> {
    pub camera: Camera,
    pub spheres: Vec<Sphere<'a>>,
}

impl<'a> World<'a> {
    pub fn new() -> WorldBuilder<'a> {
        let camera = Camera::new().build();
        WorldBuilder {
            _camera: camera,
            _spheres: vec![],
        }
    }
    pub fn hit_ray<'b>(&self, ray: &'b Ray) -> Option<HitRecord<'b, '_>> {
        let mut closest: Option<HitRecord> = None;
        let mut closest_t = f64::INFINITY;

        for sphere in self.spheres.iter() {
            let hit = ray.hit(sphere, closest_t);
            if let Some(record) = hit {
                closest_t = record.t;
                closest = Some(record);
            }
        }

        closest
    }

    pub fn ray_color(&self, ray: &Ray, depth: u32) -> Color {
        if depth <= 0 {
            return BLACK;
        };

        if let Some(record) = self.hit_ray(ray) {
            if let Some((attenuation, scattered)) = record.material_scatter() {
                return attenuation * self.ray_color(&scattered, depth - 1);
            }
            return BLACK;
        }

        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);

        WHITE * (1.0 - t) + SKY_BLUE * t
    }

    pub fn render(
        &self,
        width: u32,
        height: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        thread_count: usize,
    ) -> Vec<Color> {
        let pixel_count = (width * height) as usize;
        let reserved: Vec<Color> = Vec::with_capacity(pixel_count);
        let pixels = Arc::new(Mutex::new(reserved));
        let ranges = split_range(pixel_count, thread_count);

        thread::scope(|s| {
            for range in ranges {
                s.spawn(|_| {
                    let range_len = range.len();
                    let range_start = range.start;
                    let mut chunk: Vec<Color> = Vec::with_capacity(range_len);

                    for align in range {
                        let (w, h) = alignment_to_coordinates(align, width as usize);

                        let mut pixel = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..samples_per_pixel {
                            let u = (w as f64 + random(-1.0, 1.0)) / width as f64;
                            let v = (height as f64 - h as f64 + random(-1.0, 1.0)) / height as f64;
                            let ray = self.camera.get_ray(u, v);
                            pixel = pixel + self.ray_color(&ray, max_depth);
                        }
                        chunk.push(pixel);
                    }

                    let mut pixels = pixels.lock().unwrap();
                    let len = pixels.len();
                    // SAFETY: We know that `pixels` is able to hold all pixels
                    // we also know that the alignment is correct due to ranges
                    unsafe {
                        let p = (*pixels).as_mut_ptr();
                        let p = p.add(range_start);
                        let c = chunk.as_ptr();
                        ptr::copy_nonoverlapping(c, p, range_len);
                        pixels.set_len(len + range_len);
                    }
                });
            }
        })
        .expect("Error in spawned threads");

        Arc::try_unwrap(pixels).unwrap().into_inner().unwrap()
    }
}

pub struct WorldBuilder<'a> {
    _camera: Camera,
    _spheres: Vec<Sphere<'a>>,
}

impl<'a> WorldBuilder<'a> {
    pub fn camera(mut self, camera: Camera) -> Self {
        self._camera = camera;
        self
    }
    pub fn spheres(mut self, spheres: Vec<Sphere<'a>>) -> Self {
        self._spheres = spheres;
        self
    }

    pub fn build(self) -> World<'a> {
        World {
            camera: self._camera,
            spheres: self._spheres,
        }
    }
}

fn random(min: f64, max: f64) -> f64 {
    return min + (max - min) * rand::random::<f64>();
}

fn split_range(length: usize, div: usize) -> Vec<Range<usize>> {
    let mut ranges = vec![];
    let remainder = length % div;
    let size = ((length / div) as f64).floor() as usize;

    for i in 1..=div {
        let start = (i - 1) * size;
        let end = if i != div {
            i * size
        } else {
            i * size + remainder
        };
        ranges.push(start..end);
    }

    ranges
}

fn alignment_to_coordinates(alignment: usize, width: usize) -> (usize, usize) {
    let x = alignment % width;
    let y = ((alignment / width) as f64).floor() as usize;
    (x, y)
}
