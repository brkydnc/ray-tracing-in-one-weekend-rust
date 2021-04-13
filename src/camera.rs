use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            lookfrom: Vec3::new(0.0, 0.0, 0.0),
            lookat: Vec3::new(0.0, 0.0, -1.0),
            vfov: 50.0,
            aspect_ratio: 16.0 / 9.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let offset = self.horizontal * u + self.vertical * v;
        let direction = self.lower_left_corner + offset - self.origin;
        Ray {
            origin: self.origin,
            direction,
        }
    }
}

pub struct CameraBuilder {
    lookfrom: Vec3,
    lookat: Vec3,
    vfov: f64,
    aspect_ratio: f64,
}

impl CameraBuilder {
    pub fn look_from(mut self, point: Vec3) -> Self {
        self.lookfrom = point;
        self
    }

    pub fn look_at(mut self, point: Vec3) -> Self {
        self.lookat = point;
        self
    }

    pub fn vfov(mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        self
    }

    pub fn aspect_ratio(mut self, r: f64) -> Self {
        self.aspect_ratio = r;
        self
    }

    pub fn build(self) -> Camera {
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        let abs_vertical = Vec3::new(0.0, 1.0, 0.0);
        let w = (self.lookfrom - self.lookat).normalize();
        let u = abs_vertical.cross(w).normalize();
        let v = w.cross(u); // normalized

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let offset = horizontal * 0.5 + vertical * 0.5;
        let lower_left_corner = self.lookfrom - offset - w;

        Camera {
            origin: self.lookfrom,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}
