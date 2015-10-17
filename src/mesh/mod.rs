use index;
use vertex;

use view;

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

  pub fn f32_view_for(&self, name: vertex::AttributeName) -> Option<vertex::Attribute> {
    let attribute = self.attribute_for(name).unwrap_or_else(|| return None);

    let buffer_index = attribute.buffer_index;
    let buffer = self.buffers[buffer_index];

    let layout = self.descriptor.layout[buffer_index];
    let stride = layout.stride;

    return Some(view::F32OmniView::new(buffer.as_slice(), attribute, self.vertex_count, stride));
  }
}
