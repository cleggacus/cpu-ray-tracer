use std::borrow::BorrowMut;

use egui::{Style, epaint::Shadow, Frame};

use crate::world::{World, Light, PointLight, DirectionalLight, AmbientLight};

use super::utils::{ShowableUI, combo};

pub struct LightSettings {
  showing: bool,
  light: usize,
  label: String,
}

impl ShowableUI<&mut World> for LightSettings {
  fn ui(&mut self, ctx: &egui::Context, world: &mut World) {
    egui::Window::new(self.label())
      .open(&mut self.showing)
      .frame(
        Frame::window(&Style::default())
          .shadow(Shadow::NONE)
      )
      .show(ctx, |ui| {
        egui::Grid::new("my_grid")
          .num_columns(2)
          .spacing([40.0, 4.0])
          .striped(true)
          .show(ui, |ui| {
            let lights = world.lights_mut();

            if self.light >= lights.len() {
              self.light = 0;
            }

            if lights.len() > 0 {
              ui.label("Selected Light");

              combo(ui, &mut self.light, 
                (0..lights.len()).collect::<Vec<usize>>()
              );

              ui.end_row(); 

              match lights[self.light].borrow_mut() {
                Light::Ambient(light) => {
                  ui.label("Color");
                  ui.color_edit_button_rgb(&mut light.color);
                  ui.end_row(); 
                },
                Light::Directional(light) => {
                  ui.label("Color");
                  ui.color_edit_button_rgb(&mut light.color);
                  ui.end_row(); 
                  ui.label("Direction");
                  ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(light.direction.x_mut()));
                    ui.add(egui::DragValue::new(light.direction.y_mut()));
                    ui.add(egui::DragValue::new(light.direction.z_mut()));
                  });
                  ui.end_row();
                },
                Light::Point(light) => {
                  ui.label("Color");
                  ui.color_edit_button_rgb(&mut light.color);

                  ui.end_row();

                  ui.label("Position");
                  ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(light.position.x_mut()));
                    ui.add(egui::DragValue::new(light.position.y_mut()));
                    ui.add(egui::DragValue::new(light.position.z_mut()));
                  });
                  ui.end_row();
                }
              }

              ui.label("Remove Light");
              if ui.button("Remove").clicked() {
                world.lights_mut().remove(self.light);
              }
              ui.end_row(); 
            }

            ui.label("Add Directional");
            if ui.button("Add").clicked() {
              world.lights_mut().push(Light::Directional(DirectionalLight::new()));
            }
            ui.end_row(); 

            ui.label("Add Point");
            if ui.button("Add").clicked() {
              world.lights_mut().push(Light::Point(PointLight::new()));
            }
            ui.end_row(); 

            ui.label("Add Ambient");
            if ui.button("Add").clicked() {
              world.lights_mut().push(Light::Ambient(AmbientLight::new()));
            }
            ui.end_row(); 
          })
      });
  }

  fn label(&self) -> &str {
    self.label.as_str()
  }

  fn show(&mut self) {
    self.showing = true;
  }  
}

impl LightSettings {
  pub fn new() -> LightSettings {
    LightSettings {
      label: String::from("Light Settings"),
      light: 0,
      showing: false,
    }
  }
}