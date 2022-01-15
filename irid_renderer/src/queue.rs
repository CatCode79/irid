//= USES ===========================================================================================

use std::future::Future;
use thiserror::Error;

use irid_assets_interface::{Image, ImageSize, Texture, TextureError};
use irid_renderer_interface::Camera;

use crate::{camera_bind::CameraBindGroup, texture_metadatas::TextureImageMetadatas, utils::log2};

//= ERRORS =========================================================================================

#[non_exhaustive]
#[derive(Debug, Error)]  // TODO: impossible to Clone because of TextureError
pub enum QueueError {
    #[error("Impossible to enqueue the texture")]
    WriteTexture {
        #[from]
        source: TextureError,
    },
}

//= QUEUE ==========================================================================================

///
#[derive(Debug)]
pub struct Queue {
    wgpu_queue: wgpu::Queue,
}

impl Queue {
    //- Constructors -------------------------------------------------------------------------------

    ///
    pub fn new(wgpu_queue: wgpu::Queue) -> Self {
        Self { wgpu_queue }
    }

    //- Wrapped Methods ----------------------------------------------------------------------------

    /// Schedule a data write into `buffer` starting at `offset`.
    ///
    /// This method is intended to have low performance costs.
    /// As such, the write is not immediately submitted, and instead enqueued
    /// internally to happen at the start of the next `submit()` call.
    // TODO: to refact after the camera refact, need to pass only one arg
    pub(crate) fn write_camera_buffer<C: Camera>(
        &self,
        camera: &C,
        camera_metadatas: &CameraBindGroup,
    ) {
        let mut camera_uniform = *camera_metadatas.uniform();
        camera_uniform.update_view_proj(camera);
        self.wgpu_queue.write_buffer(
            camera_metadatas.buffer(),
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    /// Schedule a data write into texture.
    ///
    /// This method is intended to have low performance costs.
    /// As such, the write is not immediately submitted, and instead enqueued
    /// internally to happen at the start of the next `submit()` call.
    pub fn write_texture<T: Texture<S>, S: ImageSize>(
        &self,
        texture_image_metadatas: &[Vec<TextureImageMetadatas>],
        texture: T,
    ) -> Result<(), QueueError> {
        // TODO: better add a ref to metas inside irid Texture structs
        let metadatas = &texture_image_metadatas[log2(texture.size().width() as i32) as usize]
            [log2(texture.size().height() as i32) as usize];

        Ok(self.wgpu_queue.write_texture(
            metadatas.create_image_copy(),
            texture.image().as_rgba8_bytes().unwrap(),
            *metadatas.image_data_layout(),
            *metadatas.image_size(),
        ))
    }

    /// Submits a series of finished command buffers for execution.
    pub fn submit<I: IntoIterator<Item = wgpu::CommandBuffer>>(&self, command_buffers: I) {
        self.wgpu_queue.submit(command_buffers);
    }

    /// Gets the amount of nanoseconds each tick of a timestamp query represents.
    ///
    /// Returns zero if timestamp queries are unsupported.
    pub fn get_timestamp_period(&self) -> f32 {
        self.wgpu_queue.get_timestamp_period()
    }

    /// Returns a future that resolves once all the work submitted by this point
    /// is done processing on GPU.
    pub fn on_submitted_work_done(&self) -> impl Future<Output = ()> + Send {
        self.wgpu_queue.on_submitted_work_done()
    }
}
