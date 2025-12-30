use std::ops::Sub;

pub struct Vec3D<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

pub type IVec3D = Vec3D<i32>;

impl IVec3D {
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn length(&self) -> f64 {
        ((i64::from(self.x).pow(2) + i64::from(self.y).pow(2) + i64::from(self.z).pow(2)) as f64)
            .sqrt()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy + Sub<Output = T>> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn diff(&self, other: &Point3D<T>) -> Vec3D<T> {
        Vec3D::new(other.x - self.x, other.y - self.y, other.z - self.z)
    }
}

pub type IPoint3D = Point3D<i32>;

impl IPoint3D {
    #[must_use]
    pub fn distance(&self, other: &IPoint3D) -> f64 {
        self.diff(other).length()
    }
}
