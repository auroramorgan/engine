use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use asset;
use model;

use mesh::Mesh;

use index;
use vertex;
mod mesh;
mod blend_shape;
mod bone_bindings;

mod skeleton;
mod mesh_bindings;

mod animation;

pub fn import(input: Vec<u8>) -> Option<asset::Asset> {
  let mut cursor = Cursor::new(input);

  let _ = cursor.read_u8().unwrap(); // TODO: File version, seen 0 and 60…

  let mesh_count = cursor.read_u8().unwrap();

  let meshes: Vec<(Mesh, Vec<Mesh>)> = (0 .. mesh_count).map(|_| {
    let mesh = mesh::read_mesh(&mut cursor);
    let _ = bone_bindings::read_bone_bindings(&mut cursor);
    let blend_shapes = blend_shape::read_blend_shapes(&mut cursor);

    (mesh, blend_shapes)
  }).collect();

  let model_count = cursor.read_u8().unwrap();

  let models: Vec<model::Model> = (0 .. model_count).map(|_| {
    let name = read_string(&mut cursor);

    let skeleton = skeleton::read_skeleton(&mut cursor);
    let mesh_bindings = mesh_bindings::read_mesh_bindings(&mut cursor);

    assert_eq!(mesh_bindings.len(), 1);

    let (ref mesh, ref blend_shapes) = meshes[mesh_bindings[0]];

    model::Model {
      name: name,
      mesh: mesh.clone(),
      blend_shapes: blend_shapes.clone(),
      skeleton: skeleton
    }
  }).collect();

  let animation_count = cursor.read_u8().unwrap();

  (0 .. animation_count).map(|_| {
    animation::read_animation(&mut cursor)
  }).last();

  let mut objects = Vec::new();
  for model in models {
    objects.push(asset::Object::Model(model));
  }

  return Some(asset::Asset { objects: objects });
}

fn read_string(cursor: &mut Read) -> String {
  let mut string = String::new();

  let length = cursor.read_u8().unwrap();

  cursor.take(length as u64).read_to_string(&mut string).unwrap();

  return string;
}

fn read_vertex_buffer(cursor: &mut Read) -> (usize, Vec<u8>, vertex::Descriptor) {
  let decl_length = cursor.read_u8().unwrap();

  let mut vertex_size = 0usize;

  let mut vertex_attributes = Vec::new();

  for _ in 0 .. decl_length {
    let usage = cursor.read_u8().unwrap();
    let _ = cursor.read_u8().unwrap(); // TODO: Is this useful?

    let file_type = cursor.read_u8().unwrap();
    let width = vertex::Width::from_integer((file_type as usize >> 5) + 1).unwrap();
    let offset = vertex_size;

    let scalar = match file_type & 0x0F {
      0 => vertex::Scalar::i8,
      1 => vertex::Scalar::i16,
      2 => vertex::Scalar::i32,
      3 => vertex::Scalar::f16,
      4 => vertex::Scalar::f32,
      8 => vertex::Scalar::u8,
      9 => vertex::Scalar::u16,
      10 => vertex::Scalar::u32,
      16 => vertex::Scalar::i8_normalized,
      17 => vertex::Scalar::i16_normalized,
      24 => vertex::Scalar::u8_normalized,
      25 => vertex::Scalar::u16_normalized,
      ty => panic!("Unknown ty in .wbg ({:?})", ty)
    };

    let format = vertex::Format(scalar, width);

    vertex_size += format.byte_size();

    let name = match usage {
      0 => vertex::AttributeName::Position,
      1 => vertex::AttributeName::Color,
      2 => vertex::AttributeName::Normal,
      3 => vertex::AttributeName::Tangent,
      4 => vertex::AttributeName::Binormal,
      5 => vertex::AttributeName::TextureCoordinate,
      6 => vertex::AttributeName::JointWeights,
      7 => vertex::AttributeName::JointIndices,
      name => panic!("Unknown name in .wbg ({:?})", name)
    };

    let vertex_attribute = vertex::Attribute {
      name: name,
      format: format,
      offset: offset as usize,
      buffer_index: 0,
    };

    vertex_attributes.push(vertex_attribute);
  }

  let mut buffer = Cursor::new(Vec::new());
  let vertex_count = cursor.read_u32::<LittleEndian>().unwrap() as usize;

  for _ in 0 .. vertex_count {
    for attribute in &vertex_attributes {
      let format = attribute.format;

      for _ in 0 .. format.elements() {
        match format.scalar().byte_size() {
          1 => {
            let value = cursor.read_i8().unwrap();
            buffer.write_i8(value).unwrap();
          }
          2 => {
            let value = cursor.read_i16::<LittleEndian>().unwrap();
            buffer.write_i16::<LittleEndian>(value).unwrap();
          }
          4 => {
            let value = cursor.read_i16::<LittleEndian>().unwrap();
            buffer.write_i16::<LittleEndian>(value).unwrap();
          }
          size => {
            panic!("Unknown scalar size in .wbg {:?}, expected 1, 2, or 4", size);
          }
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
    n => panic!("Unknown format in .wbg ({:?})", n)
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
