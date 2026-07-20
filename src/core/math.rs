use serde::{Deserialize, Serialize};

pub type Scalar = f32;

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Vec3 {
    #[inline]
    pub const fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline]
    pub fn distance_sq(self, other: Self) -> Scalar {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }

    #[inline]
    pub fn dot(self, other: Self) -> Scalar {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn length_sq(self) -> Scalar {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let len_sq = self.length_sq();
        if len_sq > 0.0 {
            let inv_len = 1.0 / len_sq.sqrt();
            self.scale(inv_len)
        } else {
            Self::zero()
        }
    }

    #[inline]
    pub fn scale(self, scalar: Scalar) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    #[inline]
    pub fn lerp(self, other: Self, t: Scalar) -> Self {
        Self::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
            self.z + (other.z - self.z) * t,
        )
    }
}

impl core::ops::Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl core::ops::Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl core::ops::Mul<Scalar> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Scalar) -> Self::Output {
        self.scale(rhs)
    }
}
