use crate::material::Material;
use crate::vec3::Vec3;

pub struct Sphere<'a> {
    pub position: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material + 'a>,
}

impl<'a> Sphere<'a> {
    pub fn new<T>(position: Vec3, radius: f64, material: T) -> Self
    where
        T: Material + 'a,
    {
        Self {
            position,
            radius,
            material: Box::new(material),
        }
    }
}
