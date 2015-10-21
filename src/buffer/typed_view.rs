use std::mem;
use std::ops;

use buffer::buffer_view::BufferView;

#[derive(Debug, Clone)]
pub struct TypedView<'a, T: 'a> {
  name: Option<String>,

  offset: usize,
  stride: usize,
  length: usize,

  data: &'a [T]
}

impl<'a, T: 'a> TypedView<'a, T> {
  pub fn new(name: Option<String>, view: &'a BufferView, offset: usize, stride: usize, length: usize) -> TypedView<'a, T> {
    let stride_unaligned = stride % mem::align_of::<T>() != 0;
    let offset_unaligned = offset % mem::align_of::<T>() != 0;

    if stride_unaligned || offset_unaligned {
      panic!("Buffer would not be aligned, that could crash the whole application!")
    }

    return TypedView {
      name: name,
      stride: stride / mem::size_of::<T>(),
      offset: offset / mem::size_of::<T>(),
      length: length,
      data: unsafe { mem::transmute(&view[..]) }
    };
  }
}

impl<'a, T: 'a> ops::Index<usize> for TypedView<'a, T> {
  type Output = T;

  #[inline(always)]
  fn index(&self, index: usize) -> &T {
    assert!(index <= self.length);

    return &self.data[self.stride * index + self.offset];
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use buffer::Buffer;
  use buffer::buffer_view::BufferView;

  #[test]
  fn test_index() {
    let buffer = Buffer::new(None, None, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    let view = BufferView::new(None, buffer, 0, 8);
    let tv = TypedView::<u16>::new(None, &view, 2, 4, 2);

    assert_eq!(&tv[0], &0x0302);
    assert_eq!(&tv[1], &0x0706);
  }
}
