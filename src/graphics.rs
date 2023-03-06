use std::{borrow::{Cow}};

use glium::{Frame, Surface, Display, texture::{ClientFormat}, Rect, Texture2d, BlitTarget, uniforms::MagnifySamplerFilter};

use crate::{renderer::Renderer, world::World};

pub struct Graphics {
  texture: Texture2d,
  renderer: Renderer,
  world: World,
}

impl Graphics {
  pub fn new(display: &Display) -> Graphics {

    let mut renderer = Renderer::new();

    let mut world = World::new();

    let data = renderer.render(&mut world);

    let image_raw = glium::texture::RawImage2d {
      data: Cow::Borrowed(data.image_buffer.flatten()),
      format: ClientFormat::U8U8U8,
      width: data.image_width,
      height: data.image_height
    };

    let texture = glium::texture::Texture2d::new(display, image_raw).unwrap();

    Graphics {
      texture,
      renderer,
      world
    }
  }

  pub fn world(&mut self) -> &mut World {
    &mut self.world
  }
  
  pub fn draw(&mut self, target: &mut Frame, display: &Display) {
    let renderer = &mut self.renderer;

    let data = renderer.render(&mut self.world);

    let image_raw = glium::texture::RawImage2d {
      data: Cow::Borrowed(data.image_buffer.flatten()),
      format: ClientFormat::U8U8U8,
      width: data.image_width,
      height: data.image_height,
    };

    let rect = Rect { 
      left: 0, 
      bottom: 0, 
      width: data.image_width,
      height: data.image_height,
    };

    if self.texture.dimensions() == (data.image_width, data.image_height) {
      self.texture.write(rect, image_raw);
    } else {
      self.texture = glium::texture::Texture2d::new(display, image_raw).unwrap();
    }

    let (width, height) = display.get_framebuffer_dimensions();

    let image_ratio = data.image_width as f64 / data.image_height as f64;
    let dest_ratio = width as f64 / height as f64;

    let dest_rect = if image_ratio > dest_ratio {
      let scale = width as f64 / data.image_width as f64;
      let adjusted_height = (data.image_height as f64 * scale) as i32;

      BlitTarget {
        left: 0,
        bottom: (height - adjusted_height as u32) / 2,
        width: width as i32,
        height: adjusted_height,
      }
    } else {
      let scale = height as f64 / data.image_height as f64;
      let adjusted_width = (data.image_width as f64 * scale) as i32;

      BlitTarget {
        left: (width - adjusted_width as u32) / 2,
        bottom: 0,
        width: adjusted_width,
        height: height as i32,
      }
    };

    target.clear_color(0_f32, 0_f32, 0_f32, 1_f32);

    self.texture.as_surface().blit_whole_color_to(target, &dest_rect, MagnifySamplerFilter::Nearest);
  }
}
