
use std::time::Instant;

use glium::glutin;
use graphics::Graphics;
use gui::GUI;

mod gui;
mod graphics;
mod raytracer;

fn main() {
  let event_loop = glutin::event_loop::EventLoopBuilder::with_user_event().build();
  let display = create_display(&event_loop);

  let mut egui_glium = egui_glium::EguiGlium::new(&display, &event_loop);

  let mut gui = GUI::new();
  let mut graphics = Graphics::new(&display);

  let mut now = Instant::now();

  event_loop.run(move |event, _, control_flow| {
    let mut redraw = || {
      let elapsed = now.elapsed();
      let fps = 1_f64 / elapsed.as_secs_f64();
      now = Instant::now();
      let repaint_after = egui_glium.run(&display, |ctx| gui.ui(ctx, fps as i32, &mut graphics));

      *control_flow = if repaint_after.is_zero() {
        display.gl_window().window().request_redraw();
        glutin::event_loop::ControlFlow::Poll
      } else {
        glutin::event_loop::ControlFlow::Poll
      };

      {
        let mut target = display.draw();

        graphics.draw(&mut target, &display);
        egui_glium.paint(&display, &mut target);

        target.finish().unwrap();
      }
    };

    match event {
      glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
      glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

      glutin::event::Event::WindowEvent { event, .. } => {
        use glutin::event::WindowEvent;
        if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
          *control_flow = glutin::event_loop::ControlFlow::Exit;
        }

        let event_response = egui_glium.on_event(&event);

        if event_response.repaint {
          display.gl_window().window().request_redraw();
        }
        
      }
      glutin::event::Event::NewEvents(glutin::event::StartCause::ResumeTimeReached {
        ..
      }) => {
        display.gl_window().window().request_redraw();
      }
      _ => (),
    }
  });
}

fn create_display(event_loop: &glutin::event_loop::EventLoop<()>) -> glium::Display {
    let window_builder = glutin::window::WindowBuilder::new()
      .with_resizable(true)
      .with_inner_size(glutin::dpi::LogicalSize {
        width: 768,
        height: 480,
      })
      .with_title("egui_glium example");

    let context_builder = glutin::ContextBuilder::new()
      .with_depth_buffer(0)
      .with_stencil_buffer(0);
      // .with_vsync(true);

    glium::Display::new(window_builder, context_builder, event_loop).unwrap()
}