use std::sync::Arc;

use index;
use vertex;
use vertex::{Format, Scalar};

use buffer::{BufferView, TypedView, ScalarTypedView, UntypedView, ScalarUntypedView};

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
    return FaceIterator { view: self.untyped_view(), geometry: self.geometry, current: 0 };
  }
  
  pub fn untyped_view<'a>(&'a self) -> ScalarUntypedView<'a> {
    let view = &self.view;
    let length = self.index_count;

    return match self.index_format {
      index::Format::u16 => ScalarUntypedView::u16(ScalarTypedView::<u16>::new(None, view, 0, 0, length)),
      index::Format::u32 => ScalarUntypedView::u32(ScalarTypedView::<u32>::new(None, view, 0, 0, length))
    };
  }
}

pub struct FaceIterator<'a> {
  view: ScalarUntypedView<'a>,
  geometry: index::Geometry,

  current: usize
}

impl<'a> Iterator for FaceIterator<'a> {
  type Item = Vec<usize>;

  fn next(&mut self) -> Option<Vec<usize>> {
    if self.current >= self.len() {
      return None;
    }

    let mut indices = match self.geometry {
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

    for i in 0 .. indices.len() {
      indices[i] = self.view.get_usize(indices[i]);
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
    let index_count = self.view.len();

    return match self.geometry {
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

  pub fn untyped_view_for<'a>(&'a self, name: &vertex::AttributeName) -> Option<UntypedView<'a>> {
    if let Some(attribute) = self.attribute_for(name) {

      let view = &self.buffers[attribute.buffer_index];

      let offset = attribute.offset;
      let stride = self.descriptor.layouts[attribute.buffer_index].stride;
      let length = self.vertex_count;

      return Some(match attribute.format {
        Format(Scalar::f16, width) => UntypedView::f16(TypedView::<i16>::new(None, view, width, offset, stride, length)),
        Format(Scalar::f32, width) => UntypedView::f32(TypedView::<f32>::new(None, view, width, offset, stride, length)),
        Format(Scalar::u8, width) => UntypedView::u8(TypedView::<u8>::new(None, view, width, offset, stride, length)),
        Format(Scalar::u16, width) => UntypedView::u16(TypedView::<u16>::new(None, view, width, offset, stride, length)),
        Format(Scalar::u32, width) => UntypedView::u32(TypedView::<u32>::new(None, view, width, offset, stride, length)),
        Format(Scalar::u8_normalized, width) => UntypedView::u8_normalized(TypedView::<u8>::new(None, view, width, offset, stride, length)),
        Format(Scalar::u16_normalized, width) => UntypedView::u16_normalized(TypedView::<u16>::new(None, view, width, offset, stride, length)),
        Format(Scalar::u32_normalized, width) => UntypedView::u32_normalized(TypedView::<u32>::new(None, view, width, offset, stride, length)),
        Format(Scalar::i8, width) => UntypedView::i8(TypedView::<i8>::new(None, view, width, offset, stride, length)),
        Format(Scalar::i16, width) => UntypedView::i16(TypedView::<i16>::new(None, view, width, offset, stride, length)),
        Format(Scalar::i32, width) => UntypedView::i32(TypedView::<i32>::new(None, view, width, offset, stride, length)),
        Format(Scalar::i8_normalized, width) => UntypedView::i8_normalized(TypedView::<i8>::new(None, view, width, offset, stride, length)),
        Format(Scalar::i16_normalized, width) => UntypedView::i16_normalized(TypedView::<i16>::new(None, view, width, offset, stride, length)),
        Format(Scalar::i32_normalized, width) => UntypedView::i32_normalized(TypedView::<i32>::new(None, view, width, offset, stride, length)),
      });
    }

    return None;
  }
}
