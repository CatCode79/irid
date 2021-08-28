
//= CONSTS =========================================================================================

use wgpu::TextureAspect;

// TODO farlo come funzione per ricavare, anche solo per debug, che cosa è preferito dal device
// Most images are stored using sRGB so we need to reflect that here.
pub(crate) const PREFERRED_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

// TODO: questa è una di quelle parti di codice da cambiare per rendere il framework più usabile;
//  l'idea di fondo cmq mi piace perchè migliora le prestazioni durante la creazione delle texture.
pub const DEFAULT_TEXTURE_WIDTH: u32 = 256;
pub const DEFAULT_TEXTURE_HEIGHT: u32 = 256;

pub(crate) const DEFAULT_TEXTURE_SIZE: wgpu::Extent3d = wgpu::Extent3d {
    width: DEFAULT_TEXTURE_WIDTH,
    height: DEFAULT_TEXTURE_HEIGHT,
    // All textures are stored as 3D, we represent our 2D texture by setting depth to 1
    depth_or_array_layers: 1,
};


//= DEVICE WRAPPER =================================================================================

///
pub struct Device {
    pub surface: wgpu::Surface,
    wgpu_device: std::rc::Rc<wgpu::Device>,
    pub diffuse_texture: wgpu::Texture,
    diffuse_sampler: wgpu::Sampler,
    diffuse_texture_view: wgpu::TextureView,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub diffuse_bind_group: wgpu::BindGroup,  // TODO: questo campo ha più senso in RenderPass, credo, oppure direttamente in Renderer
}


impl Device {
    /// The device is an open connection to a graphics and/or compute device responsible
    /// for the creation of most rendering and compute resources.
    /// The queue executes recorded CommandBuffer and writes to buffers and textures.
    pub fn new(window: &winit::window::Window) -> (Self, wgpu::Queue){
        // Context for all other wgpu objects
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        // For debug purpose prints on console all the available adapters
        enumerate_all_adapters(&instance);

        // Handle to a presentable surface onto which rendered images
        let surface = unsafe { instance.create_surface(window) };

        // Adapter can be used to open a connection to the corresponding graphical device
        let adapter = futures::executor::block_on(async {
            instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                }
            ).await
        }).unwrap();  // todo Result check

        let (wgpu_device, queue) = futures::executor::block_on(async {
            adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("New Device & Queue"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            ).await
        }).unwrap(); // todo Result check

        let diffuse_texture = wgpu_device.create_texture(&wgpu::TextureDescriptor {
            size: DEFAULT_TEXTURE_SIZE,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: PREFERRED_TEXTURE_FORMAT,
            // SAMPLED tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: Some("Device Diffuse Texture"),
        });

        let diffuse_sampler = wgpu_device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Device Texture Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,  // TODO: probabilmente meglio utilizzare MirrorRepeated per evitare le Bleeding Textures
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: None,
            anisotropy_clamp: None,
            border_color: None
        });

        let diffuse_texture_view = diffuse_texture.create_view(
                &wgpu::TextureViewDescriptor {
                label: Some("Device Texture View"),
                format: None,
                dimension: None,
                aspect: TextureAspect::All,
                base_mip_level: 0,
                mip_level_count: None,
                base_array_layer: 0,
                array_layer_count: None
            }
        );

        let texture_bind_group_layout = wgpu_device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            // This is only for TextureSampleType::Depth
                            comparison: false,
                            // This should be true if the sample_type of the texture is:
                            //     TextureSampleType::Float { filterable: true }
                            // Otherwise you'll get an error.
                            filtering: true,
                        },
                        count: None,
                    },
                ],
                label: Some("Device Texture Bind Group Layout"),
            }
        );

        let diffuse_bind_group = wgpu_device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                    }
                ],
                label: Some("Diffuse Bind Group"),
            }
        );

        let device = Self {
            surface,
            wgpu_device: std::rc::Rc::new(wgpu_device),
            diffuse_texture,
            diffuse_sampler,
            diffuse_texture_view,
            texture_bind_group_layout,
            diffuse_bind_group,
        };
        (device, queue)
    }

    ///
    pub fn create_vertex_buffer_init(
        &self,
        label_text: &str,
        vertices: &[crate::meshes::Vertex]
    ) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        self.wgpu_device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(label_text),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsage::VERTEX,
            }
        )
    }

    ///
    pub fn create_indices_buffer_init(&self, label_text: &str, indices: &[u16]) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        self.wgpu_device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(label_text),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsage::INDEX,
            }
        )
    }

    ///
    pub fn expose_wgpu_device(&self) -> &std::rc::Rc<wgpu::Device> {
        &self.wgpu_device
    }
}

/*
        // TODO: meglio spostarla in una struct Device, senza option, e inizializzata durante
        // la creazione del rendering
        unsafe {
            if SAMPLER.is_none() {
                SAMPLER = Some(device.create_sampler(
                    &wgpu::SamplerDescriptor {
                        address_mode_u: wgpu::AddressMode::ClampToEdge,  // TODO: da considerare il MirroreRepeat per evitare il beeeding edges
                        address_mode_v: wgpu::AddressMode::ClampToEdge,
                        address_mode_w: wgpu::AddressMode::ClampToEdge,
                        mag_filter: wgpu::FilterMode::Linear,
                        min_filter: wgpu::FilterMode::Nearest,
                        mipmap_filter: wgpu::FilterMode::Nearest,
                        ..Default::default()
                    })
                );
            }
        }
*/


// TODO: diamine non funziona..
/// Show all the adapters information for debug.
//#[cfg(debug_assertions)]
fn enumerate_all_adapters(instance: &wgpu::Instance) {
    instance.poll_all(true);
    for adapter in instance.enumerate_adapters(wgpu::BackendBit::all()) {
        use log::info;
        info!("{:#?}\n", adapter.get_info())
    }
}
