
use std::{ops::Neg};
use rayon::prelude::*;

use crate::{world::{World, Object, Ray, RGB, RayIntersection, Light}, vector::Vector3};

pub struct RenderedData<'a> {
  pub image_buffer: &'a Vec<[u8; 3]>,
  pub image_width: u32,
  pub image_height: u32,
}

pub struct Renderer {
  image_buffer: Vec<[u8; 3]>,
  image_width: u32,
  image_height: u32,
}

impl Renderer {
  pub fn new() -> Renderer {
    Renderer {
      image_buffer: Vec::new(),
      image_width: 600,
      image_height: 400,
    }
  }

  pub fn render(&mut self, world: &mut World) -> RenderedData {
    self.image_width = world.camera().camera_info().viewport_width;
    self.image_height = world.camera().camera_info().viewport_height;

    let rays = world.camera().rays();
    let buffer_size = (self.image_width * self.image_height) as usize;

    if self.image_buffer.len() != buffer_size {
      self.image_buffer.resize(buffer_size, [0, 0, 0]);
    }

    let depth = *world.camera().depth();

    self.image_buffer.par_iter_mut().zip(rays).for_each(|(pixel, ray)| {
      let color = Renderer::trace_ray_color(ray, world, depth);

      *pixel.get_mut(0).unwrap() = (color[0] * 255.0) as u8;
      *pixel.get_mut(1).unwrap() = (color[1] * 255.0) as u8;
      *pixel.get_mut(2).unwrap() = (color[2] * 255.0) as u8;
    });

    RenderedData { 
      image_buffer: &self.image_buffer,
      image_width: self.image_width, 
      image_height: self.image_height 
    }
  }

  pub fn trace_ray<'a>(ray: &'a Ray, world: &'a World) -> Option<RayIntersection<'a>> {
    let mut closest: Option<RayIntersection> = None;

    for object in world.objects() {
      match object {
        Object::Sphere(sphere_data) => {
          let [t1, t2] = sphere_data.trace_ray(ray);

          let (t, n1, n2) = {
              if let Some(t2) = t2 {
                (t2, 1.0, sphere_data.material.refractive_index)
              } else if let Some(t1) = t1 {
                (t1, sphere_data.material.refractive_index, 1.0)
              } else {
                continue;
              }
          };

          let intersection = ray.position_from_distance(t);

          let normal = if t == t1.unwrap() {
            (sphere_data.position - intersection).normalise()
          } else {
            (intersection - sphere_data.position).normalise()
          };

          match &closest {
            Some(val) => {
              if t < *val.distance() {
                closest = Some(
                  RayIntersection::new(ray, &sphere_data.material, t, normal, n1, n2)
                )
              }
            },
            None => {
              closest = Some(
                RayIntersection::new(ray, &sphere_data.material, t, normal, n1, n2)
              );
            }
          }
        },
        Object::Plane(plane_data) => {
          let t = plane_data.trace_ray(ray);

          if t.is_none() {
            continue;
          }

          let t = t.unwrap();

          let up = Vector3::new(0.0, 1.0, 0.0);

          match &closest {
            Some(val) => {
              if t < *val.distance() {
                closest = Some(
                  RayIntersection::new(ray, &plane_data.material, t, up, 1.0, 1.0)
                )
              }
            },
            None => {
              closest = Some(
                RayIntersection::new(ray, &plane_data.material, t, up, 1.0, 1.0)
              );
            }
          }
        }
      }
    }

    closest
  }

  pub fn trace_ray_color(ray: &Ray, world: &World, depth: u32) -> RGB {
    let closest = Renderer::trace_ray(ray, world);

    if depth == 0 || closest.is_none() {
      return world.camera().camera_info().miss_color;
    }

    let closest = closest.unwrap();

    let point = closest.position();
    let normal = *closest.normal();
    let material = closest.material();

    let mut diffuse = [0.0, 0.0, 0.0];
    let mut specular = [0.0, 0.0, 0.0];
    let mut ambient = [0.0, 0.0, 0.0];

    for light in world.lights() {
      match light {
        Light::Point(light) => {
          let light_ray = Ray {
            position: point,
            direction: (light.position - point).normalise(),
          };

          let shadow_ray = Renderer::trace_ray(&light_ray, world);

          let has_shadow = shadow_ray.is_some();

          let shadow: f32 = if has_shadow {
            0.3
          } else {
            1.0
          };

          let intensity = normal.dot(&light_ray.direction).clamp(0.0, 1.0) as f32;

          diffuse[0] += light.color[0] * intensity * shadow;
          diffuse[1] += light.color[1] * intensity * shadow;
          diffuse[2] += light.color[2] * intensity * shadow;


          if material.has_specular {
            let r = ray.direction - normal * (ray.direction.dot(&normal) * 2.0);
            let specular_i = light_ray.direction.dot(&r).max(0.0).powf(material.specular_reflection) as f32;

            specular[0] += light.color[0] * specular_i;
            specular[1] += light.color[1] * specular_i;
            specular[2] += light.color[2] * specular_i;
          }
        }
        Light::Directional(light) => {
          let light_ray = Ray {
            position: point,
            direction: light.direction.normalise(),
          };

          let bounce = Renderer::trace_ray(&light_ray, world);

          let shadow: f32 = if bounce.is_none() {
            0.3
          } else {
            1.0
          };

          let intensity = normal.dot(&light_ray.direction.neg()).clamp(0.0, 1.0) as f32;

          diffuse[0] += light.color[0] * intensity * shadow;
          diffuse[1] += light.color[1] * intensity * shadow;
          diffuse[2] += light.color[2] * intensity * shadow;

          if material.has_specular {
            let r = light_ray.direction - normal * (light_ray.direction.dot(&normal) * 2.0);
            let specular_i = ray.direction.neg().dot(&r).powf(material.specular_reflection) as f32;

            specular[0] += light.color[0] * specular_i * shadow * intensity;
            specular[1] += light.color[1] * specular_i * shadow * intensity;
            specular[2] += light.color[2] * specular_i * shadow * intensity;
          }
        },
        Light::Ambient(light) => {
          ambient[0] += light.color[0];
          ambient[1] += light.color[1];
          ambient[2] += light.color[2];
        },
      }
    }

    let color = material.color;
    let ambient_reflection = material.ambient_reflection as f32;
    let diffuse_reflection = material.diffuse_reflection as f32;

    ambient[0] *= ambient_reflection;
    ambient[1] *= ambient_reflection;
    ambient[2] *= ambient_reflection;

    diffuse[0] *= diffuse_reflection;
    diffuse[1] *= diffuse_reflection;
    diffuse[2] *= diffuse_reflection;

    let mut color = [
      (color[0] * (ambient[0] + diffuse[0]) + specular[0]).clamp(0.0, 1.0),
      (color[1] * (ambient[1] + diffuse[1]) + specular[1]).clamp(0.0, 1.0),
      (color[2] * (ambient[2] + diffuse[2]) + specular[2]).clamp(0.0, 1.0),
    ];


    let reflectivity = material.reflectivity as f32;

    if reflectivity > 0.0 {
      let direction = ray.direction - normal * (ray.direction.dot(&normal) * 2.0);

      let bounce_ray = Ray { 
        position: point, 
        direction 
      };

      let bounce_color = Renderer::trace_ray_color(&bounce_ray, world, depth-1);

      color[0] = (1.0 - reflectivity) * color[0] + reflectivity * bounce_color[0];
      color[1] = (1.0 - reflectivity) * color[1] + reflectivity * bounce_color[1];
      color[2] = (1.0 - reflectivity) * color[2] + reflectivity * bounce_color[2];
    }

    let transparency = material.transparency as f32;

    if transparency > 0.0 {
      let direction = Renderer::calc_snells_law(ray.direction, normal, *closest.n1(), *closest.n2());

      let through_ray = Ray { 
        position: point, 
        direction
      };

      let through_color = Renderer::trace_ray_color(&through_ray, world, depth-1);

      color[0] = (1.0 - transparency) * color[0] + transparency * through_color[0];
      color[1] = (1.0 - transparency) * color[1] + transparency * through_color[1];
      color[2] = (1.0 - transparency) * color[2] + transparency * through_color[2];
    }

    color
  }

  pub fn calc_snells_law(i: Vector3, normal: Vector3, n1: f64, n2: f64) -> Vector3 {
    let mu = n1/n2;
    let ni = normal.dot(&i);

    -normal * (1.0 - (mu.powi(2) * (1.0 - ni.powi(2)))).sqrt() + (i - normal * ni) * mu
  }
}