pub mod utils;
pub mod camera_settings;
pub mod light_settings;
pub mod object_settings;

use std::{time::Instant};

use egui::{Context};

use crate::{world::{World}};

use self::{utils::{ShowableUI}, camera_settings::CameraSettings, object_settings::ObjectSettings, light_settings::LightSettings};

pub struct GUI {
  windows: Vec<Box<dyn for<'a> ShowableUI<&'a mut World>>>,
  instant: Instant,
}

impl GUI {
  pub fn new() -> GUI {
    GUI {
      windows: vec![
        Box::new(CameraSettings::new()),
        Box::new(ObjectSettings::new()),
        Box::new(LightSettings::new()),
      ],
      instant: Instant::now(),
    }
  }

  pub fn ui(&mut self, ctx: &Context, world: &mut World) {
    self.menu_bar(ctx, world);
    self.windows(ctx, world);
  }

  fn menu_bar(&mut self, ctx: &Context, world: &mut World) {
    let fps = 1.0 / self.instant.elapsed().as_secs_f64();
    self.instant = Instant::now();

    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
          if ui.button("Open").clicked() {
            world.open_world();
          } else if ui.button("Save").clicked() {
            world.save_world();
          }
        });

        ui.menu_button("View", |ui| {
          for window in &mut self.windows {
            if ui.button(window.label()).clicked() {
              window.show();
              ui.close_menu();
            }
          }
        });

        ui.label(format!("fps: {}", fps.round()));
      });
    });
  }

  fn windows(&mut self, ctx: &Context, world: &mut World) {
    for window in &mut self.windows {
      window.ui(ctx, world);
    }
  }
}