//= USES ===========================================================================================

use winit::{
	window::Window,
	event::WindowEvent,
};


//= CONSTS =========================================================================================

/**
 We arrange the vertices in counter clockwise order: top, bottom left, bottom right.
 We do it this way partially out of tradition, but mostly because we specified
 in the rasterization_state of the render_pipeline that we want the front_face of our triangle
 to be wgpu::FrontFace::Ccw so that we cull the back face.
 This means that any triangle that should be facing us should have its vertices
 in counter clockwise order.
 */
pub(crate) const VERTICES: &[irid::vertex::Vertex] = &[
	irid::vertex::Vertex { position: [-0.08682410,  0.49240386, 0.0], /*color: [0.10, 0.0, 0.50],*/ tex_coords: [0.4131759000, 0.007596140] },  // 0
	irid::vertex::Vertex { position: [-0.49513406,  0.06958647, 0.0], /*color: [0.20, 0.0, 0.40],*/ tex_coords: [0.0048659444, 0.430413540] },  // 1
	irid::vertex::Vertex { position: [-0.21918549, -0.44939706, 0.0], /*color: [0.25, 0.0, 0.25],*/ tex_coords: [0.2808145300, 0.949397057] },  // 2
	irid::vertex::Vertex { position: [ 0.35966998, -0.34732910, 0.0], /*color: [0.40, 0.0, 0.50],*/ tex_coords: [0.8596700000, 0.847329110] },  // 3
	irid::vertex::Vertex { position: [ 0.44147372,  0.23473590, 0.0], /*color: [0.50, 0.0, 0.10],*/ tex_coords: [0.9414737000, 0.265264100] },  // 4
];


pub(crate) const INDICES: &[u16] = &[
	0, 1, 4,
	1, 2, 4,
	2, 3, 4,
];


//= STATE STRUCT AND IMPL ==========================================================================

pub struct State {
	renderer: irid::renderer::Renderer,
	render_pipeline: wgpu::RenderPipeline,
	clear_color: wgpu::Color,

	// Texture support
	diffuse_bind_group: wgpu::BindGroup,

	// Polygon support
	vertex_buffer: wgpu::Buffer,
	index_buffer: wgpu::Buffer,
	num_indices: u32,

	// Camera
	camera_controller: irid::camera::CameraController,
	uniforms: irid::uniform::Uniforms,
	uniform_staging: irid::uniform::UniformStaging,
	uniform_buffer: wgpu::Buffer,
	uniform_bind_group: wgpu::BindGroup,
}


impl State {
	pub fn new(window: &Window) -> Self {
		let renderer = irid::renderer::Renderer::new(window);

		let clear_color = wgpu::Color::BLACK;  // todo nascondere wgpu

		//- Texture Section ------------------------------------------------------------------------

		let diffuse_texture = {
			let diffuse_bytes = include_bytes!("assets/textures/happy-tree.png");
			irid::texture::Texture::from_bytes(&renderer, diffuse_bytes, "happy-tree.png").unwrap()
		};

		let texture_bind_group_layout = irid::texture::create_bind_group_layout(
			&renderer,
			"Texture Bind Group Layout"
		);

		let diffuse_bind_group = diffuse_texture.create_bind_group(
			&renderer,
			"Diffuse Bind Group",
			&texture_bind_group_layout,
		);

		//- Camera ---------------------------------------------------------------------------------

		// TODO Pensare se fare un descrittore che crei la camera senza rendere così gli attributi pubblici
		let camera = irid::camera::Camera {
			// Position the camera one unit up and 2 units back +z is out of the screen
			eye: (0.0, 1.0, 2.0).into(),
			// Have it look at the origin
			target: (0.0, 0.0, 0.0).into(),
			// Which way is "up"
			up: cgmath::Vector3::unit_y(),
			aspect: renderer.calc_aspect_ratio(),
			fovy: 45.0,
			znear: 0.1,
			zfar: 100.0,
		};

		let camera_controller = irid::camera::CameraController::new(0.2);

		let mut uniforms = irid::uniform::Uniforms::new();

		let uniform_staging = irid::uniform::UniformStaging::new(camera);
		uniform_staging.update_uniforms(&mut uniforms);

		let uniform_buffer = irid::uniform::create_buffer_init(
			&renderer,
			"Uniform Buffer",
			uniforms
		);

		let uniform_bind_group_layout = irid::uniform::create_bind_group_layout(
			&renderer,
			"Uniform Bind Group Layout"
		);

		let uniform_bind_group = irid::uniform::create_bind_group(
			&renderer,
			"Uniform Bind Group",
			&uniform_bind_group_layout,
			&uniform_buffer
		);


		//- Shader Section -------------------------------------------------------------------------

		let render_pipeline = {
			let vs_module = irid::shader::create_module(&renderer, &wgpu::include_spirv!("assets/shaders/shader.vert.spv"));
			let fs_module = irid::shader::create_module(&renderer, &wgpu::include_spirv!("assets/shaders/shader.frag.spv"));

			let render_pipeline_layout = renderer.create_pipeline_layout(
				"Render Pipeline Layout",
				&[
					&texture_bind_group_layout,
					&uniform_bind_group_layout
				]
			);

			renderer.create_render_pipeline(
				"Render Pipeline",
				&render_pipeline_layout,
				&vs_module,
				&fs_module
			)
		};

		//- Vertex And Indices Section -------------------------------------------------------------

		let vertex_buffer = irid::vertex::create_buffer_for_vertices(
			&renderer,
			"Vertex Buffer",
			&VERTICES
		);

		let index_buffer = irid::vertex::create_buffer_for_indices(
			&renderer,
			"Index Buffer",
			&INDICES
		);

		let num_indices = INDICES.len() as u32;

		//- State Struct Instantiation -------------------------------------------------------------

		Self {
			renderer,
			clear_color,
			render_pipeline,
			diffuse_bind_group,
			vertex_buffer,
			index_buffer,
			num_indices,
			camera_controller,
			uniforms,
			uniform_staging,
			uniform_buffer,
			uniform_bind_group,
		}
	}

	pub fn refresh_size(&mut self) {
		self.renderer.update_swap_chain();
		self.uniform_staging.update_camera(self.renderer.calc_aspect_ratio());
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		self.renderer.set_size(new_size);
		self.refresh_size();
	}

	pub fn input(&mut self, event: &WindowEvent) -> bool {
		self.camera_controller.process_events(event)
		/*match event {
			WindowEvent::CursorMoved { position, .. } => {
				self.clear_color = wgpu::Color {
					r: position.x as f64 / self.size.width as f64,
					g: position.y as f64 / self.size.height as f64,
					b: 1.0,
					a: 1.0,
				};
				true
			},
			_ => self.camera_controller.process_events(event),
		}*/
	}

	pub fn update(&mut self) {
		self.camera_controller.update_camera(&mut self.uniform_staging.camera);
		self.uniform_staging.model_rotation += cgmath::Deg(2.0);
		self.uniform_staging.update_uniforms(&mut self.uniforms);
		self.renderer.write_queue_buffer(&self.uniform_buffer, 0, self.uniforms);
	}

	pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
		let frame = self.renderer.get_current_frame()?.output;

		let mut encoder = self.renderer.create_command_encoder("Render Encoder");

		{
			let mut render_pass = encoder.begin_render_pass(
				&wgpu::RenderPassDescriptor {
					label: Some("Render Pass"),
					color_attachments: &[wgpu::RenderPassColorAttachment {
						view: &frame.view,
						resolve_target: None,
						ops: wgpu::Operations {
							load: wgpu::LoadOp::Clear(self.clear_color),
							store: true,
						},
					}],
					depth_stencil_attachment: None,
				}
			);

			render_pass.set_pipeline(&self.render_pipeline);
			render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
			render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);

			let data = (&self.vertex_buffer, &self.index_buffer, self.num_indices);
			render_pass.set_vertex_buffer(0, data.0.slice(..));
			render_pass.set_index_buffer(data.1.slice(..), wgpu::IndexFormat::Uint16);

			render_pass.draw_indexed(0..data.2, 0, 0..1);
		}

		self.renderer.submit_to_queue(encoder);

		Ok(())
	}
}
