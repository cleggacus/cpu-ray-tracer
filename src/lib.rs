#![feature(slice_flatten)]

use egui_glium::EguiGlium;
use event_manager::EventManager;
use glium::{glutin::{self, event::{WindowEvent, Event}, event_loop::{ControlFlow, EventLoopBuilder}, dpi::LogicalSize, window::WindowBuilder, ContextBuilder}};
use graphics::Graphics;
use gui::GUI;

mod gui;
mod graphics;
mod world;
mod renderer;
mod vector;
mod event_manager;
mod quaternion;

pub fn run() {
  let event_loop = EventLoopBuilder::with_user_event().build();
  let display = create_display(&event_loop);

  let mut egui_glium = EguiGlium::new(&display, &event_loop);

  let mut gui = GUI::new();
  let mut graphics = Graphics::new(&display);

  let mut event_manager = EventManager::new();

  let mut consumed = false;

  event_loop.run(move |event, _, control_flow| {
    event_manager.update(&event, consumed);
    graphics.world().update(&event_manager);

    let mut redraw = || {
      let mut target = display.draw();

      egui_glium.run(&display, |ctx| gui.ui(ctx, graphics.world()));

      graphics.world().update(&event_manager);
      graphics.draw(&mut target, &display);

      egui_glium.paint(&display, &mut target);

      display.gl_window().window().request_redraw();

      target.finish().unwrap();
    };

    match &event {
      Event::RedrawEventsCleared if cfg!(windows) => redraw(),
      Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

      Event::WindowEvent { event, .. } => {
        if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
          *control_flow = ControlFlow::Exit;
        }

        let event_response = egui_glium.on_event(&event);
        consumed = event_response.consumed;

        if event_response.repaint {
          display.gl_window().window().request_redraw();
        }
      },
      _ => (),
    }
  });
}

fn create_display(event_loop: &glutin::event_loop::EventLoop<()>) -> glium::Display {
  let window_builder = WindowBuilder::new()
    .with_resizable(true)
    .with_inner_size(LogicalSize {
      width: 640,
      height: 640,
    })
    .with_title("CPU Ray Tracer");

  let context_builder = ContextBuilder::new();

  glium::Display::new(window_builder, context_builder, event_loop).unwrap()
}