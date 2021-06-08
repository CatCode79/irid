//= MODS ===========================================================================================

pub use controller::CameraController;

pub mod controller;

//= CONSTS =========================================================================================

/**
 * The coordinate system in Wgpu is based on DirectX, and Metal's coordinate systems.
 * That means that in normalized device coordinates the x axis and y axis are in the range
 * of -1.0 to +1.0, and the z axis is 0.0 to +1.0.
 * The cgmath crate (as well as most game math crates) are built for OpenGL's coordinate system.
 * This matrix will scale and translate our scene from OpenGL's coordinate system to WGPU's.
 *
 * Note: We don't explicitly need the OPENGL_TO_WGPU_MATRIX, but models centered on (0, 0, 0) will
 * be halfway inside the clipping area. This is only an issue if you aren't using a camera matrix.
 */
#[rustfmt::skip]
pub(crate) const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);


//= STRUCTS ========================================================================================

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}


//= IMPLS ==========================================================================================

impl Camera {
    pub(crate) fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // The view matrix moves the world to be at the position and rotation of the camera.
        // It's essentially an inverse of whatever the transform matrix of the camera would be.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);

        // The proj matrix wraps the scene to give the effect of depth.
        // Without this, objects up close would be the same size as objects far away.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // 3.
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}
