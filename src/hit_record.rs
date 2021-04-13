use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Vec3};

pub struct HitRecord<'ray, 'sphere> {
    pub ray: &'ray Ray,
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub sphere: &'sphere Sphere<'sphere>,
}

impl<'ray, 'sphere> HitRecord<'ray, 'sphere> {
    pub fn material_scatter(&self) -> Option<(Color, Ray)> {
        self.sphere.material.scatter(self.ray, self)
    }
}
