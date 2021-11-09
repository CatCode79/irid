//= USES ===========================================================================================

use irid_assets_traits::Image;
use irid_renderer_traits::Vertex;

//= MODEL OBJECT ===================================================================================

pub struct Model<'a, V: Vertex, I: Image> {
    pub meshes: Vec<Mesh<'a, V>>,
    pub materials: Vec<Material<I>>,
}

pub struct Material<I: Image> {
    pub name: String,
    pub image: I,
}

pub struct Mesh<'a, V: Vertex> {
    pub name: String,
    pub vertices: &'a [V],
    pub indices: &'a [u32],
    pub num_elements: u32,
    pub material: usize,
}

impl<'a, V: Vertex, I: Image> Model<'a, V, I> {
    ///
    // TODO also here I have to remove at least surface param
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
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
            let filepath = containing_folder.join(mat.diffuse_texture);
            let texture = I::load(*filepath)?;

            materials.push(Material {
                name: mat.name,
                image: texture,
            });
        }

        let mut meshes = Vec::new();
        for obj_model in obj_models {
            let mut vertices = Vec::new();
            for i in 0..obj_model.mesh.positions.len() / 3 {
                let mut vertex = V::new();
                vertex.position([
                    obj_model.mesh.positions[i * 3],
                    obj_model.mesh.positions[i * 3 + 1],
                    obj_model.mesh.positions[i * 3 + 2],
                ]);
                vertex.tex_coords([
                    obj_model.mesh.texcoords[i * 2],
                    obj_model.mesh.texcoords[i * 2 + 1]
                ]);
                vertex.normal([
                    obj_model.mesh.normals[i * 3],
                    obj_model.mesh.normals[i * 3 + 1],
                    obj_model.mesh.normals[i * 3 + 2],
                ]);
                vertices.push(vertex);
            }

            meshes.push(Mesh {
                name: obj_model.name,
                vertices: vertices.as_slice(),
                indices: obj_model.mesh.indices.as_slice(),
                num_elements: obj_model.mesh.indices.len() as u32,
                material: obj_model.mesh.material_id.unwrap_or(0),
            });
        }

        Ok(Self { meshes, materials })
    }
}
