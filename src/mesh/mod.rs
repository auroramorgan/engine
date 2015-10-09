use index;
use vertex;

#[derive(Debug, PartialEq, Clone)]
pub struct Submesh {
  pub name: String,
  pub buffer: Vec<u8>,
  pub index_count: usize,
  pub index_format: index::Format,
  pub geometry: index::Geometry
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mesh {
  pub name: String,
  pub vertex_count: usize,
  pub descriptor: vertex::Descriptor,
  pub buffers: Vec<Vec<u8>>,
  pub submeshes: Vec<Submesh>
}

impl Mesh {
  pub fn attribute_for(&self, name: vertex::AttributeName) -> Option<vertex::Attribute> {
    return self.descriptor.attribute_for(name);
  }
}
