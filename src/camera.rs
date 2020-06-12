use {
    crate::geometry::{Geometry, World},
    crate::image::Painter,
    crate::prelude::*,
    std::path::Path,
};

#[derive(Debug)]
pub struct Camera {
    origin: Point3,
    lb: Point3,
    horizontal_full: Vec3,
    vertical_full: Vec3,
    horizontal_unit: Vec3,
    vertical_unit: Vec3,
    aspect_ratio: f64,
    aperture: f64,
    shutter_speed: f64,
}

impl Camera {
    #[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)] // internal
    pub(self) fn new(
        look_from: &Point3, look_at: &Point3, vup: &Vec3, fov: f64, aspect_ratio: f64,
        aperture: f64, focus_distance: f64, shutter_speed: f64,
    ) -> Self {
        let fov = d2r(fov);
        let h = (fov / 2.0).tan();
        let vh = 2.0 * h;
        let vw = vh * aspect_ratio;

        let w = (look_at - look_from).unit();
        let horizontal_unit = w.cross(vup).unit();
        let vertical_unit = horizontal_unit.cross(&w).unit();

        let horizontal_full = focus_distance * vw * &horizontal_unit;
        let vertical_full = focus_distance * vh * &vertical_unit;
        let lb = look_from - &horizontal_full / 2.0 - &vertical_full / 2.0 + focus_distance * w;
        Self {
            origin: look_from.clone(),
            lb,
            horizontal_full,
            vertical_full,
            horizontal_unit,
            vertical_unit,
            aspect_ratio,
            aperture,
            shutter_speed,
        }
    }

    #[must_use]
    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.aperture / 2.0 * Vec3::random_unit_disk();
        let offset = &self.horizontal_unit * rd.x + &self.vertical_unit * rd.y;
        let origin = &self.origin + offset;
        let direction = &self.lb + u * &self.horizontal_full + v * &self.vertical_full - &origin;

        Ray::new(origin, direction, self.shutter_speed * Random::normal())
    }

    #[must_use]
    pub const fn take_photo<'i, 'w>(&'i self, world: &'w World) -> TakePhotoSettings<'i, 'w> {
        TakePhotoSettings::new(self, world)
    }
}

#[derive(Debug, Clone)]
pub struct TakePhotoSettings<'c, 'w> {
    camera: &'c Camera,
    world: &'w World,
    depth: usize,
    samples: usize,
    picture_height: usize,
}

impl<'c, 'w> TakePhotoSettings<'c, 'w> {
    #[must_use]
    pub const fn new(camera: &'c Camera, world: &'w World) -> Self {
        Self {
            camera,
            world,
            depth: 8,
            samples: 50,
            picture_height: 108,
        }
    }

    #[must_use]
    pub const fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    #[must_use]
    pub const fn samples(mut self, samples: usize) -> Self {
        self.samples = samples;
        self
    }

    #[must_use]
    pub const fn height(mut self, height: usize) -> Self {
        self.picture_height = height;
        self
    }

    fn background(ray: &Ray) -> Color {
        let unit = ray.direction.unit();
        let t = 0.5 * (unit.y + 1.0);
        Color::newf(1.0, 1.0, 1.0).gradient(&Color::newf(0.5, 0.7, 1.0), t)
    }

    fn ray_color(ray: &Ray, world: &World, depth: usize) -> Color {
        if depth == 0 {
            return Color::default();
        }
        if let Some(hit) = world.hit(ray, 0.001..INFINITY) {
            let material = hit.material;
            if let Some(scattered) = material.scatter(ray, hit) {
                return scattered.color * Self::ray_color(&scattered.ray, world, depth - 1);
            }
            return Color::default();
        }

        Self::background(ray)
    }

    /// # Errors
    /// When open or save to file failed
    #[allow(clippy::needless_pass_by_value)] // Directly used public API, add & will make it harder to use
    pub fn shot<P: AsRef<Path>>(&self, path: Option<P>) -> std::io::Result<()> {
        // because picture height/width is always positive and small enough in practice
        #[allow(
            clippy::cast_sign_loss,
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation
        )]
        Painter::new(
            (self.picture_height as f64 * self.camera.aspect_ratio).round() as usize,
            self.picture_height,
        )
        .set_samples(self.samples)
        .draw(&path, |u, v| -> Vec3 {
            let ray = self.camera.ray(u, v);
            Self::ray_color(&ray, self.world, self.depth).into()
        })
    }
}

#[derive(Debug)]
pub struct CameraBuilder {
    look_from: Point3,
    look_at: Point3,
    vup: Vec3,
    fov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_distance: f64,
    shutter_speed: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            look_from: Point3::default(),
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            fov: 90.0,
            aspect_ratio: 16.0 / 9.0,
            aperture: 0.0,
            focus_distance: 1.0,
            shutter_speed: 0.0,
        }
    }
}

impl CameraBuilder {
    #[must_use]
    pub const fn look_from(mut self, look_from: Point3) -> Self {
        self.look_from = look_from;
        self
    }

    #[must_use]
    pub const fn look_at(mut self, look_at: Point3) -> Self {
        self.look_at = look_at;
        self
    }

    #[must_use]
    pub const fn vup(mut self, vup: Vec3) -> Self {
        self.vup = vup;
        self
    }

    #[must_use]
    pub fn fov(mut self, fov: f64) -> Self {
        debug_assert!(0.0 < fov && fov <= 180.0, "fov = {}", fov);
        self.fov = fov;
        self
    }

    #[must_use]
    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        debug_assert!(aspect_ratio > 0.0, "aspect_ratio = {}", aspect_ratio);
        self.aspect_ratio = aspect_ratio;
        self
    }

    #[must_use]
    pub fn aperture(mut self, aperture: f64) -> Self {
        debug_assert!(aperture >= 0.0, "aperture = {}", aperture);
        self.aperture = aperture;
        self
    }

    #[must_use]
    pub fn focus(mut self, distance: f64) -> Self {
        debug_assert!(distance >= 0.0, "distance = {}", distance);
        self.focus_distance = distance;
        self
    }

    #[must_use]
    pub fn focus_to_look_at(self) -> Self {
        let distance = (&self.look_at - &self.look_from).length();
        self.focus(distance)
    }

    #[must_use]
    pub fn shutter_speed(mut self, duration: f64) -> Self {
        self.shutter_speed = duration;
        self
    }

    #[must_use]
    pub fn build(self) -> Camera {
        Camera::new(
            &self.look_from,
            &self.look_at,
            &self.vup,
            self.fov,
            self.aspect_ratio,
            self.aperture,
            self.focus_distance,
            self.shutter_speed,
        )
    }
}
