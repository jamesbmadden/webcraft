mod render;
mod camera;
mod texture;
mod world;
use winit::{
  event::{Event, WindowEvent},
  event_loop::EventLoop,
  window::Window,
};

async fn run(event_loop: EventLoop<()>, window: Window) {

  // create the world
  let mut world = world::World::test();
  // generate the instances
  let instances = world.gen_instances();
  
  let mut camera = camera::Camera::new();
  let mut render = render::Render::new(&window, &mut camera, instances).await;

  event_loop.run(move |event, target| {
    // Have the closure take ownership of the resources.
    
    if let Event::WindowEvent {
      window_id: _,
      event,
    } = event
    {
      match event {
        WindowEvent::Resized(new_size) => {
          
          render.resize(new_size);
          
        }
        WindowEvent::RedrawRequested => {
          camera.update();
          render.update_camera(&camera);
          render.render();
          render.window.request_redraw();
        }
        WindowEvent::CloseRequested => target.exit(),
        _ => {}
      };
    }
  })
  .unwrap();
}

pub fn main() {
  let event_loop = EventLoop::new().unwrap();
  #[cfg_attr(
    not(target_arch = "wasm32"),
    expect(unused_mut, reason = "`wasm32` re-assigns to specify canvas")
  )]
  let mut builder = winit::window::WindowBuilder::new();
  #[cfg(target_arch = "wasm32")]
  {
    use wasm_bindgen::JsCast;
    use winit::platform::web::WindowBuilderExtWebSys;
    let canvas = web_sys::window()
    .unwrap()
    .document()
    .unwrap()
    .get_element_by_id("canvas")
    .unwrap()
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .unwrap();
    builder = builder.with_canvas(Some(canvas));
  }
  let window = builder.with_title("Cube").build(&event_loop).unwrap();
  
  #[cfg(not(target_arch = "wasm32"))]
  {
    env_logger::init();
    pollster::block_on(run(event_loop, window));
  }
  #[cfg(target_arch = "wasm32")]
  {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");
    wasm_bindgen_futures::spawn_local(run(event_loop, window));
  }
}
