use mesh;

use animation::skeleton;

#[derive(Debug, PartialEq, Clone)]
pub struct Model {
  pub name: String,
  pub mesh: mesh::Mesh,
  pub blend_shapes: Vec<mesh::Mesh>,
  pub skeleton: skeleton::Skeleton,
}
