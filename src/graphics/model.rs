extern crate glm;
extern crate gl;
use self::gl::types::*;

use std::fs::File;
use std::io::Read;
use std::mem;
use std::str;
use graphics::shader;
use graphics::texture::Texture;

pub struct Mesh
{
    pub vertices: Vec<glm::Vec3>,
    pub uv_coords: Vec<glm::Vec2>,
    pub normals: Vec<glm::Vec3>
}

impl Mesh
{
    pub fn new(path: &str) -> Mesh {
        let mut data = String::new();
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("Error could not open {}", path);
                panic!(e);
            }
        };

        file.read_to_string(&mut data).expect("Unable to read data");

        Mesh::parse_obj(&data)
    }

    fn parse_obj(data: &str) -> Mesh
    {
        let mut temp_vertices: Vec<glm::Vec3> = Vec::new();
        let mut temp_uv_coords: Vec<glm::Vec2> = Vec::new();
        let mut temp_normals: Vec<glm::Vec3> = Vec::new();

        let mut temp_vertex_indices: Vec<usize> = Vec::new();
        let mut temp_uv_indices: Vec<usize> = Vec::new();
        let mut temp_normal_indices: Vec<usize> = Vec::new();

        let mut out_vertices: Vec<glm::Vec3> = Vec::new();
        let mut out_uvs: Vec<glm::Vec2> = Vec::new();
        let mut out_normals: Vec<glm::Vec3> = Vec::new();

        for line in data.lines()
        {
            let line_data: Vec<&str> = line.split_whitespace().collect();

            match line_data[0] {
                // parse vertices
                "v" => {
                    let mut verts = vec![0.0; 3];

                    for i in 1..3 {
                        let val = line_data[i].parse::<f32>().expect("Unable to read vertex");
                        verts[i - 1] = val;
                    }
                    temp_vertices.push(glm::Vec3::new(verts[0], verts[1], verts[2]));
                }
                // parse uv coordinates
                "vt" => {
                    let mut uvs = vec![0.0; 2];

                    for i in 1..2 {
                        let val = line_data[i].parse::<f32>().expect("Unable to read uv coordinate");
                        uvs[i - 1] = val;
                    }

                    temp_uv_coords.push(glm::Vec2::new(uvs[0], uvs[1]));
                }
                // parse normals
                "vn" => {
                    let mut norms = vec![0.0; 3];

                    for i in 1..3 {
                        let val = line_data[i].parse::<f32>().expect("Unable to read normal");
                        norms[i - 1] = val;
                    }

                    temp_normals.push(glm::Vec3::new(norms[0], norms[1], norms[2]));
                }

                "f" => {
                    let first = line_data[1].split('/').collect::<Vec<&str>>();
                    let second = line_data[2].split('/').collect::<Vec<&str>>();
                    let third = line_data[3].split('/').collect::<Vec<&str>>();

                    temp_vertex_indices.push(first[0].parse::<usize>().expect("malformed file"));
                    temp_vertex_indices.push(second[0].parse::<usize>().expect("malformed file"));
                    temp_vertex_indices.push(third[0].parse::<usize>().expect("malformed file"));

                    temp_uv_indices.push(first[1].parse::<usize>().expect("malformed fiel"));
                    temp_uv_indices.push(second[1].parse::<usize>().expect("malformed file"));
                    temp_uv_indices.push(third[1].parse::<usize>().expect("malformed file"));

                    temp_normal_indices.push(first[2].parse::<usize>().expect("malformed file"));
                    temp_normal_indices.push(second[2].parse::<usize>().expect("malformed file"));
                    temp_normal_indices.push(third[2].parse::<usize>().expect("malformed file"));

                }
                _ => println!("Other value {}", line_data[0])
            }
        }

        for i in 0..temp_vertex_indices.len() {
            let vertex_index = temp_vertex_indices[i];
            let uv_index = temp_uv_indices[i];
            let normal_index = temp_normal_indices[i];

            // subtract 1 because OBJ indices start at 1, but Rust array indices start at 0
            let mesh_vertex = temp_vertices[normal_index - 1];
            let uv = temp_uv_coords[uv_index - 1];
            let normal = temp_normals[normal_index - 1];

            out_vertices.push(mesh_vertex);
            out_uvs.push(uv);
            out_normals.push(normal);
        }

        Mesh {
            vertices: out_vertices,
            uv_coords: out_uvs,
            normals: out_normals
        }
    }
}

pub struct Model
{
    pub mesh: Mesh,
    pub vao: u32,
    pub vertex_buffer: u32,
    pub uv_buffer: u32,
    pub normal_buffer: u32,
    pub texture_id: u32
}

pub struct ModelBuilder
{
    pub mesh: Mesh,
    pub vao: u32,
    pub vertex_buffer: u32,
    pub uv_buffer: u32,
    pub normal_buffer: u32,
    pub texture_id: u32
}

impl ModelBuilder {
    pub fn new(path: &str) -> ModelBuilder {
        let mut data = String::new();
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                panic!("Error! Could not open {} Description: {}", path, e);
            }
        };

        file.read_to_string(&mut data).expect("Unable to read data");

        let mesh = Mesh::new(path);

        let mut vao = 0;
        let mut vertex_buffer = 0;
        let mut uv_buffer = 0;
        let mut normal_buffer = 0;

        unsafe {
            // create vertex array object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // generate vertex array buffer and copy vertex data into it
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (mesh.vertices.len() * mem::size_of::<glm::Vec3>()) as GLsizeiptr,
                            mem::transmute(&mesh.vertices[0]), gl::STATIC_DRAW);

            // generate uv array buffer and copy uv data into it
            gl::GenBuffers(1, &mut uv_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, uv_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (mesh.uv_coords.len() * mem::size_of::<glm::Vec2>()) as GLsizeiptr,
                            mem::transmute(&mesh.uv_coords[0]), gl::STATIC_DRAW);

            gl::GenBuffers(1, &mut normal_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, normal_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (mesh.normals.len() * mem::size_of::<glm::Vec3>()) as GLsizeiptr,
                            mem::transmute(&mesh.normals[0]), gl::STATIC_DRAW);
        }

        ModelBuilder {
            mesh: mesh,
            vao: vao,
            vertex_buffer: vertex_buffer,
            uv_buffer: uv_buffer,
            normal_buffer: normal_buffer,
            texture_id: 1,
        }
    }

    pub fn set_texture(mut self, texture: Texture) -> ModelBuilder {
        self.texture_id = texture.texture_id;
        self
    }

    pub fn finalize(self) -> Model {
        Model {
            mesh: self.mesh,
            vao: self.vao,
            vertex_buffer: self.vertex_buffer,
            uv_buffer: self.uv_buffer,
            normal_buffer: self.normal_buffer,
            texture_id: self.texture_id,
        }
    }
}
