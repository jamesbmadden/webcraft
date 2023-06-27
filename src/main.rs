/**
 * Creates a new window and manages events on it
 * 
 * Author: James Madden
 */
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod draw;
mod matrix;
use draw::Draw;

async fn run(event_loop: EventLoop<()>, window: Window) {

  // create an instance of the renderer
  let renderer = Draw::new(&window).await;

  event_loop.run(move | event, _, control_flow | {

    *control_flow = ControlFlow::Wait;

    match event {
      Event::RedrawRequested(_) => {
        renderer.draw();
      }

      Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,
      _ => {}
    }

  });

}

fn main() {
  
  // create a new window
  let event_loop = EventLoop::new();
  let window = Window::new(&event_loop).unwrap();

  // add web support in the future
  {
    env_logger::init();
    pollster::block_on(run(event_loop, window));
  }

}