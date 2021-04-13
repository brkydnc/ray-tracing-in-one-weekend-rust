use crate::ray::Ray;
use crate::vec3::Color;
use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::hit_record::HitRecord;

const WHITE: Color = Color { x: 1.0, y: 1.0, z: 1.0 };
const SKY_BLUE: Color = Color { x: 0.5, y: 0.7, z: 1.0 };

pub struct World<'a> {
    pub camera: Camera,
    pub spheres: Vec<Sphere<'a>>,
}

impl<'a> World<'a> {
    pub fn new() -> WorldBuilder<'a> {
        let camera = Camera::new().build();
        WorldBuilder { _camera: camera, _spheres: vec![], }
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
      if depth <= 0 { return Color::new(0.0, 0.0, 0.0); };

      if let Some(record) = self.hit_ray(ray) {
        if let Some((attenuation, scattered)) = record.material_scatter() {
          return attenuation * self.ray_color(&scattered, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
      }

      let unit_direction = ray.direction.normalize();
      let t = 0.5 * (unit_direction.y + 1.0);

      WHITE * (1.0 - t) + SKY_BLUE * t
    }

    pub fn render(
        &self,
        width: u32, height: u32,
        samples_per_pixel: u32, max_depth: u32
    ) -> Vec<Color> {
        let mut pixels: Vec<Color> = Vec::with_capacity((width * height) as usize);

        for h in 0..height {
            for w in 0..width {
                let mut pixel = Color::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = (w as f64 + random(-1.0, 1.0)) / width as f64;
                    let v = (height as f64 - h as f64 + random(-1.0, 1.0)) / height as f64;
                    let ray = self.camera.get_ray(u, v);
                    pixel = pixel + self.ray_color(&ray, max_depth);
                }
                pixels.push(pixel);
            }
        }
        
        pixels
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
