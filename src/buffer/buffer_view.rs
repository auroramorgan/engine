use std::ops;

use std::sync::Arc;

use buffer::Buffer;

#[derive(Debug, Clone)]
pub struct BufferView {
  pub name: Option<String>,

  pub buffer: Arc<Buffer>,
  pub offset: usize,
  pub length: usize
}

impl BufferView {
  pub fn new(name: Option<String>, buffer: Arc<Buffer>, offset: usize, length: usize) -> Arc<BufferView> {
    return Arc::new(BufferView { name: name, buffer: buffer, offset: offset, length: length });
  }

  #[inline(always)]
  pub fn as_slice(&self) -> &[u8] {
    return &self[..];
  }
}

impl ops::Index<usize> for BufferView {
  type Output = u8;

  #[inline(always)]
  fn index(&self, index: usize) -> &u8 {
    assert!(index <= self.length);

    return &self.buffer[self.offset + index];
  }
}

impl ops::Index<ops::Range<usize>> for BufferView {
  type Output = [u8];

  #[inline(always)]
  fn index(&self, index: ops::Range<usize>) -> &[u8] {
    let ops::Range { start, end } = index;

    assert!(start <= self.length);
    assert!(end <= self.length);

    return &self.buffer[self.offset + start .. self.offset + end];
  }
}

impl ops::Index<ops::RangeTo<usize>> for BufferView {
  type Output = [u8];

  #[inline(always)]
  fn index(&self, index: ops::RangeTo<usize>) -> &[u8] {
    let ops::RangeTo { end } = index;

    return &self[0 .. end];
  }
}

impl ops::Index<ops::RangeFrom<usize>> for BufferView {
  type Output = [u8];

  #[inline(always)]
  fn index(&self, index: ops::RangeFrom<usize>) -> &[u8] {
    let ops::RangeFrom { start } = index;

    return &self[start .. self.length];
  }
}

impl ops::Index<ops::RangeFull> for BufferView {
  type Output = [u8];

  #[inline(always)]
  fn index(&self, _index: ops::RangeFull) -> &[u8] {
    return &self[0 .. self.length];
  }
}

impl ops::Deref for BufferView {
  type Target = [u8];

  fn deref(&self) -> &[u8] {
    return &self[..];
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use buffer::Buffer;

  #[test]
  fn test_as_slice() {
    let buffer = Buffer::new(None, None, vec![0, 1, 2, 3]);
    let view = BufferView::new(None, buffer, 1, 2);

    assert_eq!(view.as_slice(), &[1, 2]);
  }

  #[test]
  fn test_deref() {
    let buffer = Buffer::new(None, None, vec![0, 1, 2, 3]);
    let view = BufferView::new(None, buffer, 1, 2);

    assert_eq!(&**view, &[1, 2]);
  }

  #[test]
  fn test_index() {
    let buffer = Buffer::new(None, None, vec![0, 1, 2, 3]);
    let view = BufferView::new(None, buffer, 1, 2);

    assert_eq!(&view[..], &[1, 2]);
    assert_eq!(&view[1 .. 2], &[2]);
    assert_eq!(&view[.. 1], &[1]);
    assert_eq!(&view[1 ..], &[2]);
  }
}
