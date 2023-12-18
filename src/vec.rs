use std::{fmt, ops};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/* CONSTRUCTORS */
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn origin() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

/* ANALYTIC GEOMETRY */
impl Vec3 {
    fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Return the dot/scalar product of a Vec3
    /// It is used to determine orthogonality
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Return the cross/vector product of two Vec3s
    /// The returned Vec3 is orthogonal to the
    /// passed Vec3s
    pub fn cross(&self, other: &Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Vec3::new(x, y, z)
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Vec3::new(x, y, z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Vec3::new(x, y, z)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        Vec3::new(x, y, z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Vec3::new(x, y, z)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let x = self * rhs.x;
        let y = self * rhs.y;
        let z = self * rhs.z;
        Vec3::new(x, y, z)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("A Vec3 only has three coordinates"),
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3::origin()
    }
}
