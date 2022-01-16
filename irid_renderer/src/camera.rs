//= USES ===========================================================================================

use irid_renderer_interface::Camera;

//= CONSTS =========================================================================================

/// The coordinate system in Wgpu is based on DirectX, and Metal's coordinate systems.
/// That means that in normalized device coordinates the x axis and y axis are in the range
/// of -1.0 to +1.0, and the z axis is 0.0 to +1.0.
/// The cgmath crate (as well as most game math crates) are built for OpenGL's coordinate system.
/// This matrix will scale and translate our scene from OpenGL's coordinate system to WGPU's.
/// Note: We don't explicitly need the OPENGL_TO_WGPU_MATRIX, but models centered on (0, 0, 0) will
/// be halfway inside the clipping area. This is only an issue if you aren't using a camera matrix.
#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

//= CAMERA =========================================================================================

///
#[derive(Debug, Clone)]
pub struct PerspectiveCamera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera for PerspectiveCamera {
    //- Constructors -------------------------------------------------------------------------------

    fn new(width: f32, height: f32) -> Self {
        Self {
            // position the camera one unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 1.0, 2.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: width / height,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    //- Camera Uniform Helpers ---------------------------------------------------------------------

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // The view matrix moves the world to be at the position and rotation of the camera.
        // It's essentially an inverse of whatever the transform matrix of the camera would be.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);

        // The proj matrix wraps the scene to give the effect of depth.
        // Without this, objects up close would be the same size as objects far away.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    //- Getters ------------------------------------------------------------------------------------

    #[inline]
    fn eye(&self) -> cgmath::Point3<f32> {
        self.eye
    }

    #[inline]
    fn target(&self) -> cgmath::Point3<f32> {
        self.target
    }

    #[inline]
    fn up(&self) -> cgmath::Vector3<f32> {
        self.up
    }

    //- Setters ------------------------------------------------------------------------------------

    #[inline]
    fn set_eye(&mut self, value: cgmath::Point3<f32>) {
        self.eye = value;
    }

    #[inline]
    fn add_to_eye(&mut self, value: cgmath::Vector3<f32>) {
        self.eye += value;
    }

    #[inline]
    fn sub_to_eye(&mut self, value: cgmath::Vector3<f32>) {
        self.eye -= value;
    }
}

//= CAMERA CONTROLLER ==============================================================================

///
#[derive(Clone, Debug)]
pub(crate) struct CameraController {
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    //- Constructors -------------------------------------------------------------------------------

    ///
    pub(crate) fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    ///
    pub(crate) fn process_events(&mut self, input: winit::event::KeyboardInput) -> bool {
        match input {
            winit::event::KeyboardInput {
                state,
                virtual_keycode: Some(keycode),
                ..
            } => {
                let is_pressed = state == winit::event::ElementState::Pressed;
                match keycode {
                    winit::event::VirtualKeyCode::Space => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    winit::event::VirtualKeyCode::LShift => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    winit::event::VirtualKeyCode::W | winit::event::VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    winit::event::VirtualKeyCode::A | winit::event::VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    winit::event::VirtualKeyCode::S | winit::event::VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    winit::event::VirtualKeyCode::D | winit::event::VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    ///
    pub(crate) fn update_camera<C: Camera>(&self, camera: &mut C) {
        use cgmath::InnerSpace;

        let forward = camera.target() - camera.eye();
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when camera gets too close to the center of the scene
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.add_to_eye(forward_norm * self.speed);
        }
        if self.is_backward_pressed {
            camera.sub_to_eye(forward_norm * self.speed);
        }

        let right = forward_norm.cross(camera.up());

        // Redo radius calc in case the up/ down is pressed
        let forward = camera.target() - camera.eye();
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and eye so that it doesn't change.
            // The eye therefore still lies on the circle made by the target and eye.
            camera.set_eye(
                camera.target() - (forward + right * self.speed).normalize() * forward_mag,
            );
        }
        if self.is_left_pressed {
            camera.set_eye(
                camera.target() - (forward - right * self.speed).normalize() * forward_mag,
            );
        }
    }
}
