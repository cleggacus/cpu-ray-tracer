use egui::{Style, epaint::Shadow, Frame};

use crate::world::{World, Object, Plane, Sphere};

use super::utils::{ShowableUI, combo};

pub struct ObjectSettings {
  showing: bool,
  object: usize,
  label: String,
}

impl ShowableUI<&mut World> for ObjectSettings {
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
            let objects = world.objects_mut();

            if self.object >= objects.len() {
              self.object = 0;
            }

            if objects.len() > 0 {
              ui.label("Selected Object");

              combo(ui, &mut self.object, 
                (0..objects.len()).collect::<Vec<usize>>()
              );

              ui.end_row(); 

              let material = match &mut objects[self.object] {
                Object::Sphere(sphere) => {
                  ui.label("Radius");
                  ui.add(egui::Slider::new(&mut sphere.radius, 0_f64..=500_f64).step_by(0.1));
                  ui.end_row(); 
                  ui.label("Position");
                  ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(sphere.position.x_mut()));
                    ui.add(egui::DragValue::new(sphere.position.y_mut()));
                    ui.add(egui::DragValue::new(sphere.position.z_mut()));
                  });

                  &mut sphere.material
                },
                Object::Plane(plane) => {
                  ui.label("Height");
                  ui.add(egui::Slider::new(&mut plane.height, 0_f64..=500_f64).step_by(0.1));
                  ui.end_row(); 
                  ui.label("Width");
                  ui.add(egui::Slider::new(&mut plane.width, 0_f64..=500_f64).step_by(0.1));
                  ui.end_row(); 
                  ui.label("Position");
                  ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(plane.position.x_mut()));
                    ui.add(egui::DragValue::new(plane.position.y_mut()));
                    ui.add(egui::DragValue::new(plane.position.z_mut()));
                  });

                  &mut plane.material
                },
              };

              ui.end_row();

              ui.label("Color");
              ui.color_edit_button_rgb(&mut material.color);

              ui.end_row();
              ui.label("Reflectivity");
              ui.add(
                egui::DragValue::new(&mut material.reflectivity)
                  .clamp_range(0..=1)
                  .speed(0.05)
              );

              ui.end_row();
              ui.label("Refracive Index");
              ui.add(
                egui::DragValue::new(&mut material.refractive_index)
                  .clamp_range(0..=3)
                  .speed(0.05)
              );

              ui.end_row();
              ui.label("Transparency");
              ui.add(
                egui::DragValue::new(&mut material.transparency)
                  .clamp_range(0..=1)
                  .speed(0.05)
              );

              ui.end_row();
              ui.label("Ambient Reflection");
              ui.add(
                egui::DragValue::new(&mut material.ambient_reflection)
                  .clamp_range(0..=100)
                  .speed(0.05)
              );

              ui.end_row();
              ui.label("Diffuse Reflection");
              ui.add(
                egui::DragValue::new(&mut material.diffuse_reflection)
                  .clamp_range(0..=100)
                  .speed(0.05)
              );

              ui.end_row();
              ui.label("Specular Reflection");
              let specular_label = if material.has_specular { "on" } else {"off"};
              ui.toggle_value(&mut material.has_specular, specular_label);
              ui.end_row();

              if material.has_specular {
                ui.label("Specular Amount");
                ui.add(
                  egui::DragValue::new(&mut material.specular_reflection)
                    .clamp_range(0..=100)
                    .speed(1.0)
                );
              } 

              ui.end_row(); 

              ui.label("Remove Object");
              if ui.button("Remove").clicked() {
                objects.remove(self.object);
              }
              ui.end_row(); 
            }

            ui.label("Add Sphere");
            if ui.button("Add").clicked() {
              world.objects_mut().push(
                Object::Sphere(Sphere::new())
              );
            }
            ui.end_row(); 

            ui.label("Add Plane");
            if ui.button("Add").clicked() {
              world.objects_mut().push(
                Object::Plane(Plane::new())
              );
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

impl ObjectSettings {
  pub fn new() -> ObjectSettings {
    ObjectSettings {
      label: String::from("Object Settings"),
      object: 0,
      showing: false,
    }
  }
}