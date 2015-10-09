use mesh;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
  Mesh(mesh::Mesh)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Asset {
  pub objects: Vec<Object>
}
