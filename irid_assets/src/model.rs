//= USES =====================================================================

/*use crate::{GenericImage, GenericVertex};

//= MODEL INTERFACE ==========================================================

///
pub trait GenericModel {
    type Mdl;

    fn load<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self::Mdl>;
}

//= MODEL OBJECT =============================================================

///
pub struct Model<I: GenericImage, V: GenericVertex> {
    pub meshes: Vec<Mesh<V>>,
    pub materials: Vec<Material<I>>,
}

///
pub struct Material<I: GenericImage> {
    pub name: String,
    pub image: I,
}

///
pub struct Mesh<V: GenericVertex> {
    pub name: String,
    pub vertices: Vec<V>,
    pub num_elements: u32,
    pub indices: Vec<u32>,
    pub material: usize,
}

impl<I, V> Model<I, V> {
    ///
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
            let texture = I::load(&filepath)?;

            materials.push(Material {
                name: mat.name,
                image: texture,
            });
        }

        let mut meshes = Vec::new();
        for om in obj_models {
            let mut vertices = Vec::new();
            for i in 0..om.mesh.positions.len() / 3 {
                let mut vertex = V::new();
                vertex.position([
                    om.mesh.positions[i * 3],
                    om.mesh.positions[i * 3 + 1],
                    om.mesh.positions[i * 3 + 2],
                ]);
                vertex.tex_coords([
                    om.mesh.texcoords[i * 2],
                    om.mesh.texcoords[i * 2 + 1]
                ]);
                vertex.normal([
                    om.mesh.normals[i * 3],
                    om.mesh.normals[i * 3 + 1],
                    om.mesh.normals[i * 3 + 2],
                ]);
                vertices.push(vertex);
            }

            meshes.push(Mesh {
                name: om.name,
                vertices,
                num_elements: om.mesh.indices.len() as u32,
                indices: om.mesh.indices,
                material: om.mesh.material_id.unwrap_or(0),
            });
        }

        Ok(Self { meshes, materials })
    }
}*/
