extern crate glm;

pub struct Vertex
{
    pub vertices: glm::Vec3,
    pub uv_coords: glm::Vec2,
    pub normals: glm::Vec3
}

impl Vertex {
    pub fn new(verts: glm::Vec3, uvs: glm::Vec2, norms: glm::Vec3) -> Vertex {
        Vertex {vertices: verts, uv_coords: uvs, normals: norms}
    }
}
