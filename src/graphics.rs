use std::{borrow::{Cow}};

use glium::{Frame, Surface, Display, texture::{ClientFormat}, Rect, Texture2d, BlitTarget};
use image::{EncodableLayout};

use crate::raytracer::{RayTracer};

pub struct Graphics {
  texture: Texture2d,
  ray_tracer: RayTracer,
}

impl Graphics {
  pub fn new(display: &Display) -> Graphics {

    let mut ray_tracer = RayTracer::new(768, 480);

    let image = ray_tracer.update();
    let image_dimensions = (image.width(), image.height());
    let image = glium::texture::RawImage2d::from_raw_rgb_reversed(&image.as_bytes(), image_dimensions);
    let texture = glium::texture::Texture2d::new(display, image).unwrap();

    Graphics {
      texture,
      ray_tracer,
    }
  }

  pub fn ray_tracer(&mut self) -> &mut RayTracer {
    &mut self.ray_tracer
  }

  pub fn draw(&mut self, target: &mut Frame, display: &Display) {
    let image = self.ray_tracer.update();

    let image_raw = glium::texture::RawImage2d {
      data: Cow::Borrowed(image.as_bytes()),
      width: image.width(),
      height: image.height(),
      format: ClientFormat::U8U8U8
    };

    let rect = Rect { 
      left: 0, 
      bottom: 0, 
      width: image.width(), 
      height: image.height()
    };

    self.texture.write(rect, image_raw);

    let (width, height) = display.get_framebuffer_dimensions();

    let image_ratio = image.width() as f64 / image.height() as f64;
    let dest_ratio = width as f64 / height as f64;

    let dest_rect = if image_ratio > dest_ratio {
      let scale = width as f64 / image.width() as f64;
      let adjusted_height = (image.height() as f64 * scale) as i32;

      BlitTarget {
        left: 0,
        bottom: (height - adjusted_height as u32) / 2,
        width: width as i32,
        height: adjusted_height,
      }
    } else {
      let scale = height as f64 / image.height() as f64;
      let adjusted_width = (image.width() as f64 * scale) as i32;

      BlitTarget {
        left: (width - adjusted_width as u32) / 2,
        bottom: 0,
        width: adjusted_width,
        height: height as i32,
      }
    };

    target.clear_color(0_f32, 0_f32, 0_f32, 1_f32);
    
    self.texture.as_surface().blit_whole_color_to(target, &dest_rect, glium::uniforms::MagnifySamplerFilter::Linear);
  }
}
