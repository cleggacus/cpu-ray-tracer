use std::collections::HashSet;

use glium::glutin::event::{VirtualKeyCode, Event, ElementState, WindowEvent, MouseButton};

pub struct EventManager {
  keys_down: HashSet<VirtualKeyCode>,
  left_mouse_down: bool,
  mouse_position: (f64, f64),
  mouse_move: (f64, f64),
}

impl EventManager {
  pub fn new() -> EventManager {
    EventManager {
      keys_down: HashSet::new(),
      left_mouse_down: false,
      mouse_position: (0.0, 0.0),
      mouse_move: (0.0, 0.0),
    }
  }

  pub fn is_key_down(&self, key: VirtualKeyCode) -> bool {
    self.keys_down.contains(&key)
  }

  pub fn is_left_mouse_down(&self) -> bool {
    self.left_mouse_down
  }

  pub fn mouse_move(&self) -> (f64, f64) {
    self.mouse_move
  }

  pub fn update(&mut self, event: &Event<()>, consumed: bool) {
    self.mouse_move = (0.0, 0.0);
    
    if consumed {
      self.left_mouse_down = false;
    }

    match event {
      Event::WindowEvent { event, .. } => {

        match event {
          WindowEvent::KeyboardInput { input, .. } => {
            if let Some(key) = input.virtual_keycode {
              if input.state == ElementState::Pressed {
                self.keys_down.insert(key);
              } else {
                self.keys_down.remove(&key);
              }
            }
          },
          WindowEvent::CursorMoved { position, .. } => {
            let (x, y) = self.mouse_position;

            self.mouse_move = (
              x - position.x,
              y - position.y,
            );

            self.mouse_position = (
              position.x,
              position.y,
            );
          },
          WindowEvent::MouseInput { state, button, .. } => {
            if *button == MouseButton::Left {
              self.left_mouse_down = *state == ElementState::Pressed;
            }
          },
          _ => {}
        }
      }
      _ => {}
    }
  }
}