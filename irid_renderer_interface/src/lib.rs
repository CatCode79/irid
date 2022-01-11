//= USES ===========================================================================================

//= CAMERA =========================================================================================

pub trait Camera {
    /// Create a new camera given the window's width and height
    fn new(width: f32, height: f32) -> Self;
}
