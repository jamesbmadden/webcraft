use std::f32::consts::PI;

/*
 * provides a struct managing the camera
 */
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 0.5, 0.5,
  0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
  eye: cgmath::Point3<f32>,
  target: cgmath::Point3<f32>,
  up: cgmath::Vector3<f32>,
  aspect: f32,
  fovy: f32,
  znear: f32,
  zfar: f32,
  create_time: std::time::SystemTime
}

impl Camera {

  /**
   * create a default instance of camera
   */
  pub fn new() -> Camera {

    Camera {
      // position the camera 1 unit up and 2 units back
      // +z is out of the screen
      eye: (0.0, 1.0, 2.0).into(),
      // have it look at the origin
      target: (16.0, -128.0, 16.0).into(),
      // which way is "up"
      up: cgmath::Vector3::unit_y(),
      aspect: 400.0 / 300.0,
      fovy: 45.0,
      znear: 0.1,
      zfar: 100.0,
      create_time: std::time::SystemTime::now()
    }

  }

  /**
   * update the aspect ratio
   */
  pub fn set_aspect(&mut self, width: f32, height: f32) {
    self.aspect = width / height;
  }

  /**
   * make the camera spin around the origin
   */
  pub fn update(&mut self) {
    let radius = 32.0;
    let time = std::time::SystemTime::now()
      .duration_since(self.create_time)
      .unwrap()
      .as_millis() as f32 / 50.0;
    let x = (time * PI / 180.0).sin() * radius;
    let z = (time * PI / 180.0).cos() * radius;

    self.eye = cgmath::Point3::new(x + 16.0, -120.0, z + 16.0);
  }

  /**
   * calculate the view projection matrix for rendering
   */
  pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {

    let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
    let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

    return OPENGL_TO_WGPU_MATRIX * proj * view;

  }
}