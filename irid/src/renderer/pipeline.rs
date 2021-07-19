
//= PIPELINE LAYOUT ================================================================================

pub(crate) struct PipelineLayoutDescriptor<'a> {
    wrapped_pipeline_layout_desc: wgpu::PipelineLayoutDescriptor<'a>
}


impl<'a> PipelineLayoutDescriptor<'a> {
    pub(crate) fn new(label_text: &'a str) -> &mut Self {
        &mut Self {
            wrapped_pipeline_layout_desc: wgpu::PipelineLayoutDescriptor::default(),
        }.label(label_text)
    }

    pub(crate) fn label(&mut self, label_text: &'a str) -> &mut Self {
        self.wrapped_pipeline_layout_desc.label = if label_text.is_empty() {
            if cfg!(debug_assertions) {
                Some(&format!("Pipeline Layout Descriptor Default Label {:p}", self))
            } else {
                wgpu::Label::default()
            }
        } else {
            Some(label_text)
        };
        self
    }

    pub(crate) fn bind_group_layouts(&mut self, values: &'a [&wgpu::BindGroupLayout]) -> &mut Self {
        self.wrapped_pipeline_layout_desc.bind_group_layouts = values;
        self
    }

    pub(crate) fn push_constant_ranges(&mut self, values: &'a [wgpu::PushConstantRange]) -> &mut Self {
        self.wrapped_pipeline_layout_desc.push_constant_ranges = values;
        self
    }

    pub(crate) fn expose_wrapped_pipeline_layout_desc(&self) -> &wgpu::PipelineLayoutDescriptor {
        &self.wrapped_pipeline_layout_desc
    }
}


//= PIPELINE DESCRIPTOR ============================================================================

pub(crate) struct PipelineDescriptor<'a> {
    wrapped_pipeline_desc: wgpu::RenderPipelineDescriptor<'a>,
}


impl<'a> PipelineDescriptor<'a> {
    pub(crate) fn new(label_text: &str) -> &mut Self {
        let desc = &mut Self {
            wrapped_pipeline_desc: wgpu::RenderPipelineDescriptor {
                pub label: Label<'a>,
                pub layout: Option<&'a PipelineLayout>,
                pub vertex: VertexState<'a>,
                pub primitive: PrimitiveState,
                pub depth_stencil: Option<DepthStencilState>,
                pub multisample: MultisampleState,
                pub fragment: Option<FragmentState<'a>>,
            },
        };
        desc.label(label_text)
    }

    pub(crate) fn label(&mut self, label_text: &'a str) -> &mut Self {
        self.wrapped_pipeline_desc.label = if label_text.is_empty() {
            if cfg!(debug_assertions) {
                Some(&format!("Pipeline Descriptor Default Label {:p}", self))
            } else {
                wgpu::Label::default()
            }
        } else {
            Some(label_text)
        };
        self
    }

    pub(crate) fn layout(&mut self, value: &'a wgpu::PipelineLayout) -> &mut Self {
        self.wrapped_pipeline_desc.layout = Some(value);
        self
    }

    pub(crate) fn vertex(&mut self, value: wgpu::VertexState<'a>) -> &mut Self {
        self.wrapped_pipeline_desc.vertex = value;
        self
    }

    pub(crate) fn primitive(&mut self, value: wgpu::PrimitiveState) -> &mut Self {
        self.wrapped_pipeline_desc.primitive = value;
        self
    }

    pub(crate) fn depth_stencil(&mut self, value: wgpu::DepthStencilState) -> &mut Self {
        self.wrapped_pipeline_desc.depth_stencil = Some(value);
        self
    }

    pub(crate) fn multisample(&mut self, value: wgpu::MultisampleState) -> &mut Self {
        self.wrapped_pipeline_desc.multisample = value;
        self
    }

    pub(crate) fn fragment(&mut self, value: wgpu::FragmentState<'a>) -> &mut Self {
        self.wrapped_pipeline_desc.fragment = Some(value);
        self
    }

    pub(crate) fn expose_wrapped_pipeline_desc(&self) -> &wgpu::RenderPipelineDescriptor {
        &self.wrapped_pipeline_desc
    }
}


//= PIPELINE WRAPPER ===============================================================================

/// Wrapper to the wgpu handle's rendering graphics pipeline.
///
/// See [`wgpu::RenderPipeline`](wgpu::RenderPipeline).
pub(crate) struct Pipeline<'a> {
    pipeline_desc: crate::renderer::PipelineDescriptor<'a>,
    wgpu_pipeline: wgpu::RenderPipeline,
}


impl<'a> Pipeline<'a> {
    pub fn new(device: &wgpu::Device, shader_filename: &str) -> Self {
        let pipeline_layout = device.create_pipeline_layout(
            crate::renderer::PipelineLayoutDescriptor::new("")
                .expose_wrapped_pipeline_layout_desc()
        );

        let (vertex, fragment) = Pipeline::create_shader(device, shader_filename);

        let primitive = wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLAMPING
            clamp_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        };

        let multisample = wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        // TODO ATTENZIONE  depth_stencil: None  come default
        let pipeline_desc = crate::renderer::PipelineDescriptor::new("")
            .layout(&pipeline_layout)
            .vertex(vertex)
            .fragment(fragment)
            .primitive(primitive)
            .multisample(multisample);

        let wgpu_pipeline = device.create_render_pipeline(pipeline_desc.expose_wrapped_pipeline_desc());

        Self {
            pipeline_desc,
            wgpu_pipeline,
        }
    }

    pub fn create_shader(device: &wgpu::Device, source_text: &str) -> (wgpu::VertexState<'a>, wgpu::FragmentState<'a>) {
        let shader_module = crate::renderer::ShaderModuleBuilder::new("Shader")
            .source_as_wgsl(source_text)
            .build(&device);

        // todo da spostare in shader penso
        let vertex = wgpu::VertexState {
            module: &shader_module,
            entry_point: "main",
            buffers: &[],
        };

        // todo da spostare in shader penso
        let fragment = wgpu::FragmentState {
            module: &shader_module,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: crate::texture::PREFERRED_TEXTURE_FORMAT,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrite::ALL,
            }],
        };

        (vertex, fragment)
    }
}
