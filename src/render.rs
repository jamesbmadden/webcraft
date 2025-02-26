/**
* manage the renderer
*/
use std::borrow::Cow;

use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

use crate::camera::Camera;
use crate::texture::Texture;

// vertices for a cube
const VERT: [f32; 192] = [
  // pos              tx coords  normal
  // front
  -1.0, -1.0,  1.0,   1.0, 1.0,   0.0,  0.0, -1.0, // bottom left
   1.0, -1.0,  1.0,   0.0, 1.0,   0.0,  0.0, -1.0, // bottom right
   1.0,  1.0,  1.0,   0.0, 0.0,   0.0,  0.0, -1.0, // top right
  -1.0,  1.0,  1.0,   1.0, 0.0,   0.0,  0.0, -1.0, // top left
  // back
  -1.0, -1.0, -1.0,   1.0, 1.0,   0.0,  0.0,  1.0, // bottom left
   1.0, -1.0, -1.0,   0.0, 1.0,   0.0,  0.0,  1.0, // bottom right
   1.0,  1.0, -1.0,   0.0, 0.0,   0.0,  0.0,  1.0, // top right
  -1.0,  1.0, -1.0,   1.0, 0.0,   0.0,  0.0,  1.0, // top left
  // bottom
  -1.0, -1.0, -1.0,   1.0, 0.0,   0.0, -1.0,  0.0, // back left
  -1.0, -1.0,  1.0,   0.0, 0.0,   0.0, -1.0,  0.0, // front left
   1.0, -1.0,  1.0,   0.0, 1.0,   0.0, -1.0,  0.0, // front right
   1.0, -1.0, -1.0,   1.0, 1.0,   0.0, -1.0,  0.0, // back right
   // left
  -1.0, -1.0, -1.0,   0.0, 1.0,  -1.0,  0.0,  0.0, // bottom back
  -1.0,  1.0, -1.0,   0.0, 0.0,  -1.0,  0.0,  0.0, // top back
  -1.0, -1.0,  1.0,   1.0, 1.0,  -1.0,  0.0,  0.0, // bottom front
  -1.0,  1.0,  1.0,   1.0, 0.0,  -1.0,  0.0,  0.0, // top front
  // right
   1.0,  1.0, -1.0,   1.0, 0.0,   1.0,  0.0,  0.0, // top back
   1.0, -1.0, -1.0,   1.0, 1.0,   1.0,  0.0,  0.0, // bottom back
   1.0,  1.0,  1.0,   0.0, 0.0,   1.0,  0.0,  0.0, // top front
   1.0, -1.0,  1.0,   0.0, 1.0,   1.0,  0.0,  0.0, // bottom front
  // top
   1.0,  1.0,  1.0,   1.0, 1.0,   0.0,  1.0,  0.0, // front right
  -1.0,  1.0,  1.0,   0.0, 1.0,   0.0,  1.0,  0.0, // front left
   1.0,  1.0, -1.0,   1.0, 0.0,   0.0,  1.0,  0.0, // back right
  -1.0,  1.0, -1.0,   0.0, 0.0,   0.0,  1.0,  0.0, // back left


];

const INDEX: [u16; 36] = [ 

  // back face
  4, 5, 7,
  5, 6, 7,

  // bottom face
  8, 9, 10,
  8, 10, 11,

  // left face
  12, 13, 14,
  13, 15, 14,

  // right face
  16, 17, 18,
  17, 19, 18,

  // top face
  20, 21, 22,
  21, 23, 22,

  // front face
  0, 1, 2,
  0, 2, 3

];

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
  view_proj: [[f32; 4]; 4]
}

impl Uniforms {
  fn new() -> Self {
    use cgmath::SquareMatrix;
    Self {
      view_proj: cgmath::Matrix4::identity().into(),
    }
  }

  fn update_view_proj(&mut self, camera: &Camera) {
    self.view_proj = camera.build_view_projection_matrix().into();
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
  pub pos: [i32; 3],
  pub block: u32
}

pub struct Render<'a> {
  
  surface: wgpu::Surface<'a>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  pipeline: wgpu::RenderPipeline,
  config: wgpu::SurfaceConfiguration,
  vbuf: wgpu::Buffer,
  ibuf: wgpu::Buffer,
  isize: u32,
  instbuf: wgpu::Buffer,
  instsize: u32,
  ubuf: wgpu::Buffer,
  ubg: wgpu::BindGroup,
  tbg: wgpu::BindGroup,
  uniforms: Uniforms,
  texture: Texture,
  depth_texture: Texture,
  pub window: &'a winit::window::Window
  
}

impl<'a> Render<'a> {
  
  /**
  * create a new instance of render
  */
  pub async fn new (window: &'a winit::window::Window, camera: &mut Camera, instances: Vec<Instance>) -> Render<'a> {
    
    // create the renderer
    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

    camera.set_aspect(size.width as f32, size.height as f32);
    
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());
    
    let surface = instance.create_surface(window).unwrap();
    let adapter = instance
    .request_adapter(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::default(),
      force_fallback_adapter: false,
      // Request an adapter which can render to our surface
      compatible_surface: Some(&surface),
    })
    .await
    .expect("Failed to find an appropriate adapter");
    
    // Create the logical device and command queue
    let (device, queue) = adapter
    .request_device(
      &wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
        required_limits: wgpu::Limits::downlevel_webgl2_defaults()
        .using_resolution(adapter.limits()),
        memory_hints: wgpu::MemoryHints::MemoryUsage,
      },
      None,
    )
    .await
    .expect("Failed to create device");

    // load the texture
    let tx_bytes = include_bytes!("textures/moss.png");
    let texture = Texture::from_bytes(&device, &queue, tx_bytes, "moss.png").unwrap();

    let tbg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
          },
          count: None,
        },
        wgpu::BindGroupLayoutEntry {
          binding: 1,
          visibility: wgpu::ShaderStages::FRAGMENT,
          // This should match the filterable field of the
          // corresponding Texture entry above.
          ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
          count: None,
        },
      ],
      label: Some("texture_bind_group_layout"),
    });

    let tbg = device.create_bind_group(
    &wgpu::BindGroupDescriptor {
      layout: &tbg_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&texture.view), // CHANGED!
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&texture.sampler), // CHANGED!
        }
      ],
      label: Some("texture_bind_group"),
    }
  );
    
    // Load the shaders from disk
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    // create the uniforms
    let mut uniforms = Uniforms::new();
    uniforms.update_view_proj(camera);

    let ubuf = device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Unfirom Buffer"),
        contents: bytemuck::cast_slice(&[uniforms]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      }
    );

    let ubg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::VERTEX,
          ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        }
      ],
      label: Some("uniforms_bind_group_layout"),
    });

    let ubg = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &ubg_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: ubuf.as_entire_binding(),
        }
      ],
      label: Some("uniform_bind_group"),
    });
    
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: None,
      bind_group_layouts: &[
        &ubg_layout,
        &tbg_layout
      ],
      push_constant_ranges: &[],
    });
    
    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];
    
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: None,
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: Some("vs_main"),
        buffers: &[
          // vertex buffer layout
          wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
              wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
              },
              wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x2,
              },
              wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                shader_location: 2,
                format: wgpu::VertexFormat::Float32x3,
              },
            ],
          },
          // instance buffer layout
          wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
              wgpu::VertexAttribute {
                offset: 0,
                shader_location: 5,
                format: wgpu::VertexFormat::Sint32x3,
              },
              wgpu::VertexAttribute {
                offset: std::mem::size_of::<[i32; 3]>() as wgpu::BufferAddress,
                shader_location: 6,
                format: wgpu::VertexFormat::Uint32,
              },
            ],
          }
        ],
        compilation_options: Default::default(),
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: Some("fs_main"),
        compilation_options: Default::default(),
        targets: &[Some(swapchain_format.into())],
      }),
      primitive: wgpu::PrimitiveState::default(),
      depth_stencil: Some(wgpu::DepthStencilState {
        format: Texture::DEPTH_FORMAT,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default()
      }),
      multisample: wgpu::MultisampleState::default(),
      multiview: None,
      cache: None,
    });

    let vbuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Vertex Buffer"),
      contents: bytemuck::cast_slice(&VERT),
      usage: wgpu::BufferUsages::VERTEX,
    });
    let ibuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Index Buffer"),
      contents: bytemuck::cast_slice(&INDEX),
      usage: wgpu::BufferUsages::INDEX,
    });
    let instbuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Instance Buffer"),
      contents: bytemuck::cast_slice(&instances),
      usage: wgpu::BufferUsages::VERTEX,
    });

    let isize = INDEX.len() as u32;
    let instsize = instances.len() as u32;
    
    let config = surface
    .get_default_config(&adapter, size.width, size.height)
    .unwrap();
    surface.configure(&device, &config);

    // create depth texture
    let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");
    
    Render { surface, device, queue, pipeline: render_pipeline, config, vbuf, ibuf, isize, instbuf, instsize, ubuf, ubg, tbg, uniforms, texture, depth_texture, window }
    
  }
  
  /**
  * resize the surface to be drawn to
  */
  pub fn resize (&mut self, new_size: PhysicalSize<u32>) {
    
    // Reconfigure the surface with the new size
    self.config.width = new_size.width.max(1);
    self.config.height = new_size.height.max(1);
    self.surface.configure(&self.device, &self.config);
    self.depth_texture = Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
    // On macos the window needs to be redrawn manually after resizing
    self.window.request_redraw();
    
  }

  /**
   * update the uniforms with the new data
   */
  pub fn update_camera (&mut self, camera: &Camera) {
    self.uniforms.update_view_proj(camera);
    self.queue.write_buffer(&self.ubuf, 0, bytemuck::cast_slice(&[self.uniforms]));
  }

  pub fn update_instances (&mut self, instances: Vec<Instance>) {

  }
  
  /**
  * draw the scene to the screen
  */
  pub fn render (&mut self) {
    let frame = self.surface.get_current_texture()
    .expect("Failed to acquire next swap chain texture");
    let view = frame
    .texture
    .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: None,
    });
    {
      let mut rpass =
      encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
              r: 0.47,
              g: 0.65,
              b: 1.0,
              a: 1.0
            }),
            store: wgpu::StoreOp::Store,
          },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
          view: &self.depth_texture.view,
          depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.0),
            store: wgpu::StoreOp::Store,
          }),
          stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
      });

      rpass.set_pipeline(&self.pipeline);
      rpass.set_bind_group(0, &self.ubg, &[]);
      rpass.set_bind_group(1, &self.tbg, &[]);
      rpass.set_vertex_buffer(0, self.vbuf.slice(..));
      rpass.set_vertex_buffer(1, self.instbuf.slice(..));
      rpass.set_index_buffer(self.ibuf.slice(..), wgpu::IndexFormat::Uint16);
      rpass.draw_indexed(0..self.isize, 0, 0..self.instsize);
    }
    
    self.queue.submit(Some(encoder.finish()));
    frame.present();
  }
  
}