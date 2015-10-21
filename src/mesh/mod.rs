use std::sync::Arc;

use index;
use vertex;

use buffer::BufferView;

#[derive(Debug, Clone)]
pub struct Submesh {
  pub name: String,
  pub view: Arc<BufferView>,
  pub index_count: usize,
  pub index_format: index::Format,
  pub geometry: index::Geometry
}

#[derive(Debug, Clone)]
pub struct Mesh {
  pub name: String,
  pub vertex_count: usize,
  pub descriptor: vertex::Descriptor,
  pub buffers: Vec<Arc<BufferView>>,
  pub submeshes: Vec<Submesh>
}

impl Mesh {
  pub fn attribute_for(&self, name: &vertex::AttributeName) -> Option<&vertex::Attribute> {
    return self.descriptor.attribute_for(name);
  }
}
