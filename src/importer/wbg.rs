use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use mesh;
use index;
use vertex;

pub fn import(input: Vec<u8>) -> Option<mesh::Mesh> {
  let mut cursor = Cursor::new(input);

  assert_eq!(cursor.read_u8().unwrap(), 0); // File version
  assert_eq!(cursor.read_u8().unwrap(), 1); // Mesh count

  return read_mesh(&mut cursor);
}

fn read_mesh(cursor: &mut Read) -> Option<mesh::Mesh> {
  let name_length = cursor.read_u8().unwrap();
  let mut name = String::new();
  cursor.take(name_length as u64).read_to_string(&mut name).unwrap();

  let (vertex_count, descriptor, vertex_buffer) = read_vertex_buffer(cursor);
  let index_buffer = read_index_buffer(cursor);

  return Some(mesh::Mesh {
    name: String::new(),
    vertex_count: vertex_count,
    descriptor: descriptor,
    buffers: vec![vertex_buffer],
    submeshes: vec![index_buffer]
  });
}

fn read_vertex_buffer(cursor: &mut Read) -> (usize, vertex::Descriptor, Vec<u8>) {
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
      0 => vertex::Format::i8,
      1 => vertex::Format::i16,
      2 => vertex::Format::i32,
      3 => vertex::Format::f16,
      4 => vertex::Format::f32,
      8 => vertex::Format::u8,
      9 => vertex::Format::u16,
      10 => vertex::Format::u32,
      16 => vertex::Format::i8_normalized,
      17 => vertex::Format::i16_normalized,
      24 => vertex::Format::u8_normalized,
      25 => vertex::Format::u16_normalized,
      
      _ => panic!("Unknown ty in .wbg")
    };

    vertex_size += ty.byte_size() * (elements as usize);

    let name = match usage {
      0 => vertex::AttributeName::Position,
      1 => vertex::AttributeName::Color,
      2 => vertex::AttributeName::Normal,
      3 => vertex::AttributeName::Tangent,
      4 => vertex::AttributeName::Binormal,
      5 => vertex::AttributeName::TextureCoordinate,
      6 => vertex::AttributeName::JointWeights,
      7 => vertex::AttributeName::JointIndices,
      _ => panic!("Unknown name in .wbg")
    };

    let vertex_attribute = vertex::Attribute {
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
          vertex::Format::i16 | vertex::Format::f16 => {
            let value = cursor.read_i16::<LittleEndian>().unwrap();
            buffer.write_i16::<LittleEndian>(value).unwrap();
          }
          _ => panic!("Unknown ty in .wbg")
        };
      }
    }
  }

  let vertex_descriptor = vertex::Descriptor {
    attributes: vertex_attributes,
    layouts: vec![vertex::BufferLayout { stride: vertex_size }]
  };

  return (vertex_count, vertex_descriptor, buffer.into_inner());
}

fn read_index_buffer(cursor: &mut Read) -> mesh::Submesh {
  let ty = match cursor.read_u8().unwrap() {
    0 => index::Format::u16,
    1 => index::Format::u32,
    _ => panic!("Unknown ty in .wbg")
  };

  let index_count = cursor.read_u32::<LittleEndian>().unwrap();
  let mut index_buffer = Cursor::new(Vec::new());

  for _ in 0 .. index_count {
    match ty {
      index::Format::u16 => {
        let value = cursor.read_u16::<LittleEndian>().unwrap();
        index_buffer.write_u16::<LittleEndian>(value).unwrap();
      }
      index::Format::u32 => {
        let value = cursor.read_u32::<LittleEndian>().unwrap();
        index_buffer.write_u32::<LittleEndian>(value).unwrap();
      }
      _ => panic!("Unknown ty in .wbg")
    }
  }

  return mesh::Submesh {
    name: String::new(),
    index_count: index_count as usize,
    index_format: ty,
    geometry: index::Geometry::Triangles,
    buffer: index_buffer.into_inner()
  }
}