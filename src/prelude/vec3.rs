use {
    super::{Color, Random, PI},
    std::{
        fmt::Display,
        iter::Sum,
        ops::{
            Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub,
            SubAssign,
        },
    },
};

#[derive(Default, Clone, Debug, PartialOrd, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub fn random_in_unit_box() -> Self {
        Self::new(
            Random::range(-1.0..1.0),
            Random::range(-1.0..1.0),
            Random::range(-1.0..1.0),
        )
    }

    #[must_use]
    pub fn random_range(r: Range<f64>) -> Self {
        Self::new(
            Random::range(r.clone()),
            Random::range(r.clone()),
            Random::range(r),
        )
    }

    #[must_use]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in_unit_box();
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[must_use]
    pub fn random_in_unit_hemisphere(dir: &Self) -> Self {
        let u = Self::random_in_unit_sphere();
        if u.dot(dir) > 0.0 {
            u
        } else {
            -u
        }
    }

    #[must_use]
    pub fn random_unit() -> Self {
        let a: f64 = Random::range(0.0..(2.0 * PI));
        let z: f64 = Random::range(-1.0..1.0);
        let r = (1.0 - z * z).sqrt();
        Self::new(r * a.cos(), r * a.sin(), z)
    }

    #[must_use]
    pub fn random_unit_dir(dir: &Self) -> Self {
        let u = Self::random_unit();
        if u.dot(dir) > 0.0 {
            u
        } else {
            -u
        }
    }

    #[must_use]
    pub fn random_unit_disk() -> Self {
        loop {
            let p = Self::new(Random::range(-1.0..1.0), Random::range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[must_use]
    pub fn length_squared(&self) -> f64 {
        self.z
            .mul_add(self.z, self.x.mul_add(self.x, self.y * self.y))
    }

    #[must_use]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    #[must_use]
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.z.mul_add(rhs.z, self.x.mul_add(rhs.x, self.y * rhs.y))
    }

    #[must_use]
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    #[must_use]
    pub fn unit(&self) -> Self {
        self / self.length()
    }

    #[allow(clippy::cast_precision_loss)] // sample count is small enough in practice
    #[must_use]
    pub fn into_color(mut self, sample_count: usize, gamma: bool) -> Color {
        self /= sample_count as f64;
        if gamma {
            Color::newf(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
        } else {
            Color::newf(self.x, self.y, self.z)
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}", self.x, self.y, self.z))
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 can only index by 0-2, {} provided", index),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 can only index by 0-2, {} provided", index),
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl Add<Self> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        self + &rhs
    }
}

impl Add<Self> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add<&Self> for Vec3 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        &self + rhs
    }
}

impl Sub<Self> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        self - &rhs
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Sub<&Self> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        &self - rhs
    }
}

impl Mul<Self> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self * &rhs
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&Self> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&Color> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Color) -> Self::Output {
        let rhs = rhs.f();
        Vec3::new(
            self.x * rhs.r as f64,
            self.y * rhs.g as f64,
            self.z * rhs.b as f64,
        )
    }
}

impl Mul<Color> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Color) -> Self::Output {
        self * &rhs
    }
}

impl Mul<Color> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Color) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&Color> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: &Color) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&Vec3> for &Color {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for Color {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Vec3> for Color {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<Vec3> for &Color {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self * &rhs
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl AddAssign<&Self> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign<&Self> for Vec3 {
    fn sub_assign(&mut self, rhs: &Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<Self> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<Self> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<&Self> for Vec3 {
    fn mul_assign(&mut self, rhs: &Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, val| acc + val)
    }
}

impl From<Color> for Vec3 {
    fn from(c: Color) -> Self {
        let c = c.f();
        Self::new(c.r as f64, c.g as f64, c.b as f64)
    }
}
