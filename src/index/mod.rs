#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Format {
  u8, u16, u32, u64
}

impl Format {
  pub fn byte_size(&self) -> usize {
    return match *self {
      Format::u8 => 1,
      Format::u16 => 2,
      Format::u32 => 4,
      Format::u64 => 8
    };
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Geometry {
  Points,
  Lines,
  Triangles,
  TriangleStrips
}
