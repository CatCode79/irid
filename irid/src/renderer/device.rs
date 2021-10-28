
//= USES ===========================================================================================

use crate::assets::ModelVertex;
use crate::renderer::Adapter;


//= DEVICE WRAPPER =================================================================================

/// Open connection to a graphics and/or compute device.
///
/// Responsible for the creation of most rendering and compute resources.
/// These are then used in commands, which are submitted to a [`Queue`](wgpu::Queue).
///
/// A device may be requested from an adapter with
/// [`Adapter::request_device`](Adapter::request_device).
#[derive(Debug)]
pub struct Device {
    label_text: String,
    wgpu_device: wgpu::Device,
}


impl Device {

    //- Constructor Methods ------------------------------------------------------------------------

    /// Create a new Device and Queue given ad adapter.
    pub async fn new(
        adapter: &Adapter
    ) -> anyhow::Result<(Self, wgpu::Queue), wgpu::RequestDeviceError> {
        let label_text = format!(
            "Device Default Label [creation {:?}]", std::time::SystemTime::now()
        );

        let (wgpu_device, queue) = {
            adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: Some(label_text.as_str()),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_defaults(),  // TODO to be choosable by user
                },
                None, // Trace path
            ).await
        }?;

        let device = Self {
            label_text,
            wgpu_device,
        };

        Ok((device, queue))
    }

    //- Creation Methods ---------------------------------------------------------------------------

    /// Creates a [Buffer](wgpu::Buffer) with data to initialize it.
    pub fn create_buffer_init(&self, buffer_init_desc: &wgpu::util::BufferInitDescriptor) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        self.wgpu_device.create_buffer_init(buffer_init_desc)
    }

    /// Creates a vertex Buffer with data to initialize it.
    pub fn create_vertex_buffer_init(
        &self,
        label_text: &str,
        vertices: &[ModelVertex]
    ) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        self.wgpu_device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(label_text),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        )
    }

    /// Creates a indices Buffer with data to initialize it.
    pub fn create_indices_buffer_init(&self, label_text: &str, indices: &[u32]) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        self.wgpu_device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(label_text),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        )
    }

    /// Creates a [BindGroupLayout](wgpu::BindGroupLayout).
    pub fn create_bind_group_layout(
        &self,
        bind_group_layout_desc: &wgpu::BindGroupLayoutDescriptor
    ) -> wgpu::BindGroupLayout {
        self.wgpu_device.create_bind_group_layout(bind_group_layout_desc)
    }

    /// Creates a new [BindGroup](wgpu::BindGroup).
    pub fn create_bind_group(&self, bind_group_desc: &wgpu::BindGroupDescriptor) -> wgpu::BindGroup {
         self.wgpu_device.create_bind_group(bind_group_desc)
    }

    /// Creates a [ShaderModule](wgpu::ShaderModule) from either SPIR-V or WGSL source code.
    pub fn create_shader_module(
        &self,
        shader_module_desc: &wgpu::ShaderModuleDescriptor
    ) -> wgpu::ShaderModule {
        self.wgpu_device.create_shader_module(shader_module_desc)
    }

    /// Creates a [PipelineLayout](wgpu::PipelineLayout).
    pub fn create_pipeline_layout(
        &self,
        pipeline_layout_desc: &wgpu::PipelineLayoutDescriptor
    ) -> wgpu::PipelineLayout {
        self.wgpu_device.create_pipeline_layout(pipeline_layout_desc)
    }

    /// Creates a [RenderPipeline](wgpu::RenderPipeline).
    pub fn create_render_pipeline(
        &self,
        render_pipeline_desc: &wgpu::RenderPipelineDescriptor
    ) -> wgpu::RenderPipeline {
        self.wgpu_device.create_render_pipeline(render_pipeline_desc)
    }

    /// Creates a [CommandEncoder](wgpu::CommandEncoder).
    pub fn create_command_encoder(
        &self,
        command_encoder_desc: &wgpu::CommandEncoderDescriptor
    ) -> wgpu::CommandEncoder {
        self.wgpu_device.create_command_encoder(command_encoder_desc)
    }

    /// Creates a new [Texture](wgpu::Texture).
    ///
    /// # Param
    /// - texture_desc specifies the general format of the texture.
    pub fn create_texture(&self, texture_desc: &wgpu::TextureDescriptor) -> wgpu::Texture {
        self.wgpu_device.create_texture(texture_desc)
    }

    /// Creates a new [Sampler](wgpu::Sampler).
    ///
    /// # Param
    /// - desc specifies the behavior of the sampler.
    pub fn create_sampler(&self, sampler_desc: &wgpu::SamplerDescriptor) -> wgpu::Sampler {
        self.wgpu_device.create_sampler(sampler_desc)
    }

    //- Wrapper Methods ----------------------------------------------------------------------------

    // This method MUST remains public at the crate level.
    pub(crate) fn expose_wrapped_device(&self) -> &wgpu::Device {
        &self.wgpu_device
    }
}
