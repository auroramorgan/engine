#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Scalar {
  f16, f32,
  u8, u16, u32,
  u8_normalized, u16_normalized, u32_normalized,
  i8, i16, i32,
  i8_normalized, i16_normalized, i32_normalized
}

impl Scalar {
  #[inline(always)]
  pub fn byte_size(&self) -> usize {
    return match *self {
      Scalar::u8 | Scalar::u8_normalized | Scalar::i8 | Scalar::i8_normalized => 1,
      Scalar::f16 | Scalar::u16 | Scalar::u16_normalized | Scalar::i16 | Scalar::i16_normalized => 2,
      Scalar::f32 | Scalar::u32 | Scalar::u32_normalized | Scalar::i32 | Scalar::i32_normalized => 4
    };
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Width {
  Scalar, Vector2, Vector3, Vector4, Matrix3x3
}

impl Width {
  pub fn from_integer(n: usize) -> Option<Width> {
    return match n {
      1 => Some(Width::Scalar),
      2 => Some(Width::Vector2),
      3 => Some(Width::Vector3),
      4 => Some(Width::Vector4),
      9 => Some(Width::Matrix3x3),
      _ => None
    };
  }

  #[inline(always)]
  pub fn elements(&self) -> usize {
    return match *self {
      Width::Scalar => 1,
      Width::Vector2 => 2,
      Width::Vector3 => 3,
      Width::Vector4 => 4,
      Width::Matrix3x3 => 9
    };
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Format(pub Scalar, pub Width);

impl Format {
  #[inline(always)]
  pub fn scalar(&self) -> Scalar {
    let Format(scalar, _) = *self;

    return scalar;
  }

  #[inline(always)]
  pub fn width(&self) -> Width {
    let Format(_, width) = *self;

    return width;
  }

  #[inline(always)]
  pub fn elements(&self) -> usize {
    return self.width().elements();
  }

  #[inline(always)]
  pub fn byte_size(&self) -> usize {
    return match *self {
      Format(scalar, width) => scalar.byte_size() * width.elements()
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
  pub format: Format,
  pub offset: usize,
  pub buffer_index: usize
}

impl Attribute {
  #[inline(always)]
  pub fn byte_size(&self) -> usize {
    return self.format.byte_size();
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
  pub fn attribute_for(&self, name: &AttributeName) -> Option<&Attribute> {
    for attribute in &self.attributes {
      if attribute.name == *name {
        return Some(attribute);
      }
    }

    return None;
  }
}
