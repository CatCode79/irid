//= USES ===========================================================================================

use std::iter;

use futures::executor::block_on;
use winit::{
	window::Window,
	event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
};
use wgpu::util::DeviceExt;

use crate::irid::{
	vertex::{INDICES, VERTICES, Vertex, create_polygon},
};


//= STATE STRUCT AND IMPL ==========================================================================

pub struct State {
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	swap_chain_desc: wgpu::SwapChainDescriptor,
	swap_chain: wgpu::SwapChain,
	size: winit::dpi::PhysicalSize<u32>,
	clear_color: wgpu::Color,
	render_pipeline: wgpu::RenderPipeline,

	vertex_buffer: wgpu::Buffer,
	index_buffer: wgpu::Buffer,
	num_indices: u32,
	challenge_vertex_buffer: wgpu::Buffer,
	challenge_index_buffer: wgpu::Buffer,
	num_challenge_indices: u32,
	use_complex: bool,
}


impl State {
	pub fn new(window: &Window) -> Self {
		let size = window.inner_size();

		// The instance is a handle to our GPU
		// BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
		let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
		let surface = unsafe { instance.create_surface(window) };
		let adapter = block_on(async {
			instance.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::HighPerformance,
				compatible_surface: Some(&surface),
			}).await
		}).unwrap();

		let (device, queue) = block_on(async {
			adapter.request_device(
				&wgpu::DeviceDescriptor {
					label: None,
					features: wgpu::Features::empty(),
					limits: wgpu::Limits::default(),
				},
				None, // Trace path
			).await
		}).unwrap();

		let swap_chain_desc = wgpu::SwapChainDescriptor {
			usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
			format: adapter.get_swap_chain_preferred_format(&surface).unwrap_or(wgpu::TextureFormat::Bgra8UnormSrgb),
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Fifo,
		};
		let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

		let clear_color = wgpu::Color::BLACK;

		let vs_module = device.create_shader_module(&wgpu::include_spirv!("irid/shaders/shader.vert.spv"));
		let fs_module = device.create_shader_module(&wgpu::include_spirv!("irid/shaders/shader.frag.spv"));

		let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("Render Pipeline Layout"),
			bind_group_layouts: &[],
			push_constant_ranges: &[],
		});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Render Pipeline"),
			layout: Some(&render_pipeline_layout),
			vertex: wgpu::VertexState {
				module: &vs_module,
				entry_point: "main",
				buffers: &[Vertex::desc()],
			},
			fragment: Some(wgpu::FragmentState {
				module: &fs_module,
				entry_point: "main",
				targets: &[wgpu::ColorTargetState {
					format: swap_chain_desc.format,
					write_mask: wgpu::ColorWrite::ALL,
					blend: Option::from(wgpu::BlendState::REPLACE),
				}],
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: Option::from(wgpu::Face::Back),
				// Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
				clamp_depth: false,
				polygon_mode: wgpu::PolygonMode::Fill,
				conservative: false,
			},
			depth_stencil: None,
			multisample: wgpu::MultisampleState {
				count: 1,
				mask: !0,
				alpha_to_coverage_enabled: false,
			},
		});

		let vertex_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Vertex Buffer"),
				contents: bytemuck::cast_slice(VERTICES),
				usage: wgpu::BufferUsage::VERTEX,
			}
		);

		let index_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Index Buffer"),
				contents: bytemuck::cast_slice(INDICES),
				usage: wgpu::BufferUsage::INDEX,
			}
		);

		let num_indices = INDICES.len() as u32;

		let (challenge_verts, challenge_indices) = create_polygon(16);

		let challenge_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Challenge Vertex Buffer"),
			contents: bytemuck::cast_slice(&challenge_verts),
			usage: wgpu::BufferUsage::VERTEX,
		});

		let challenge_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Challenge Index Buffer"),
			contents: bytemuck::cast_slice(&challenge_indices),
			usage: wgpu::BufferUsage::INDEX,
		});

		let num_challenge_indices = challenge_indices.len() as u32;

		let use_complex = false;

		Self {
			surface,
			device,
			queue,
			swap_chain_desc,
			swap_chain,
			clear_color,
			size,
			render_pipeline,
			vertex_buffer,
			index_buffer,
			num_indices,
			challenge_vertex_buffer,
			challenge_index_buffer,
			num_challenge_indices,
			use_complex,
		}
	}

	pub fn refresh_size(&mut self) {
		self.swap_chain_desc.width = self.size.width;
		self.swap_chain_desc.height = self.size.height;
		self.swap_chain = self.device.create_swap_chain(&self.surface, &self.swap_chain_desc);
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		self.size = new_size;
		self.refresh_size();
	}

	#[allow(unused_variables)]
	pub fn input(&mut self, event: &WindowEvent) -> bool {
		match event {
			WindowEvent::CursorMoved { position, .. } => {
				self.clear_color = wgpu::Color {
					r: position.x as f64 / self.size.width as f64,
					g: position.y as f64 / self.size.height as f64,
					b: 1.0,
					a: 1.0,
				};
				true
			}
			WindowEvent::KeyboardInput {
				input:
				KeyboardInput {
					state,
					virtual_keycode: Some(VirtualKeyCode::Space),
					..
				},
				..
			} => {
				self.use_complex = *state == ElementState::Pressed;
				true
			}
			_ => false,
		}
	}

	pub fn update(&mut self) {}

	pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
		let frame = self.swap_chain.get_current_frame()?.output;

		let mut encoder = self
			.device
			.create_command_encoder(&wgpu::CommandEncoderDescriptor {
				label: Some("Render Encoder"),
			});

		{
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
			});

			render_pass.set_pipeline(&self.render_pipeline);

			let data = if self.use_complex {
				(&self.challenge_vertex_buffer, &self.challenge_index_buffer, self.num_challenge_indices)
			} else {
				(&self.vertex_buffer, &self.index_buffer, self.num_indices)
			};
			render_pass.set_vertex_buffer(0, data.0.slice(..));
			render_pass.set_index_buffer(data.1.slice(..), wgpu::IndexFormat::Uint16);

			render_pass.draw_indexed(0..data.2, 0, 0..1);
		}

		self.queue.submit(iter::once(encoder.finish()));

		Ok(())
	}
}
