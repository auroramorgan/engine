#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Format {
  f16, f32, f64,
  i8_normalized, i8, i16_normalized, i16, i32_normalized, i32,
  u8_normalized, u8, u16_normalized, u16, u32_normalized, u32
}

impl Format {
  pub fn byte_size(&self) -> usize {
    return match *self {
      Format::f16 => 2,
      Format::f32 => 4,
      Format::f64 => 8,
      Format::i8_normalized  | Format::i8  => 1,
      Format::i16_normalized | Format::i16 => 2,
      Format::i32_normalized | Format::i32 => 4,
      Format::u8_normalized  | Format::u8  => 1,
      Format::u16_normalized | Format::u16 => 2,
      Format::u32_normalized | Format::u32 => 4
    };
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AttributeName {
  Ansiotropy,
  Binormal,
  Bitangent,
  Color,
  EdgeCrease,
  JointIndices,
  JointWeights,
  Normal,
  OcclusionValue,
  Position,
  ShadingBasisU,
  ShadingBasisV,
  SubdivisionStencil,
  Tangent,
  TextureCoordinate,
  Other(String)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
  pub name: AttributeName,
  pub offset: usize,
  pub ty: Format,
  pub elements: usize,
  pub buffer_index: usize
}

impl Attribute {
  pub fn byte_size(&self) -> usize {
    return self.ty.byte_size() * self.elements;
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BufferLayout {
  pub stride: usize
}

#[derive(Debug, PartialEq, Clone)]
pub struct Descriptor {
  pub attributes: Vec<Attribute>,
  pub layouts: Vec<BufferLayout>
}

impl Descriptor {
  pub fn attribute_for(&self, name: AttributeName) -> Option<Attribute> {
    for attribute in &self.attributes {
      if attribute.name == name {
        return Some(attribute.clone());
      }
    }

    return None;
  }
}
