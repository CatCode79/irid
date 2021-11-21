//= USES ===========================================================================================

use irid_assets::GenericVertex;

use crate::{
    Device, FragmentStateBuilder, InstanceRaw, ShaderModuleBuilder, VertexStateBuilder,
    texture_metas::TextureDepthMetadatas
};

//= RENDERER PIPELINE BUILDER ======================================================================

///
pub struct RenderPipelineBuilder<'a> {
    render_pipeline_desc: wgpu::RenderPipelineDescriptor<'a>
}

// TODO: here we have to create directly an irid pipeline and not a wgpu pipeline
impl<'a> RenderPipelineBuilder<'a> {
    //- Constructors -------------------------------------------------------------------------------

    ///
    pub fn new(vertex: wgpu::VertexState<'a>) -> Self {
        Self {
            render_pipeline_desc: wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline Default Label"),
                layout: None,
                vertex,
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: Some(RenderPipelineBuilder::create_default_depth_stencil()),
                multisample: wgpu::MultisampleState::default(),
                fragment: None
            },
        }
    }

    fn create_default_depth_stencil() -> wgpu::DepthStencilState {
        wgpu::DepthStencilState {
            format: TextureDepthMetadatas::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }
    }

    //- Setters ------------------------------------------------------------------------------------

    ///
    pub fn with_label(&mut self, label_text: &'a str) -> &mut Self {
        self.render_pipeline_desc.label = if label_text.is_empty() {
            wgpu::Label::default()
        } else {
            Some(label_text)
        };
        self
    }

    ///
    pub fn with_layout(&mut self, layout: &'a wgpu::PipelineLayout) -> &mut Self {
        self.render_pipeline_desc.layout = Some(layout);
        self
    }

    ///
    pub fn with_vertex(&mut self, vertex: wgpu::VertexState<'a>) -> &mut Self {
        self.render_pipeline_desc.vertex = vertex;
        self
    }

    ///
    pub fn with_primitive(&mut self, primitive: wgpu::PrimitiveState) -> &mut Self {
        self.render_pipeline_desc.primitive = primitive;
        self
    }

    ///
    pub fn with_depth_stencil(&mut self, depth_stencil: wgpu::DepthStencilState) -> &mut Self {
        self.render_pipeline_desc.depth_stencil = Some(depth_stencil);
        self
    }

    ///
    pub fn with_multisample(&mut self, multisample: wgpu::MultisampleState) -> &mut Self {
        self.render_pipeline_desc.multisample = multisample;
        self
    }

    ///
    pub fn with_fragment(&mut self, fragment: wgpu::FragmentState<'a>) -> &mut Self {
        self.render_pipeline_desc.fragment = Some(fragment);
        self
    }

    //- Build --------------------------------------------------------------------------------------

    ///
    pub fn build(
        &mut self,
        device: &Device
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&self.render_pipeline_desc)
    }
}

//= RENDERER PIPELINE OBJECT =======================================================================

/// Wrapper to the wgpu handle's rendering graphics pipeline.
///
/// See [`wgpu::RenderPipeline`](wgpu::RenderPipeline).
pub struct RenderPipeline {
    wgpu_render_pipeline: wgpu::RenderPipeline,
}

impl RenderPipeline {
    //- Constructors -------------------------------------------------------------------------------

    ///
    pub fn new<'a, V: GenericVertex<'a>>(
        device: &Device,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        shader_source: String,
        preferred_format: wgpu::TextureFormat,
    ) -> Self {
        let pipeline_layout = PipelineLayoutBuilder::new()
            .with_bind_group_layouts(&[texture_bind_group_layout, camera_bind_group_layout])
            .build(device);

        let shader_module = ShaderModuleBuilder::new(
            wgpu::ShaderSource::Wgsl(std::borrow::Cow::Owned(shader_source))
        ).build(device);

        let buffers = [V::desc(),InstanceRaw::desc()];
        let vertex_state = {
            VertexStateBuilder::new(&shader_module)
                .with_buffers(&buffers)
                .build()
        };

        let targets = [wgpu::ColorTargetState {
            format: preferred_format,
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent::REPLACE,
                alpha: wgpu::BlendComponent::REPLACE,
            }),
            write_mask: wgpu::ColorWrites::ALL,
        }];
        let fragment_state = {
            FragmentStateBuilder::new(&shader_module)
                .with_targets(&targets)
                .build()
        };

        let primitive_state = PrimitiveStateBuilder::new().build();

        let multisample = wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        let wgpu_render_pipeline = RenderPipelineBuilder::new(vertex_state)
            .with_layout(&pipeline_layout)
            .with_fragment(fragment_state)
            .with_primitive(primitive_state)
            .with_multisample(multisample)
            .build(device);

        Self {
            wgpu_render_pipeline,
        }
    }

    //- Crate-Public Methods -----------------------------------------------------------------------

    // This method MUST remains public at the crate level.
    pub(crate) fn expose_wrapped_render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.wgpu_render_pipeline
    }
}

//= PIPELINE LAYOUT BUILDER ========================================================================

///
#[derive(Clone, Debug, Default)]
pub struct PipelineLayoutBuilder<'a> {
    pipeline_layout_desc: wgpu::PipelineLayoutDescriptor<'a>
}

impl<'a> PipelineLayoutBuilder<'a> {
    //- Constructors -------------------------------------------------------------------------------

    ///
    pub fn new() -> Self {
        Self {
            pipeline_layout_desc: wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout Default Label"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            },
        }
    }

    //- Setters ------------------------------------------------------------------------------------

    ///
    pub fn with_label(mut self, label_text: &'a str) -> Self {
        self.pipeline_layout_desc.label = if label_text.is_empty() {
            wgpu::Label::default()
        } else {
            Some(label_text)
        };
        self
    }

    ///
    pub fn with_bind_group_layouts(
        mut self,
        bind_group_layouts: &'a [&wgpu::BindGroupLayout]
    ) -> Self {
        self.pipeline_layout_desc.bind_group_layouts = bind_group_layouts;
        self
    }

    ///
    pub fn with_push_constant_ranges(
        mut self,
        push_constant_ranges: &'a [wgpu::PushConstantRange]
    ) -> Self {
        self.pipeline_layout_desc.push_constant_ranges = push_constant_ranges;
        self
    }

    //- Build --------------------------------------------------------------------------------------

    /// Build a new [PipelineLayout](wgpu::PipelineLayout).
    pub fn build(self, device: &Device) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&self.pipeline_layout_desc)
    }
}

//= PRIMITIVE STATE BUILDER ========================================================================

///
#[derive(Clone, Debug, Default)]
pub struct PrimitiveStateBuilder {
    primitive_state: wgpu::PrimitiveState,
}

impl PrimitiveStateBuilder {
    //- Constructors -------------------------------------------------------------------------------

    ///
    pub fn new() -> Self {
        Self {
            primitive_state: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Requires Features::DEPTH_CLAMPING
                unclipped_depth: false,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
        }
    }

    //- Setters ------------------------------------------------------------------------------------

    ///
    pub fn with_topology(&mut self, topology: wgpu::PrimitiveTopology) -> &mut Self {
        self.primitive_state.topology = topology;
        self
    }

    ///
    pub fn with_strip_index_format(&mut self, strip_index_format: wgpu::IndexFormat) -> &mut Self {
        self.primitive_state.strip_index_format = Some(strip_index_format);
        self
    }

    ///
    pub fn with_front_face(&mut self, front_face: wgpu::FrontFace) -> &mut Self {
        self.primitive_state.front_face = front_face;
        self
    }

    ///
    pub fn with_cull_mode(&mut self, cull_mode: wgpu::Face) -> &mut Self {
        self.primitive_state.cull_mode = Some(cull_mode);
        self
    }

    ///
    pub fn with_polygon_mode(&mut self, polygon_mode: wgpu::PolygonMode) -> &mut Self {
        self.primitive_state.polygon_mode = polygon_mode;
        self
    }

    ///
    pub fn with_unclipped_depth(&mut self, unclipped_depth: bool) -> &mut Self {
        self.primitive_state.unclipped_depth = unclipped_depth;
        self
    }

    ///
    pub fn with_conservative(&mut self, conservative: bool) -> &mut Self {
        self.primitive_state.conservative = conservative;
        self
    }

    //- Build --------------------------------------------------------------------------------------

    ///
    pub fn build(self) -> wgpu::PrimitiveState {
        self.primitive_state
    }
}
