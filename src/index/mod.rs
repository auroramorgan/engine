#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Format {
  u16, u32
}

impl Format {
  pub fn byte_size(&self) -> usize {
    return match *self {
      Format::u16 => 2,
      Format::u32 => 4
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
