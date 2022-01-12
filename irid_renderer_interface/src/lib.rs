//= CAMERA =========================================================================================

pub trait Camera {
    //- Constructors -------------------------------------------------------------------------------

    /// Create a new camera given the window's width and height
    fn new(width: f32, height: f32) -> Self;

    ///
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32>;

    //- Getters ------------------------------------------------------------------------------------

    ///
    fn eye(&self) -> cgmath::Point3<f32>;

    ///
    fn target(&self) -> cgmath::Point3<f32>;

    ///
    fn up(&self) -> cgmath::Vector3<f32>;

    //- Setters ------------------------------------------------------------------------------------

    fn set_eye(&mut self, value: cgmath::Point3<f32>);

    ///
    fn add_to_eye(&mut self, value: cgmath::Vector3<f32>);

    ///
    fn sub_to_eye(&mut self, value: cgmath::Vector3<f32>);
}
