use std::ops;

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Buffer {
  pub uri: Option<String>,
  pub name: Option<String>,

  data: Vec<u8>
}

impl Buffer {
  pub fn new(uri: Option<String>, name: Option<String>, data: Vec<u8>) -> Arc<Buffer> {
    return Arc::new(Buffer { uri: uri, name: name, data: data });
  }

  #[inline(always)]
  pub fn as_slice(&self) -> &[u8] {
    return &self[..];
  }
}

impl ops::Index<usize> for Buffer {
  type Output = u8;

  #[inline(always)]
  fn index(&self, index: usize) -> &u8 {
    return &self.data[index];
  }
}

impl ops::Index<ops::Range<usize>> for Buffer {
  type Output = [u8];

  #[inline(always)]
  fn index(&self, index: ops::Range<usize>) -> &[u8] {
    return &self.data[index];
  }
}

impl ops::Index<ops::RangeTo<usize>> for Buffer {
  type Output = [u8];

  #[inline(always)]
  fn index(&self, index: ops::RangeTo<usize>) -> &[u8] {
    return &self.data[index];
  }
}

impl ops::Index<ops::RangeFrom<usize>> for Buffer {
  type Output = [u8];

  #[inline(always)]
  fn index(&self, index: ops::RangeFrom<usize>) -> &[u8] {
    return &self.data[index];
  }
}

impl ops::Index<ops::RangeFull> for Buffer {
  type Output = [u8];

  #[inline(always)]
  fn index(&self, index: ops::RangeFull) -> &[u8] {
    return &self.data[index];
  }
}

impl ops::Deref for Buffer {
  type Target = [u8];

  fn deref(&self) -> &[u8] {
    return self.data.deref();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_as_slice() {
    let buffer = Buffer::new(None, None, vec![0, 1, 2, 3]);

    assert_eq!(buffer.as_slice(), &[0, 1, 2, 3]);
  }

  #[test]
  fn test_deref() {
    let buffer = Buffer::new(None, None, vec![0, 1, 2, 3]);

    assert_eq!(&**buffer, &[0, 1, 2, 3]);
  }

  #[test]
  fn test_index() {
    let buffer = Buffer::new(None, None, vec![0, 1, 2, 3]);

    assert_eq!(&buffer[..], &[0, 1, 2, 3]);
    assert_eq!(&buffer[1 .. 3], &[1, 2]);
    assert_eq!(&buffer[.. 2], &[0, 1]);
    assert_eq!(&buffer[2 ..], &[2, 3]);
  }
}
