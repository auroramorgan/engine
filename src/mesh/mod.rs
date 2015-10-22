use std::sync::Arc;

use index;
use vertex;

use buffer::{BufferView, TypedView};

#[derive(Debug, Clone)]
pub struct Submesh {
  pub name: String,
  pub view: Arc<BufferView>,
  pub index_count: usize,
  pub index_format: index::Format,
  pub geometry: index::Geometry
}

impl Submesh {
  pub fn faces<'a>(&'a self) -> FaceIterator<'a> {
    return FaceIterator { submesh: self, current: 0 };
  }
}

pub struct FaceIterator<'a> {
  submesh: &'a Submesh,
  current: usize
}

impl<'a> Iterator for FaceIterator<'a> {
  type Item = Vec<usize>;

  fn next(&mut self) -> Option<Vec<usize>> {
    if self.current >= self.len() {
      return None;
    }

    let mut indices = match self.submesh.geometry {
      index::Geometry::Points => {
        let offset = self.current;
        vec![offset + 0]
      }
      index::Geometry::Lines => {
        let offset = self.current;
        vec![offset + 0, offset + 1]
      }
      index::Geometry::TriangleStrips => {
        let offset = self.current;
        vec![offset + 0, offset + 1, offset + 2]
      }
      index::Geometry::Triangles => {
        let offset = 3 * self.current;
        vec![offset + 0, offset + 1, offset + 2]
      }
    };

    match self.submesh.index_format {
      index::Format::u16 => {
        let view = TypedView::<u16>::new(None, &self.submesh.view, 0, 0, self.submesh.index_count);

        for i in 0 .. indices.len() {
          indices[i] = view[indices[i]] as usize
        }
      }
      index::Format::u32 => {
        let view = TypedView::<u32>::new(None, &self.submesh.view, 0, 0, self.submesh.index_count);

        for i in 0 .. indices.len() {
          indices[i] = view[indices[i]] as usize
        }
      }
    }

    self.current += 1;

    return Some(indices);
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let length = self.len();

    return (length, Some(length));
  }
}

impl<'a> ExactSizeIterator for FaceIterator<'a> {
  #[inline(always)]
  fn len(&self) -> usize {
    let index_count = self.submesh.index_count;

    return match self.submesh.geometry {
      index::Geometry::Points => index_count,
      index::Geometry::Lines => index_count - 1,
      index::Geometry::TriangleStrips => index_count - 2,
      index::Geometry::Triangles => index_count / 3
    };
  }
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
