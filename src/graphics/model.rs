extern crate glm;

use graphics::vertex::Vertex;

use std::fs::File;
use std::io::Read;

pub struct Model
{
    vertices: Vec<Vertex>,
    texture_id: i32
}

impl Model
{
    pub fn new(path: &str) -> Model
    {
        let mut data = String::new();
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                panic!("Error! Could not open {}", path);
            }
        };

        file.read_to_string(&mut data).expect("Unable to read data");

        Self::parse_obj(path)
    }

    fn parse_obj(data: &str) -> Model
    {
        let mut temp_vertices = Vec::<glm::Vec3>::new();
        let mut temp_uv_coords = Vec::<glm::Vec2>::new();
        let mut temp_normals = Vec::<glm::Vec3>::new();

        let mut temp_vertex_indices = Vec::<i32>::new();
        let mut temp_uv_indices = Vec::<i32>::new();
        let mut temp_normal_indices = Vec::<i32>::new();

        let mut vertices = Vec::<Vertex>::new();

        for line in data.lines()
        {
            let line_data: Vec<&str> = line.split_whitespace().collect();

            match line_data[0] {
                // parse vertices
                "v" => {
                    let mut verts = vec![0.0; 3];

                    for i in 1..3 {
                        let val = match line_data[i].parse::<f32>() {
                            Ok(n) => n,
                            Err(e) => panic!("Unable to read vertex")
                        };

                        verts[i - 1] = val;
                    }
                    temp_vertices.push(glm::Vec3::new(verts[0], verts[1], verts[2]));
                }
                // parse uv coordinates
                "vt" => {
                    let mut uvs = vec![0.0; 2];

                    for i in 1..2 {
                        let val = match line_data[i].parse::<f32>() {
                            Ok(n) => n,
                            Err(e) => panic!("Unable to read uv coordinate")
                        };

                        uvs[i - 1] = val;
                    }

                    temp_uv_coords.push(glm::Vec2::new(uvs[0], uvs[1]));
                }
                // parse normals
                "vn" => {
                    let mut norms = vec![0.0; 3];

                    for i in 1..3 {
                        let val = match line_data[i].parse::<f32>() {
                            Ok(n) => n,
                            Err(e) => panic!("Unable to read normal")
                        };

                        norms[i - 1] = val;
                    }

                    temp_normals.push(glm::Vec3::new(norms[0], norms[1], norms[2]));
                }

                "f" => {
                    let first = line_data[1].split('/').collect::<Vec<&str>>();
                    let second = line_data[2].split('/').collect::<Vec<&str>>();
                    let third = line_data[3].split('/').collect::<Vec<&str>>();

                    temp_vertex_indices.push(first[0].parse::<i32>().expect("malformed file"));
                    temp_vertex_indices.push(second[0].parse::<i32>().expect("malformed file"));
                    temp_vertex_indices.push(third[0].parse::<i32>().expect("malformed file"));

                    temp_uv_indices.push(first[1].parse::<i32>().expect("malformed fiel"));
                    temp_uv_indices.push(second[1].parse::<i32>().expect("malformed file"));
                    temp_uv_indices.push(third[1].parse::<i32>().expect("malformed file"));

                    temp_normal_indices.push(first[2].parse::<i32>().expect("malformed file"));
                    temp_normal_indices.push(second[2].parse::<i32>().expect("malformed file"));
                    temp_normal_indices.push(third[2].parse::<i32>().expect("malformed file"));

                }
                _ => println!("Other value {}", line_data[0])
            }
        }

        for i in 0..temp_vertex_indices.len() {
            let vertex_index = temp_vertex_indices[i];
            let uv_index = temp_uv_indices[i];
            let normal_index = temp_normal_indices[i];

            // subtract 1 because OBJ indices start at 1, but Rust array indices start at 0
            let mesh_vertex = temp_vertices[i - 1];
            let uv = temp_uv_coords[i - 1];
            let normal = temp_normals[i - 1];

            let vertex = Vertex::new(mesh_vertex, uv, normal);
            vertices.push(vertex);

        }

        Model {
            vertices: vertices,
            texture_id: 1,
        }
    }
}
