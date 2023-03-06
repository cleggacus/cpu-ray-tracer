use egui::{Ui, Context};

pub fn combo<T>(ui: &mut Ui, value: &mut T, options: Vec<T>) 
where
  T: ToString + PartialEq
{
  egui::ComboBox::from_id_source(value.to_string())
    .selected_text(value.to_string())
    .show_ui(ui, |ui| {
      for option in options {
        let text = option.to_string();
        ui.selectable_value(value, option, text);
      }
    });
}

pub trait ShowableUI<T> {
  fn ui(&mut self, ctx: &Context, data: T);
  fn label(&self) -> &str;
  fn show(&mut self);
}