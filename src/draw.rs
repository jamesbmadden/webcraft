/**
 * Manages the wgpu rendering
 * 
 * Author: James Madden
 */
use winit::window::Window;
use std::borrow::Cow;

pub struct Draw {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  render_pipeline: wgpu::RenderPipeline,
  config: wgpu::SurfaceConfiguration
}

impl Draw {

  /**
   * Create a new instance of the renderer.
   */
  pub async fn new(window: &Window) -> Self {

    let size = window.inner_size();

    // create an instance of wpgu!
    let instance = wgpu::Instance::default();
  
    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::default(),
      force_fallback_adapter: false,
      // request an adapter compatible with the surface
      compatible_surface: Some(&surface)
    }).await.expect("Failed to find an appropriate adapter");
  
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
      label: None,
      features: wgpu::Features::empty(),
      limits: wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())
    }, None).await.expect("Failed to create device");
  
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/shader.wgsl")))
    });
  
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: None,
      bind_group_layouts: &[],
      push_constant_ranges: &[]
    });
  
    let sc_capabilities = surface.get_capabilities(&adapter);
    let sc_format = sc_capabilities.formats[0];
  
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: None,
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[]
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[Some(sc_format.into())]
      }),
      primitive: wgpu::PrimitiveState::default(),
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default(),
      multiview: None
    });
  
    let mut config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: sc_format,
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: sc_capabilities.alpha_modes[0],
      view_formats: vec![]
    };
  
    surface.configure(&device, &config);

    Draw { surface, device, queue, render_pipeline, config }

  }

  /**
   * Draw to the screen
   */
  pub fn draw(&self) {
    let frame = self.surface.get_current_texture().expect("Failed to acquire next swap chain texture");
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
      // create the render pass
      let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations { 
            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN), 
            store: true 
          }
        })],
        depth_stencil_attachment: None
      });
      rp.set_pipeline(&self.render_pipeline);
      rp.draw(0..3, 0..1);
    }

    self.queue.submit(Some(encoder.finish()));
    frame.present();
  }

}