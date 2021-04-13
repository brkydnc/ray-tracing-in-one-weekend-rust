use crate::hit_record::HitRecord;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn hit<'a>(&self, sphere: &'a Sphere, t_max: f64) -> Option<HitRecord<'_, 'a>> {
        let ctr = self.origin - sphere.position;
        let a = self.direction.dot(self.direction);
        let half_b = self.direction.dot(ctr);
        let c = ctr.dot(ctr) - sphere.radius.powf(2.0);

        let discriminant = half_b.powf(2.0) - a * c;
        if discriminant < 0.0 {
            return None;
        };

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < 0.001 || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < 0.001 || t_max < root {
                return None;
            };
        }

        let point = self.point_at(root);
        let normal = correct_normal(self, (point - sphere.position) / sphere.radius);

        let record = HitRecord {
            ray: self,
            point,
            normal,
            t: root,
            sphere,
        };

        Some(record)
    }
}

fn correct_normal(ray: &Ray, normal: Vec3) -> Vec3 {
    let same_direction = ray.direction.dot(normal) > 0.0;
    if same_direction {
        -normal
    } else {
        normal
    }
}
