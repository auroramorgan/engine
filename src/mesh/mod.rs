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
  pub fn attribute_for(&self, name: &vertex::AttributeName) -> Option<&vertex::Attribute> {
    return self.descriptor.attribute_for(name);
  }

  pub fn f32_view_for<'a>(&'a self, name: &vertex::AttributeName) -> Option<view::F32OmniView<'a>> {
    let attribute = match self.attribute_for(name) {
      Some(a) => a, None => return None
    };

    let buffer_index = attribute.buffer_index;
    let buffer = self.buffers[buffer_index].as_slice();

    let layout = self.descriptor.layouts[buffer_index];
    let stride = layout.stride;

    return Some(view::F32OmniView::new(buffer, attribute, self.vertex_count, stride));
  }
}
