#![allow(non_camel_case_types)]

pub mod wbg;

#[derive(Debug, PartialEq, Clone)]
pub enum Name {
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
pub enum VertexFormat {
  f16, f32, f64,
  i8_normalized, i8, i16_normalized, i16, i32_normalized, i32,
  u8_normalized, u8, u16_normalized, u16, u32_normalized, u32
}

impl VertexFormat {
  pub fn byte_size(&self) -> usize {
    return match *self {
      VertexFormat::f16 => 2,
      VertexFormat::f32 => 4,
      VertexFormat::f64 => 8,
      VertexFormat::i8_normalized | VertexFormat::i8 => 1,
      VertexFormat::i16_normalized | VertexFormat::i16 => 2,
      VertexFormat::i32_normalized | VertexFormat::i32 => 4,
      VertexFormat::u8_normalized | VertexFormat::u8 => 1,
      VertexFormat::u16_normalized | VertexFormat::u16 => 2,
      VertexFormat::u32_normalized | VertexFormat::u32 => 4
    };
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VertexAttribute {
  pub name: Name,
  pub offset: usize,
  pub ty: VertexFormat, pub elements: usize,
  pub buffer_index: usize
}

#[derive(Debug, PartialEq, Clone)]
pub struct VertexBuffer {
  pub stride: usize,
  pub buffer: Vec<u8>
}

#[derive(Debug, PartialEq, Clone)]
pub enum IndexFormat {
  u8, u16, u32, u64
}

impl IndexFormat {
  pub fn byte_size(&self) -> usize {
    return match *self {
      IndexFormat::u8 => 1,
      IndexFormat::u16 => 2,
      IndexFormat::u32 => 4,
      IndexFormat::u64 => 8
    };
  }
}


#[derive(Debug, PartialEq, Clone)]
pub enum GeometryType {
  Points,
  Lines,
  Triangles,
  TriangleStrips
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexBuffer {
  pub count: usize,
  pub ty: IndexFormat,
  pub geometry: GeometryType,
  pub buffer: Vec<u8>
  
}

#[derive(Debug)]
pub struct Mesh {
  pub vertex_count: usize,
  pub attributes: Vec<VertexAttribute>,
  pub buffers: Vec<VertexBuffer>,
  pub submeshes: Vec<IndexBuffer>
}

impl Mesh {
  pub fn attribute_for(&self, name: Name) -> Option<VertexAttribute> {
    for attribute in &self.attributes {
      if attribute.name == name {
        return Some(attribute.clone());
      }
    }

    return None;
  }
}
