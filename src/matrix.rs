/**
 * Handles the linear algebra for the rendering. Hoping my math 133 knowledge is enough :)
 * 
 * Author: James Madden
 */
use std::ops::{Add, Sub, Mul, Div};

/**
 * Generic Mat4 type
 */
pub struct Mat4 {
  pub data: [f32; 16]
}

#[derive(Copy, Clone)]
pub struct Vec4 {
  pub data: [f32; 4]
}

impl Mat4 {

  /**
   * creates a new projection matrix
   */
  pub fn projection(near: f32, far: f32, left: f32, right: f32, top: f32, bottom: f32) -> Mat4 {

    // create the matrix!
    Mat4 {
      data: [
        2. * near / (right - left), 0.,                         (right + left) / (right - left), 0.,
        0.,                         2. * near / (top - bottom), (top + bottom) / (top - bottom), 0.,
        0.,                         0.,                         (far + near) / (near - far),     2. * far * near / (near - far),
        0.,                         0.,                         -1.,                             0.
      ]
    }

  }

  /**
   * returns a matrix that delivers the given transformation
   */
  pub fn transformation(trans_x: f32, trans_y: f32, trans_z: f32, rot_x: f32, rot_y: f32, rot_z: f32, scale_x: f32, scale_y: f32, scale_z: f32) -> Mat4 {

    // so because a 4d vector for 3d space is going to have its last value as 1, we can achieve
    // a translation as a linear transformation.

    Mat4 {
      data: [
        scale_x * rot_y.cos() * rot_z.cos(), scale_y * -rot_z.sin(), scale_z * rot_y.sin(), trans_x,
        scale_x * rot_z.sin(), scale_y * rot_x.cos() * rot_z.cos(), scale_z * -rot_x.sin(), trans_y,
        scale_x * -rot_y.sin(), scale_y * rot_x.sin(), scale_z * rot_x.cos() * rot_y.cos(), trans_z,
        0., 0., 0., 1.
      ]
    }

  }

}

impl Vec4 {

  /**
   * creates a point in space
   */
  pub fn point(x: f32, y: f32, z: f32) -> Vec4 {
    Vec4 { data: [x, y, z, 1.] }
  }

}

/**
 * implement the ability to multiply a vector by a scalar
 */
impl Mul<Vec4> for f32 {

  type Output = Vec4;

  fn mul(self, rhs: Vec4) -> Vec4 {

    Vec4 { data: [ rhs.data[0] * self, rhs.data[1] * self, rhs.data[2] * self, rhs.data[3] * self ] }

  }

}

/**
 * implement the ability to multiply a matrix by a vector
 */
impl Mul<Vec4> for Mat4 {

  type Output = Vec4;

  fn mul(self, rhs: Vec4) -> Vec4 {

    Vec4 { data: [ 
      self.data[0] * rhs.data[0] + self.data[1] * rhs.data[1] + self.data[2] * rhs.data[2] + self.data[3] * rhs.data[3],
      self.data[4] * rhs.data[0] + self.data[5] * rhs.data[1] + self.data[6] * rhs.data[2] + self.data[7] * rhs.data[3],
      self.data[8] * rhs.data[0] + self.data[9] * rhs.data[1] + self.data[10] * rhs.data[2] + self.data[11] * rhs.data[3],
      self.data[12] * rhs.data[0] + self.data[13] * rhs.data[1] + self.data[14] * rhs.data[2] +  self.data[15] * rhs.data[3]
     ] }

  }

}