use anyhow::{Result, Context};
use morton_encoding::morton_decode;
use tobj::LoadOptions;
use wgpu::util::DeviceExt;
use std::path::Path;
use std::{ops::Range};

use crate::fetch::fetch_bytes;
use crate::render::texture;

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
}

impl Vertex for ModelVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute { 
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

impl Model {
    pub async fn load<P: AsRef<Path>>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        path: P,
    ) -> Result<Self> {
        let loader_opts = LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        };
        let p: &Path = path.as_ref();
        let p = p.canonicalize().unwrap_or(p.to_path_buf());

        let obj_bytes = fetch_bytes(p.to_str().expect("Path contained invalid unicode")).await?;
        let mut obj_buf = std::io::BufReader::new(&obj_bytes[..]);
        let (obj_models, obj_materials) = tobj::load_obj_buf_async(
            &mut obj_buf, 
            &loader_opts, 
            |mat_path: String| async {
                let full_path = match p.parent() {
                    Some(parent) => parent.join(mat_path),  
                    None => std::path::Path::new(&mat_path).to_path_buf(),
                };
                let full_path = full_path.canonicalize().unwrap_or(full_path);

                let mat_bytes = fetch_bytes(full_path).await.expect("Failed to fetch material bytes");
                let mut mat_buf = std::io::BufReader::new(&mat_bytes[..]);
                tobj::load_mtl_buf(&mut mat_buf)
            }).await?;

        let obj_materials = obj_materials?;
        let containing_folder = path.as_ref().parent()
            .context("Directory has no parent")?;

        let mut materials = Vec::new();
        for mat in obj_materials {
            let diffuse_path = mat.diffuse_texture;
            let diffuse_texture = texture::Texture::load(device, queue, containing_folder.join(diffuse_path)).await?;

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    },
                ],
                label: None,
            });

            materials.push(Material {
                name: mat.name,
                diffuse_texture,
                bind_group,
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

            let vertex_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{:?} Vertex Buffer", path.as_ref())),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                }
            );

            let index_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{:?} Index Buffer", path.as_ref())),
                    contents: bytemuck::cast_slice(&m.mesh.indices),
                    usage: wgpu::BufferUsages::INDEX,
                }
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

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

impl Mesh {
    pub fn mesh_chunk(
        device: &wgpu::Device,
        chunk: &common::Chunk,
    ) -> Self {
        let positions = &[
            [0.0f32, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ];

        let tex_coords = [
            [0.0f32, 0.0],
            [1.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0],
        ];

        let normals = [
            [0.0f32, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, -1.0],
            [0.0, -1.0, 0.0],
            [1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
        ];

        let vertices: Vec<ModelVertex> = chunk.coord_iter().flat_map(|(xyz, block)| {
            if block.id == 0 {
                return vec![].into_iter();
            }
            let (x, y, z) = (xyz.x, xyz.y, xyz.z);

            let pos = positions.iter()
                .map(|[dx, dy, dz]| [x as f32 + dx, y as f32 + dy, z as f32 + dz])
                .collect::<Vec<_>>();

            let vert = |v:usize, tc:usize, n:usize| {
                ModelVertex {
                    position: pos[v],
                    tex_coords: tex_coords[tc],
                    normal: normals[n],
                }
            };
            let mut v = vec![];

            // Back face
            if usize::from(z) == common::Chunk::DIMENSION - 1 || !chunk.has_block(x, y, z + 1) {
                let back = [
                    vert(0, 0, 0), vert(1, 1, 0), vert(2, 2, 1),
                    vert(2, 2, 0), vert(1, 1, 0), vert(3, 3, 0),
                ];
                v.extend(back);
            }

            // Up face
            if y == 0 || !chunk.has_block(x, y - 1, z) { 
                let up = [
                    vert(6, 0, 3), vert(7, 1, 3), vert(0, 2, 3),
                    vert(0, 2, 3), vert(7, 1, 3), vert(1, 3, 3),
                ];
                v.extend(up);
            }

            // Front face
            if z == 0 || !chunk.has_block(x, y, z - 1) {
                let front = [
                    vert(4, 3, 2), vert(5, 2, 2), vert(6, 1, 2),
                    vert(6, 1, 2), vert(5, 2, 2), vert(7, 0, 2),
                ];
                v.extend(front);
            }

            // Down face
            if usize::from(y) == common::Chunk::DIMENSION - 1 || !chunk.has_block(x, y + 1, z) {
                let down = [
                    vert(2, 0, 1), vert(3, 1, 1), vert(4, 2, 1),
                    vert(4, 2, 1), vert(3, 1, 1), vert(5, 3, 1),
                ];
                v.extend(down);
            } else {
                log::info!("({}, {}, {}) has block at ({}, {}, {})", x, y, z, x, y + 1, z);
            }  

            if usize::from(x) == common::Chunk::DIMENSION-1 || !chunk.has_block(x + 1, y, z) {
                let right = [
                    vert(1, 0, 4), vert(7, 1, 4), vert(3, 2, 4),
                    vert(3, 2, 4), vert(7, 1, 4), vert(5, 3, 4),
                ];
                v.extend(right);
            }

            if x == 0 || !chunk.has_block(x - 1, y, z) {
                let left = [
                    vert(6, 0, 5), vert(0, 1, 5), vert(4, 2, 5),
                    vert(4, 2, 5), vert(0, 1, 5), vert(2, 3, 5),
                ];
                v.extend(left);
            }

            v.into_iter()
        }).collect();

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Chunk Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let indices: Vec<u32> = (0..vertices.len() as u32).collect();
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Chunk Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        Self {
            name: "chunk".to_string(),
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            material: 0usize,
        }
    }
}


pub trait DrawModel<'a> {
    fn draw_mesh(&mut self, mesh: &'a Mesh, material: &'a Material, camera_bind_group: &'a wgpu::BindGroup);
    fn draw_mesh_instanced(
        &mut self, 
        mesh: &'a Mesh,
        material: &'a Material,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup
    );

    fn draw_model(&mut self, model: &'a Model, camera_bind_group: &'a wgpu::BindGroup);
    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
    );
}
impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
   fn draw_mesh(&mut self, mesh: &'b Mesh, material: &'a Material, camera_bind_group: &'b wgpu::BindGroup) { 
       self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group);
   }

   fn draw_mesh_instanced(
       &mut self,
       mesh: &'b Mesh,
       material: &'b Material,
       instances: Range<u32>,
       camera_bind_group: &'a wgpu::BindGroup
    ) {
       self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
       self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
       self.set_bind_group(0, &material.bind_group, &[]);
       self.set_bind_group(1, camera_bind_group,&[]);
       self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_model(&mut self, model: &'b Model, camera_bind_group: &'b wgpu::BindGroup) {
        self.draw_model_instanced(model, 0..1, camera_bind_group);
    }

    fn draw_model_instanced(
        &mut self,
        model: &'b Model,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material];
            self.draw_mesh_instanced(mesh, material, instances.clone(), camera_bind_group);
        }
    }
}
