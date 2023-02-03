use std::{ops::Neg, vec};

use image::{ImageBuffer, Rgb};
use nalgebra::Vector3;

pub struct Sphere {
  pub center: Vector3<f64>,
  pub radius: f64,
  pub color: egui::Color32
}

impl Sphere {
  pub fn center(&mut self) -> &mut Vector3<f64> {
    &mut self.center
  }

  pub fn color(&mut self) -> &mut egui::Color32 {
    &mut self.color
  }

  pub fn radius(&mut self) -> &mut f64 {
    &mut self.radius
  }
}

impl Default for Sphere {
    fn default() -> Self {
      Self { 
        center: Vector3::new(0_f64, 0_f64, 0_f64), 
        radius: 75_f64, 
        color: egui::Color32::LIGHT_RED
      }
    }
}

pub struct RayTracer {
  image: ImageBuffer<Rgb<u8>, Vec<u8>>,
  spheres: Vec<Sphere>,
  light_direction: Vector3<f64>,
  ambient_light: f64,
}

impl RayTracer {
  pub fn new(width: u32, height: u32) -> RayTracer {
    RayTracer { 
      image: ImageBuffer::new(width, height),
      spheres: vec! [Sphere::default()],
      light_direction: Vector3::new(-0.5_f64, -0.5_f64, 1_f64),
      ambient_light: 0.1
    }
  }

  pub fn add_sphere(&mut self) {
    self.spheres.push(Sphere::default());
  }

  pub fn sphere(&mut self, i: usize) -> &mut Sphere {
    &mut self.spheres[i]
  }

  pub fn sphere_len(&self) -> usize {
    self.spheres.len()
  }

  pub fn ambient_light(&mut self) -> &mut f64 {
    &mut self.ambient_light
  }

  pub fn light_direction(&mut self) -> &mut Vector3<f64> {
    &mut self.light_direction
  }

  pub fn update(&mut self) -> &ImageBuffer<Rgb<u8>, Vec<u8>> {
    let bg_color = Rgb([15, 0, 20]);

    let height = self.image.height();
    let width = self.image.width();

    for y in 0..height {
      for x in 0..width {
        let (x_world, y_world) = self.img_to_world_pos(x, y);

        let mut to_draw: Option<(&Sphere, f64)> = None;
        let mut closest = f64::MAX;

        for sphere in &self.spheres {
          let t = self.calc_t(x_world, y_world, sphere);

          match t {
            Some(point) => {

              if point < closest {
                closest = point;

                let direction = Vector3::new(0_f64, 0_f64, 1_f64);
                let origin = Vector3::new(x_world, y_world, -400_f64);

                let rel_origin = origin - sphere.center;

                let intensity = self.calc_intensity(rel_origin + direction * point);
                to_draw = Some((sphere, intensity));
              }
            },
            None => {},
          }
        }

        match to_draw {
          Some((sphere, i)) => {
            let r = sphere.color.r() as f64;
            let g = sphere.color.g() as f64;
            let b = sphere.color.b() as f64;

            let color = Rgb([
              (r*i) as u8, 
              (g*i) as u8, 
              (b*i) as u8
            ]);

            self.image.put_pixel(x, y, color);
          },
          None => {
            self.image.put_pixel(x, y, bg_color);
          }
        } 
      }
    }

    &self.image
  }

  fn img_to_world_pos(&self, x: u32, y: u32) -> (f64, f64) {
    let height = self.image.height();
    let width = self.image.width();

    (
      x as f64 - width as f64 * 0.5, 
      y as f64 - height as f64 * 0.5
    )
  }

  fn calc_intensity(&self, point: Vector3<f64>) -> f64 {
    let normal = point.normalize();
    normal.dot(&self.light_direction.neg()).max(self.ambient_light)
  }

  fn calc_t(&self, x: f64, y: f64, sphere: &Sphere) -> Option<f64> {
    let radius = sphere.radius;
    let direction = Vector3::new(0_f64, 0_f64, 1_f64);
    let origin = Vector3::new(x, y, -400_f64);

    let rel_origin = origin - sphere.center;

    let a = direction.dot(&direction);
    let b = rel_origin.dot(&direction) * 2_f64;
    let c = rel_origin.dot(&rel_origin) - radius.powi(2);

    // b^2 - 4ac
    let discriminant = b.powi(2) - 4_f64 * a * c;

    if discriminant < 0_f64 {
      return None;
    }

    // (-b +- sqrt(discriminant)) / 2a
    let solution_1 = (-b + discriminant.sqrt()) / (2_f64 * a);

    if solution_1 < 0_f64 {
      return None;
    }

    let solution_2 = (-b - discriminant.sqrt()) / (2_f64 * a);

    return Some(solution_2);
  }
}