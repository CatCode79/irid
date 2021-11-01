//= USES ===========================================================================================

use irid_renderer_traits::Vertex;

//= MODEL OBJECT ===================================================================================

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

pub struct Material {
    pub name: String,
    pub texture: DiffuseTexture,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

impl Model {
    ///
    // TODO also here I have to remove at least surface param
    pub fn load<P: AsRef<std::path::Path>>(
        surface: &Surface,
        device: &Device,
        path: P,
    ) -> anyhow::Result<Self> {
        let (obj_models, obj_materials) = tobj::load_obj(
            path.as_ref(),
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            }
        )?;

        let obj_materials = obj_materials?;

        // We're assuming that the texture files are stored with the obj file
        use anyhow::Context;
        let containing_folder = path.as_ref().parent()
            .context("Directory has no parent")?;

        let mut materials = Vec::new();
        for mat in obj_materials {
            use std::ops::Deref;
            let filepath = containing_folder.join(mat.diffuse_texture);
            let texture = DiffuseTexture::load(surface, device, filepath.deref())?;

            materials.push(Material {
                name: mat.name,
                texture,
            });
        }

        let mut meshes = Vec::new();
        for m in obj_models {
            let mut vertices = Vec::new();
            for i in 0..m.mesh.positions.len() / 3 {
                vertices.push(ModelVertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                    normal: [
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2],
                    ],
                });
            }

            let vertex_buffer = device.create_vertex_buffer_init(
                &format!("{:?} Vertex Buffer", path.as_ref()),
                vertices.as_slice(),
            );

            let index_buffer = device.create_indices_buffer_init(
                &format!("{:?} Index Buffer", path.as_ref()),
                m.mesh.indices.as_slice(),
            );

            meshes.push(Mesh {
                name: m.name,
                vertex_buffer,
                index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            });
        }

        Ok(Self { meshes, materials })
    }
}
