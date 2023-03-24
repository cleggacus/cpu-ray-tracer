use egui::{Style, epaint::Shadow, Frame};

use crate::world::{World, CameraType};

use super::utils::{ShowableUI, combo};

pub struct CameraSettings {
  showing: bool,
  label: String,
}

impl ShowableUI<&mut World> for CameraSettings {
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
            let camera_type = &mut world.camera_mut().camera_info_mut().camera_type;

            ui.label("Camera Type");

            combo(ui, camera_type, 
              vec![
                CameraType::Perspective,
                CameraType::Orthographic,
              ]
            );

            ui.end_row(); 

            match camera_type {
              CameraType::Perspective => {
                ui.label("FOV");
                ui.add(
                  egui::DragValue::new(&mut world.camera_mut().camera_info_mut().vertical_fov)
                    .clamp_range(0.01..=100.0)
                    .speed(0.1)
                );
                ui.end_row(); 
              },
              CameraType::Orthographic => {
                ui.label("Camera Height");
                ui.add(
                  egui::DragValue::new(&mut world.camera_mut().camera_info_mut().camera_height)
                    .clamp_range(0.01..=100.0)
                    .speed(0.1)
                );
                ui.end_row(); 
              }
            }

            ui.label("Miss Color");
            ui.color_edit_button_rgb(&mut world.camera_mut().camera_info_mut().miss_color);
            ui.end_row(); 

            ui.label("Viewport");
            ui.horizontal(|ui| {
              ui.add(
                egui::DragValue::new(&mut world.camera_mut().camera_info_mut().viewport_width)
                  .clamp_range(1..=2000)
                  .speed(1)
              );

              ui.add(
                egui::DragValue::new(&mut world.camera_mut().camera_info_mut().viewport_height)
                  .clamp_range(1..=2000)
                  .speed(1)
              );
            });

            ui.end_row(); 

            ui.label("Camera Speed");
            ui.add(
              egui::DragValue::new(world.camera_mut().speed_mut())
                .clamp_range(0..=10)
                .speed(0.01)
            );

            ui.end_row(); 

            ui.label("Ray Depth");
            ui.add(
              egui::DragValue::new(world.camera_mut().depth_mut())
                .clamp_range(0..=100)
                .speed(1)
            );
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

impl CameraSettings {
  pub fn new() -> CameraSettings {
    CameraSettings {
      label: String::from("Camera Settings"),
      showing: false,
    }
  }
}