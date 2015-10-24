use std::mem;
use std::ops;

use buffer::buffer_view::BufferView;
use vertex::Width;

#[derive(Debug, Clone)]
pub struct ScalarTypedView<'a, T: 'a> {
  pub name: Option<String>,

  pub offset: usize,
  pub stride: usize,
  pub length: usize,

  data: &'a [T]
}

impl<'a, T: 'a> ScalarTypedView<'a, T> {
  pub fn new(name: Option<String>, view: &'a BufferView, offset: usize, stride: usize, length: usize) -> ScalarTypedView<'a, T> {
    let stride = if stride == 0 { mem::align_of::<T>() } else { stride };

    let stride_unaligned = stride % mem::align_of::<T>() != 0;
    let offset_unaligned = offset % mem::align_of::<T>() != 0;

    if stride_unaligned || offset_unaligned {
      panic!("Buffer would not be aligned, that could crash the whole application!")
    }

    return ScalarTypedView {
      name: name,
      stride: stride / mem::size_of::<T>(),
      offset: offset / mem::size_of::<T>(),
      length: length,
      data: unsafe { mem::transmute(&view[..]) }
    };
  }
}

impl<'a, T: 'a> ops::Index<usize> for ScalarTypedView<'a, T> {
  type Output = T;

  #[inline(always)]
  fn index(&self, index: usize) -> &T {
    assert!(index <= self.length);

    return &self.data[self.stride * index + self.offset];
  }
}

pub struct TypedView<'a, T: 'a> {
  width: Width,
  scalar_view: ScalarTypedView<'a, T>
}

impl<'a, T: 'a> TypedView<'a, T> {
  pub fn new(name: Option<String>, view: &'a BufferView, width: Width, offset: usize, stride: usize, length: usize) -> TypedView<'a, T> {
    return TypedView {
      width: width,
      scalar_view: ScalarTypedView::new(name, view, offset, stride, length)
    };
  }
}

impl<'a, T: 'a> ops::Index<usize> for TypedView<'a, T> {
  type Output = [T];

  #[inline(always)]
  fn index(&self, index: usize) -> &[T] {
    let from = self.scalar_view.stride * index + self.scalar_view.offset;
    let elements = self.width.elements();

    assert!(index <= self.scalar_view.length);

    return &self.scalar_view.data[from .. from + elements];
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use buffer::Buffer;
  use buffer::buffer_view::BufferView;
  use vertex::Width;

  #[test]
  fn test_scalar_index() {
    let buffer = Buffer::new(None, None, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    let view = BufferView::new(None, buffer, 0, 8);
    let tv = ScalarTypedView::<u16>::new(None, &view, 2, 4, 2);

    assert_eq!(&tv[0], &0x0302);
    assert_eq!(&tv[1], &0x0706);
  }

  #[test]
  fn test_index() {
    let buffer = Buffer::new(None, None, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let view = BufferView::new(None, buffer, 0, 10);
    let tv = TypedView::<u16>::new(None, &view, Width::Vector2, 2, 4, 2);

    assert_eq!(&tv[0], &[0x0302, 0x0504]);
    assert_eq!(&tv[1], &[0x0706, 0x0908]);
  }
}
