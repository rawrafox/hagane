use super::*;

impl float3 {
  #[inline]
  pub fn cross(self, other: float3) -> float3 {
    let result = float3(self.2, self.0, self.1) * other - self * float3(other.2, other.0, other.1);

    return float3(result.2, result.0, result.1);
  }

  #[inline]
  pub fn look_at(self, target: float3, up: float3) -> float4x4 {
    let z = (target - self).normalize();
    let x = up.cross(z).normalize();
    let y = z.cross(x);
    
    return float4x4(
      float4(x.0, y.0, z.0, 0.0),
      float4(x.1, y.1, z.1, 0.0),
      float4(x.2, y.2, z.2, 0.0),
      float4(x.dot(self), y.dot(self), z.dot(self), 1.0),
    );
  }
}

impl float4x4 {
  #[inline]
  pub fn from_euler_angles(roll: f32, pitch: f32, yaw: f32) -> float4x4 {
    let (sr, cr) = roll.sin_cos();
    let (sp, cp) = pitch.sin_cos();
    let (sy, cy) = yaw.sin_cos();

    return float4x4(
      float4(cy * cp, sy * cp, -sp, 0.0),
      float4(cy * sp * sr - sy * cr, sy * sp * sr + cy * cr, cp * sr, 0.0),
      float4(cy * sp * cr + sy * sr, sy * sp * cr - cy * sr, cp * cr, 0.0),
      float4(0.0, 0.0, 0.0, 1.0)
    );
  }
  
  #[inline]
  pub fn from_rotation(angle: f32, axis: float3) -> float4x4 {
    let float3(x, y, z) = axis.normalize();

    let cos = angle.cos();
    let cosp = 1.0 - cos;
    let sin = angle.sin();

    return float4x4(
      float4(
        x * x + cos * (1.0 - x * x),
        x * y * cosp - z * sin,
        x * y * cosp - y * sin,
        0.0
      ),
      float4(
        x * y * cosp - z * sin,
        y * y + cos * (1.0 - y * y),
        y * z * cosp - x * sin,
        0.0
      ),
      float4(
        x * z * cosp - y * sin,
        y * z * cosp - x * sin,
        z * z + cos * (1.0 - z * z),
        0.0
      ),
      float4(
        0.0,
        0.0,
        0.0,
        1.0
      )
    );
  }

  #[inline]
  pub fn from_scale(scale: f32) -> float4x4 {
    let mut matrix = scale * matrix_identity_float4x4;

    matrix.3 = float4(0.0, 0.0, 0.0, 1.0);

    return matrix;
  }

  #[inline]
  pub fn from_translation(x: f32, y: f32, z: f32) -> float4x4 {
    let mut matrix = matrix_identity_float4x4;

    matrix.3 = float4(x, y, z, 1.0);

    return matrix;
  }

  #[inline]
  pub fn perspective(aspect_ratio: f32, fov_y: f32, z_near: f32, z_far: f32) -> float4x4 {
    let fov = 1.0 / ((fov_y / 2.0).tan() * aspect_ratio);
    let distance = z_near - z_far;

    let x = (z_near + z_far) / distance;
    let y = 2.0 * z_near * z_far / distance;

    return float4x4(
      float4(fov, 0.0, 0.0,  0.0),
      float4(0.0, fov, 0.0,  0.0),
      float4(0.0, 0.0,   x, -1.0),
      float4(0.0, 0.0,   y,  0.0)
    );
  }
}
