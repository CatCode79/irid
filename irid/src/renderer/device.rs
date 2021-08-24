//= STATIC VARIABLES ===============================================================================

use crate::renderer::SwapChain;

static mut SAMPLER: Option<wgpu::Sampler> = None;


//= DEVICE WRAPPER =================================================================================

///
pub struct Device<'a> {
    pub surface: wgpu::Surface,
    wgpu_device: &'a wgpu::Device,
}

impl<'a> Device<'a> {
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

        let device = Self {
            surface,
            wgpu_device: &wgpu_device,
        };
        (device, queue)
    }

    ///
    pub fn create_swap_chain(&self, size: winit::dpi::PhysicalSize<u32>) -> SwapChain {
        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: crate::renderer::PREFERRED_TEXTURE_FORMAT,
            width: size.width,
            height: size.height,
            // Fifo is "vsync on". Immediate is "vsync off". Mailbox is a hybrid between the two
            // (gpu doesn't block if running faster than the display, but screen tearing doesn't happen)
            // TODO far scegliere al giocatore prima dell'avvio del gioco
            present_mode: wgpu::PresentMode::Fifo,
        };

        crate::renderer::SwapChain::new(&self, swap_chain_desc)
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
    pub fn expose_wgpu_device(&self) -> &wgpu::Device {
        self.wgpu_device
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
