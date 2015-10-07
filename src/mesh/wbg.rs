use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use mesh;
use mesh::{VertexFormat, VertexAttribute, VertexBuffer, IndexFormat, IndexBuffer, GeometryType, Name};

pub fn load(input: Vec<u8>) -> Option<mesh::Mesh> {
  let mut cursor = Cursor::new(input);

  assert_eq!(cursor.read_u8().unwrap(), 0); // File version
  assert_eq!(cursor.read_u8().unwrap(), 1); // Mesh count

  return read_mesh(&mut cursor);
}

fn read_mesh(cursor: &mut Read) -> Option<mesh::Mesh> {
  let name_length = cursor.read_u8().unwrap();
  let mut name = String::new();
  cursor.take(name_length as u64).read_to_string(&mut name).unwrap();

  let (vertex_count, attributes, vertex_buffer) = read_vertex_buffer(cursor);
  let index_buffer = read_index_buffer(cursor);

  return Some(mesh::Mesh {
    vertex_count: vertex_count,
    attributes: attributes,
    buffers: vec![vertex_buffer],
    submeshes: vec![index_buffer]
  });
}

fn read_vertex_buffer(cursor: &mut Read) -> (usize, Vec<VertexAttribute>, VertexBuffer) {
  let decl_length = cursor.read_u8().unwrap();

  let mut vertex_size = 0usize;

  let mut vertex_attributes = Vec::new();

  for _ in 0 .. decl_length {
    let usage = cursor.read_u8().unwrap();
    let _ = cursor.read_u8().unwrap(); // TODO: Is this useful?

    let file_type = cursor.read_u8().unwrap();
    let elements = (file_type >> 5) + 1;
    let offset = vertex_size;

    let ty = match file_type & 0x0F {
      0 => VertexFormat::i8,
      1 => VertexFormat::i16,
      2 => VertexFormat::i32,
      3 => VertexFormat::f16,
      4 => VertexFormat::f32,
      8 => VertexFormat::u8,
      9 => VertexFormat::u16,
      10 => VertexFormat::u32,
      16 => VertexFormat::i8_normalized,
      17 => VertexFormat::i16_normalized,
      24 => VertexFormat::u8_normalized,
      25 => VertexFormat::u16_normalized,
      
      _ => panic!("Unknown ty in .wbg")
    };

    vertex_size += ty.byte_size() * (elements as usize);

    let name = match usage {
      0 => Name::Position,
      1 => Name::Color,
      2 => Name::Normal,
      3 => Name::Tangent,
      4 => Name::Binormal,
      5 => Name::TextureCoordinate,
      6 => Name::JointWeights,
      7 => Name::JointIndices,
      _ => panic!("Unknown name in .wbg")
    };

    let vertex_attribute = VertexAttribute {
      name: name,
      offset: offset as usize,
      buffer_index: 0,
      ty: ty, elements: elements as usize
    };

    vertex_attributes.push(vertex_attribute);
  }

  let mut buffer = Cursor::new(Vec::new());
  let vertex_count = cursor.read_u32::<LittleEndian>().unwrap() as usize;

  for _ in 0 .. vertex_count {
    for decl in &vertex_attributes {
      for _ in 0 .. decl.elements {
        match decl.ty {
          VertexFormat::i16 | VertexFormat::f16 => {
            let value = cursor.read_i16::<LittleEndian>().unwrap();
            buffer.write_i16::<LittleEndian>(value).unwrap();
          }
          _ => panic!("Unknown ty in .wbg")
        };
      }
    }
  }

  let vertex_buffer = VertexBuffer {
    stride: vertex_size,
    buffer: buffer.into_inner()
  };

  return (vertex_count, vertex_attributes, vertex_buffer);
}

fn read_index_buffer(cursor: &mut Read) -> IndexBuffer {
  let ty = match cursor.read_u8().unwrap() {
    0 => IndexFormat::u16,
    1 => IndexFormat::u32,
    _ => panic!("Unknown ty in .wbg")
  };

  let index_count = cursor.read_u32::<LittleEndian>().unwrap();
  let mut index_buffer = Cursor::new(Vec::new());

  for _ in 0 .. index_count {
    match ty {
      IndexFormat::u16 => {
        let value = cursor.read_u16::<LittleEndian>().unwrap();
        index_buffer.write_u16::<LittleEndian>(value).unwrap();
      }
      IndexFormat::u32 => {
        let value = cursor.read_u32::<LittleEndian>().unwrap();
        index_buffer.write_u32::<LittleEndian>(value).unwrap();
      }
      _ => panic!("Unknown ty in .wbg")
    }
  }

  return IndexBuffer {
    count: index_count as usize,
    ty: ty,
    geometry: GeometryType::Triangles,
    buffer: index_buffer.into_inner()
  }
}