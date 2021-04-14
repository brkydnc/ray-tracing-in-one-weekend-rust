use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub color: Color,
    pub fuzz: f64,
}

impl Metal {
    pub const fn new(r: f64, g: f64, b: f64, f: f64) -> Self {
        Metal {
            color: Color { x: r, y: g, z: b },
            fuzz: f,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let fuzz = if self.fuzz < 1.0 { self.fuzz } else { 1.0 };
        let reflected = ray.direction.normalize().reflect(record.normal);
        let fuzzy_direction = reflected + fuzz * Vec3::random_in_unit_sphere();
        let scattered = Ray::new(record.point, fuzzy_direction);

        if scattered.direction.dot(record.normal) > 0.0 {
            Some((self.color, scattered))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub color: Color,
}

impl Lambertian {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Lambertian {
            color: Color { x: r, y: g, z: b },
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let random = record.normal + Vec3::random_unit();
        let direction = if random.any_near_zero() {
            record.normal
        } else {
            random
        };
        let scattered = Ray::new(record.point, direction);

        Some((self.color, scattered))
    }
}
