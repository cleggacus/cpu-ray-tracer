use egui::{Context, Frame, epaint::Shadow, Style};

use crate::{graphics::Graphics};

pub struct GUI {
  settings_open: bool,
  sphere: usize
}

impl GUI {
  pub fn new() -> GUI {
    GUI {
      settings_open: true,
      sphere: 0
    }
  }

  pub fn ui(&mut self, ctx: &Context, fps: i32, graphics: &mut Graphics) {
    let rt = graphics.ray_tracer();

    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        ui.menu_button("View", |ui| {
          if ui.button("Show Settings").clicked() {
            self.settings_open = true;
            ui.close_menu();
          }
        })
      });
    });

    egui::Window::new("Settings")
      .open(&mut self.settings_open)
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
            ui.label("fps");
            ui.label(fps.to_string());
            ui.end_row();
            ui.label("Selected Sphere");
            egui::ComboBox::from_label("")
              .selected_text(format!("{}", self.sphere.to_string()))
              .show_ui(ui, |ui| {
                for i in 0..rt.sphere_len() {
                  ui.selectable_value(&mut self.sphere, i, format!("sphere: {}", i));
                }
              }
            );
            ui.end_row(); 
            ui.label("Radius");
            ui.add(egui::Slider::new(rt.sphere(self.sphere).radius(), 0_f64..=400_f64));
            ui.end_row(); 
            ui.label("Position");
            ui.horizontal(|ui| {
              ui.add(egui::DragValue::new(rt.sphere(self.sphere).center().index_mut(0)));
              ui.add(egui::DragValue::new(rt.sphere(self.sphere).center().index_mut(1)));
              ui.add(egui::DragValue::new(rt.sphere(self.sphere).center().index_mut(2)));
            });
            ui.end_row(); 
            ui.label("Color");
            ui.color_edit_button_srgba(rt.sphere(self.sphere).color());
            ui.end_row(); 
            ui.label("Add Sphere");
            if ui.button("Add!").clicked() {
              rt.add_sphere();
            }
            ui.end_row(); 
            ui.label("Light Direction");
            ui.horizontal(|ui| {
              ui.add(egui::DragValue::new(rt.light_direction().index_mut(0)).clamp_range(-1..=1).speed(0.1));
              ui.add(egui::DragValue::new(rt.light_direction().index_mut(1)).clamp_range(-1..=1).speed(0.1));
              ui.add(egui::DragValue::new(rt.light_direction().index_mut(2)).clamp_range(-1..=1).speed(0.1));
            });
            ui.end_row(); 
            ui.label("Ambient Light");
            ui.add(egui::Slider::new(rt.ambient_light(), 0_f64..=1_f64));
            
          })
      });
    }
}