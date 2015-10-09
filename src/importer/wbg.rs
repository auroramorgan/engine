use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use mesh;
use asset;

use index;
use vertex;

pub fn import(input: Vec<u8>) -> Option<asset::Asset> {
  let mut cursor = Cursor::new(input);

  assert_eq!(cursor.read_u8().unwrap(), 0); // File version

  let mesh_count = cursor.read_u8().unwrap();

  let mut objects = Vec::new();
  for _ in 0 .. mesh_count {
    let meshes = read_mesh(&mut cursor);

    for mesh in meshes {
      objects.push(asset::Object::Mesh(mesh));
    }
  }

  return Some(asset::Asset { objects: objects });
}

fn read_mesh(cursor: &mut Read) -> Vec<mesh::Mesh> {
  let name_length = cursor.read_u8().unwrap();
  let mut name = String::new();
  cursor.take(name_length as u64).read_to_string(&mut name).unwrap();

  let (vertex_count, vertex_buffer, descriptor) = read_vertex_buffer(cursor);
  let (_, index_buffer, index_format) = read_index_buffer(cursor);

  let mut submeshes = Vec::new();
  let area_count = cursor.read_u8().unwrap();
  for _ in 0 .. area_count {
    submeshes.push(read_mesh_area(cursor, &index_buffer, index_format));
  }

  let mut result = vec![mesh::Mesh {
    name: name,
    vertex_count: vertex_count,
    descriptor: descriptor,
    buffers: vec![vertex_buffer],
    submeshes: submeshes
  }];

  let bone_binding_count = cursor.read_u8().unwrap();
  println!("Bone binding count: {}", bone_binding_count);

  for _ in 0 .. bone_binding_count {
    read_bone_binding(cursor);
  }

  let blend_shape_count = cursor.read_u8().unwrap();
  println!("Blend shape count: {}", blend_shape_count);

  for _ in 0 .. blend_shape_count {
    result.push(read_blend_shape(cursor));
  }

  return result
}

fn read_blend_shape(cursor: &mut Read) -> mesh::Mesh {
  let name_length = cursor.read_u8().unwrap();
  let mut name = String::new();
  cursor.take(name_length as u64).read_to_string(&mut name).unwrap();

  println!("Blend shape: {}", name);

  let (vertex_count, vertex_buffer, descriptor) = read_vertex_buffer(cursor);
  let (index_count, index_buffer, index_format) = read_index_buffer(cursor);

  return mesh::Mesh {
    name: name,
    vertex_count: vertex_count,
    descriptor: descriptor,
    buffers: vec![vertex_buffer],
    submeshes: vec![mesh::Submesh {
      name: String::new(),
      buffer: index_buffer,
      index_count: index_count,
      index_format: index_format,
      geometry: index::Geometry::Points
    }]
  };
}

fn read_bone_binding(cursor: &mut Read) {
  let name_length = cursor.read_u8().unwrap();
  let mut name = String::new();
  cursor.take(name_length as u64).read_to_string(&mut name).unwrap();

  println!("Bone binding: {}, ignoring", name);
}

fn read_mesh_area(cursor: &mut Read, index_buffer: &Vec<u8>, index_format: index::Format) -> mesh::Submesh {
  let name_length = cursor.read_u8().unwrap();

  let mut name = String::new();
  cursor.take(name_length as u64).read_to_string(&mut name).unwrap();

  let start = cursor.read_u32::<LittleEndian>().unwrap() as usize;
  let count = cursor.read_u32::<LittleEndian>().unwrap() as usize;

  let _ = [ // TODO: What should we do with min_bounds?
    cursor.read_f32::<LittleEndian>().unwrap(),
    cursor.read_f32::<LittleEndian>().unwrap(),
    cursor.read_f32::<LittleEndian>().unwrap()
  ];

  let _ = [ // TODO: What should we do with max_bounds?
    cursor.read_f32::<LittleEndian>().unwrap(),
    cursor.read_f32::<LittleEndian>().unwrap(),
    cursor.read_f32::<LittleEndian>().unwrap()
  ];

  let index_count = 3 * count;
  let start_byte = index_format.byte_size() * start;
  let end_byte = start_byte + index_format.byte_size() * index_count;

  let buffer = index_buffer[start_byte .. end_byte].to_owned();

  return mesh::Submesh {
    name: name,
    buffer: buffer,
    index_count: index_count,
    index_format: index_format,
    geometry: index::Geometry::Triangles
  }
}

fn read_vertex_buffer(cursor: &mut Read) -> (usize, Vec<u8>, vertex::Descriptor) {
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

  let descriptor = vertex::Descriptor {
    attributes: vertex_attributes,
    layouts: vec![vertex::BufferLayout { stride: vertex_size }]
  };

  return (vertex_count, buffer.into_inner(), descriptor);
}

fn read_index_buffer(cursor: &mut Read) -> (usize, Vec<u8>, index::Format) {
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

  return (index_count as usize, index_buffer.into_inner(), ty);
}