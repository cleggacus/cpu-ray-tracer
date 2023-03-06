use std::ops::{Add, Sub, Mul, Div, Neg};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

impl Neg for Vector3 {
  type Output = Vector3;

  fn neg(self) -> Self::Output {
    return Vector3::new(
      -self.x,
      -self.y,
      -self.z
    )
  }
}

impl Div<f64> for Vector3 {
  type Output = Vector3;

  fn div(self, rhs: f64) -> Self::Output {
    return Vector3::new(
      self.x / rhs,
      self.y / rhs,
      self.z / rhs
    )
  }
}

impl Div for Vector3 {
  type Output = Vector3;

  fn div(self, rhs: Self) -> Self::Output {
    return Vector3::new(
      self.x / rhs.x,
      self.y / rhs.y,
      self.z / rhs.z
    )
  }
}

impl Mul<f64> for Vector3 {
  type Output = Vector3;

  fn mul(self, rhs: f64) -> Self::Output {
    return Vector3::new(
      self.x * rhs,
      self.y * rhs,
      self.z * rhs
    )
  }
}

impl Mul for Vector3 {
  type Output = Vector3;

  fn mul(self, rhs: Self) -> Self::Output {
    return Vector3::new(
      self.x * rhs.x,
      self.y * rhs.y,
      self.z * rhs.z
    )
  }
}

impl Sub<f64> for Vector3 {
  type Output = Vector3;

  fn sub(self, rhs: f64) -> Self::Output {
    return Vector3::new(
      self.x - rhs,
      self.y - rhs,
      self.z - rhs
    )
  }
}

impl Sub for Vector3 {
  type Output = Vector3;

  fn sub(self, rhs: Self) -> Self::Output {
    return Vector3::new(
      self.x - rhs.x,
      self.y - rhs.y,
      self.z - rhs.z
    )
  }
}

impl Add<f64> for Vector3 {
  type Output = Vector3;

  fn add(self, rhs: f64) -> Self::Output {
    return Vector3::new(
      self.x + rhs,
      self.y + rhs,
      self.z + rhs
    )
  }
}

impl Add for Vector3 {
  type Output = Vector3;

  fn add(self, rhs: Self) -> Self::Output {
    return Vector3::new(
      self.x + rhs.x,
      self.y + rhs.y,
      self.z + rhs.z
    )
  }
}

impl Vector3 {
  pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
    Vector3 { x, y, z }
  }

  pub fn normalise(self) -> Vector3 {
    let mag = self.mag();
    self / mag
  }

  pub fn dot(&self, rhs: &Vector3) -> f64 {
    self.x * rhs.x +
    self.y * rhs.y +
    self.z * rhs.z
  }

  pub fn cross(&self, rhs: &Vector3) -> Vector3 {
    Vector3::new(
      self.y * rhs.z - self.z * rhs.y,
      self.z * rhs.x - self.x * rhs.z,
      self.x * rhs.y - self.y * rhs.x
    )
  }

  pub fn mag(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
  }

  pub fn x_mut(&mut self) -> &mut f64 {
    &mut self.x
  }

  pub fn y_mut(&mut self) -> &mut f64 {
    &mut self.y
  }

  pub fn z_mut(&mut self) -> &mut f64 {
    &mut self.z
  }
}