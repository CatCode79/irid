//= USES =====================================================================

use irid_assets::DiffuseTexture;
use irid_assets_interface::{Image, ImageSize};
use thiserror::Error;

use crate::camera::Camera;
use crate::camera_bind::CameraBindGroup;
use crate::texture_metadata::TextureImageMetadata;
use crate::utils::log2;

//= ERRORS ===================================================================

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("Impossible to enqueue None bytes, as rgba, from texture {{0}}")]
    RgbaTextureNoneBytes { path: std::path::PathBuf },
}

//= QUEUE ====================================================================

///
#[derive(Debug)]
pub struct Queue {
    wgpu_queue: wgpu::Queue,
}

impl Queue {
    //- Constructors ---------------------------------------------------------

    ///
    pub fn new(wgpu_queue: wgpu::Queue) -> Self {
        Self { wgpu_queue }
    }

    //- Wrapped Methods ------------------------------------------------------

    /// Schedule a data write into `buffer` starting at `offset`.
    ///
    /// This method is intended to have low performance costs.
    /// As such, the write is not immediately submitted, and instead enqueued
    /// internally to happen at the start of the next `submit()` call.
    // TODO: to refact after the camera, need to pass only one arg
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
    pub fn write_texture(
        &self,
        texture_image_metadatas: &[Vec<TextureImageMetadata>],
        texture: DiffuseTexture,
    ) -> Result<(), QueueError> {
        // TODO: better add a ref to metas inside irid Texture structs
        let metadatas = &texture_image_metadatas[log2(texture.size().width() as i32) as usize]
            [log2(texture.size().height() as i32) as usize];

        let bytes = texture.image().as_rgba8_bytes().ok_or(
            // It's ok to have a clone here, is only called if an error occurs
            QueueError::RgbaTextureNoneBytes {
                path: texture.path().clone(),
            },
        )?;

        self.wgpu_queue.write_texture(
            metadatas.create_image_copy(),
            bytes,
            *metadatas.image_data_layout(),
            *metadatas.image_size(),
        );

        Ok(())
    }

    /// Submits a series of finished command buffers for execution.
    pub fn submit<I: IntoIterator<Item = wgpu::CommandBuffer>>(&self, command_buffers: I) {
        let _ = self.wgpu_queue.submit(command_buffers);
    }

    /// Gets the amount of nanoseconds each tick of a timestamp query represents.
    ///
    /// Returns zero if timestamp queries are unsupported.
    pub fn get_timestamp_period(&self) -> f32 {
        self.wgpu_queue.get_timestamp_period()
    }
}
