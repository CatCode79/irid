//= USES ===========================================================================================


//= PIPELINE LAYOUT ================================================================================

pub struct PipelineLayoutBuilder<'a> {
    pipeline_layout_desc: wgpu::PipelineLayoutDescriptor<'a>
}


impl<'a> PipelineLayoutBuilder<'a> {
    pub fn new() -> Self {
        #[cfg(feature = "debug_label")]
        let label = Some("Pipeline Layout Default Label");
        #[cfg(not(feature = "debug_label"))]
        let label = wgpu::Label.default();

        Self {
            pipeline_layout_desc: wgpu::PipelineLayoutDescriptor {
                label,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            },
        }
    }

    pub fn label(mut self, label_text: &'a str) -> Self {
        self.pipeline_layout_desc.label = if label_text.is_empty() {
            wgpu::Label::default()
        } else {
            Some(label_text)
        };
        self
    }

    pub fn bind_group_layouts(
        mut self,
        bind_group_layouts: &'a [&wgpu::BindGroupLayout]
    ) -> Self {
        self.pipeline_layout_desc.bind_group_layouts = bind_group_layouts;
        self
    }

    pub fn push_constant_ranges(
        mut self,
        push_constant_ranges: &'a [wgpu::PushConstantRange]
    ) -> Self {
        self.pipeline_layout_desc.push_constant_ranges = push_constant_ranges;
        self
    }

    pub fn expose_wrapped_desc(&self) -> &wgpu::PipelineLayoutDescriptor {
        &self.pipeline_layout_desc
    }

    pub fn build(self, device: &wgpu::Device) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&self.pipeline_layout_desc)
    }
}


//= PRIMITIVE STATE ================================================================================

pub struct PrimitiveStateBuilder {
    primitive_state: wgpu::PrimitiveState,
}

impl PrimitiveStateBuilder {
    pub fn new() -> Self {  // TODO: bug! a me puzza sto static
        Self {
            primitive_state: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),  // TODO: Qui in nannou il cull_mode è None, da controllare in learnwgpu cosa servisse esattamente
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLAMPING
                clamp_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
        }
    }

    pub fn topology(&mut self, topology: wgpu::PrimitiveTopology) -> &mut Self {
        self.primitive_state.topology = topology;
        self
    }

    pub fn strip_index_format(&mut self, strip_index_format: wgpu::IndexFormat) -> &mut Self {
        self.primitive_state.strip_index_format = Some(strip_index_format);
        self
    }

    pub fn front_face(&mut self, front_face: wgpu::FrontFace) -> &mut Self {
        self.primitive_state.front_face = front_face;
        self
    }

    pub fn cull_mode(&mut self, cull_mode: wgpu::Face) -> &mut Self {
        self.primitive_state.cull_mode = Some(cull_mode);
        self
    }

    pub fn polygon_mode(&mut self, polygon_mode: wgpu::PolygonMode) -> &mut Self {
        self.primitive_state.polygon_mode = polygon_mode;
        self
    }

    pub fn clamp_depth(&mut self, clamp_depth: bool) -> &mut Self {
        self.primitive_state.clamp_depth = clamp_depth;
        self
    }

    pub fn conservative(&mut self, conservative: bool) -> &mut Self {
        self.primitive_state.conservative = conservative;
        self
    }

    pub fn build(self) -> wgpu::PrimitiveState {
        self.primitive_state
    }
}


//= PIPELINE DESCRIPTOR ============================================================================

pub struct RenderPipelineBuilder<'a> {
    render_pipeline_desc: wgpu::RenderPipelineDescriptor<'a>
}


// TODO: qui devo creare direttamente una pipeline irid e non wgpu
impl<'a> RenderPipelineBuilder<'a> {
    pub fn new(vertex: wgpu::VertexState<'a>) -> Self {
        #[cfg(feature = "debug_label")]
        let label = Some("Render Pipeline Default Label");
        #[cfg(not(feature = "debug_label"))]
        let label = None;

        Self {
            render_pipeline_desc: wgpu::RenderPipelineDescriptor {
                label,
                layout: None,
                vertex,
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                fragment: None
            },
        }
    }

    pub fn label(&mut self, label_text: &'a str) -> &mut Self {
        self.render_pipeline_desc.label = if label_text.is_empty() {
            wgpu::Label::default()
        } else {
            Some(label_text)
        };
        self
    }

    pub fn layout(&mut self, layout: &'a wgpu::PipelineLayout) -> &mut Self {
        self.render_pipeline_desc.layout = Some(layout);
        self
    }

    pub fn vertex(&mut self, vertex: wgpu::VertexState<'a>) -> &mut Self {
        self.render_pipeline_desc.vertex = vertex;
        self
    }

    pub fn primitive(&mut self, primitive: wgpu::PrimitiveState) -> &mut Self {
        self.render_pipeline_desc.primitive = primitive;
        self
    }

    pub fn depth_stencil(&mut self, depth_stencil: wgpu::DepthStencilState) -> &mut Self {
        self.render_pipeline_desc.depth_stencil = Some(depth_stencil);
        self
    }

    pub fn multisample(&mut self, multisample: wgpu::MultisampleState) -> &mut Self {
        self.render_pipeline_desc.multisample = multisample;
        self
    }

    pub fn fragment(&mut self, fragment: wgpu::FragmentState<'a>) -> &mut Self {
        self.render_pipeline_desc.fragment = Some(fragment);
        self
    }

    pub fn expose_wrapped_desc(&self) -> &wgpu::RenderPipelineDescriptor {
        &self.render_pipeline_desc
    }

    pub fn build(&mut self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&self.render_pipeline_desc)
    }
}


//= PIPELINE WRAPPER ===============================================================================

/// Wrapper to the wgpu handle's rendering graphics pipeline.
///
/// See [`wgpu::RenderPipeline`](wgpu::RenderPipeline).
pub struct RenderPipeline {
    wgpu_render_pipeline: wgpu::RenderPipeline,
}


impl RenderPipeline {
    pub fn new(
        device: &crate::renderer::Device,
        texture_metadatas: &crate::renderer::TextureMetadatas,
        shader_source: String
    ) -> Self {
        let wgpu_device = device.expose_wgpu_device();

        let pipeline_layout = PipelineLayoutBuilder::new()
            .bind_group_layouts(&[&texture_metadatas.bind_group_layout()])
            .build(wgpu_device);

        let buffers = &[crate::meshes::VertexTexture::desc()];
        let shader_module = crate::renderer::ShaderModuleBuilder::new(
            wgpu::ShaderSource::Wgsl(std::borrow::Cow::Owned(shader_source))
        ).build(wgpu_device);
        let vertex_state = crate::renderer::VertexStateBuilder::new(&shader_module)
            .buffers(buffers)
            .build();
        let fragment_state = crate::renderer::FragmentStateBuilder::new(&shader_module).build();

        let primitive_state = PrimitiveStateBuilder::new().build();

        // TODO: fare il Builder di 'sta roba?
        let multisample = wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        let wgpu_render_pipeline = RenderPipelineBuilder::new(vertex_state)
            .layout(&pipeline_layout)
            .fragment(fragment_state)
            .primitive(primitive_state)
            .multisample(multisample)
            .build(wgpu_device);

        Self {
            wgpu_render_pipeline,
        }
    }

    pub fn expose_wrapped_render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.wgpu_render_pipeline
    }
}
