
//= CONSTS =========================================================================================

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


//= STATIC VARIABLES ===============================================================================

static mut SAMPLER: Option<wgpu::Sampler> = None;


//= DEVICE WRAPPER =================================================================================

///
pub struct Device {
    pub texture: wgpu::Texture,
    pub surface: wgpu::Surface,
    wgpu_device: std::rc::Rc<wgpu::Device>,
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

        let texture = wgpu_device.create_texture(
            &wgpu::TextureDescriptor {
                size: DEFAULT_TEXTURE_SIZE,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: PREFERRED_TEXTURE_FORMAT,
                // SAMPLED tells wgpu that we want to use this texture in shaders
                // COPY_DST means that we want to copy data to this texture
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
                label: Some("Diffuse Texture"),
            }
        );

        let device = Self {
            texture,
            surface,
            wgpu_device: std::rc::Rc::new(wgpu_device),
        };
        (device, queue)
    }

    ///
    pub fn create_vertex_buffer_init(
        &self,
        label_text: &str,
        vertices: &[crate::vertex::Vertex]
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
