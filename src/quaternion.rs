use std::{ops::Mul, f64::consts::PI};

use crate::vector::Vector3;

#[derive(Clone, Copy)]
pub struct Quaternion {
  w: f64,
  x: f64,
  y: f64,
  z: f64,
}

impl Mul for Quaternion {
  type Output = Quaternion;

  fn mul(self, rhs: Self) -> Self::Output {
    Quaternion::new(
      (self.w * rhs.w) - (self.x * rhs.x) - (self.y * rhs.y) - (self.z * rhs.z),
      (self.w * rhs.x) + (self.x * rhs.w) - (self.y * rhs.z) + (self.z * rhs.y),
      (self.w * rhs.y) + (self.x * rhs.z) + (self.y * rhs.w) - (self.z * rhs.x),
      (self.w * rhs.z) - (self.x * rhs.y) + (self.y * rhs.x) + (self.z * rhs.w),
    )
  }
}

impl Quaternion {
  pub fn new(w: f64, x: f64, y: f64, z: f64) -> Quaternion {
    Quaternion { w, x, y, z }
  }

  pub fn inverse(&self) -> Quaternion {
    Quaternion::new(
      self.w, 
      -self.x, 
      -self.y, 
      -self.z
    )
  }

  pub fn from_angle_axis(mut angle: f64, axis: Vector3) -> Quaternion {
    while angle < 0.0 {
      angle += PI * 2.0;
    }

    while angle >= PI * 2.0 {
      angle -= PI * 2.0;
    }

    Quaternion::new(
      (angle/2.0).cos(),
      axis.x * (angle/2.0).sin(),
      axis.y * (angle/2.0).sin(),
      axis.z * (angle/2.0).sin(),
    )
  }

  pub fn from_vector_3(vector: Vector3) -> Quaternion {
    Quaternion::new(0.0, vector.x, vector.y, vector.z)
  }

  pub fn to_vector_3(&self) -> Vector3 {
    Vector3::new(self.x, self.y, self.z)
  }
}