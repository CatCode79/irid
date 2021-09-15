
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

pub struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    /// Create a new camera given the window's width and height
    pub fn new(width: f32, height: f32) -> Self {
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

    ///
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // The view matrix moves the world to be at the position and rotation of the camera.
        // It's essentially an inverse of whatever the transform matrix of the camera would be.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);

        // The proj matrix wraps the scene to give the effect of depth.
        // Without this, objects up close would be the same size as objects far away.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    /// Create a new CameraMetadatas from this camera.
    pub fn create_metadatas(&self, device: &crate::renderer::Device) -> CameraMetadatas {
        let wgpu_device = device.expose_wgpu_device();

        let mut uniform = CameraUniform::new();
        uniform.update_view_proj(self);

        use wgpu::util::DeviceExt;
        let buffer = wgpu_device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[uniform.clone()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group_layout = wgpu_device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
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
                label: Some("Camera Bind Group Layout"),
            }
        );

        let bind_group = wgpu_device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    }
                ],
                label: Some("Camera Bind Group"),
            }
        );

        CameraMetadatas {
            uniform,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
}


//= CAMERA UNIFORM BUFFER ==========================================================================

// We need those to store our data correctly for the shaders and the buffer
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    #[inline]
    pub(crate) fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}


//= CAMERA METADATAS ===============================================================================

pub struct CameraMetadatas {
    uniform: CameraUniform,
    buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl CameraMetadatas {
    #[inline(always)]
    pub fn uniform(&self) -> &CameraUniform {
        &self.uniform
    }

    #[inline(always)]
    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    #[inline(always)]
    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    #[inline(always)]
    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}


//= CAMERA CONTROLLER ==============================================================================

///
pub struct CameraController {
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}


impl CameraController {
    ///
    pub fn new(speed: f32) -> Self {
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
    pub fn process_events(&mut self, input: &winit::event::KeyboardInput) -> bool {
        match input {
            &winit::event::KeyboardInput {
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

    pub fn update_camera(&self, camera: &mut crate::renderer::Camera) {
        use cgmath::InnerSpace;

        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when camera gets too close to the center of the scene
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the up/ down is pressed
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and eye so that it doesn't change.
            // The eye therefore still lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
