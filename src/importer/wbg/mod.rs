use std::io::{Cursor, Read, Seek, SeekFrom};
use std::sync::Arc;

use byteorder::{LittleEndian, ReadBytesExt};

use asset;
use buffer::{Buffer, BufferView};
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

#[cfg(target_endian = "big")]
pub fn import(input: Vec<u8>) -> Option<asset::Asset> {
  return None; // TODO: Support Big Endian too in the long run maybe?
}

#[cfg(target_endian = "little")]
pub fn import(input: Arc<Buffer>) -> Option<asset::Asset> {
  let mut cursor = Cursor::new(&input[..]);

  let _ = cursor.read_u8().unwrap(); // TODO: File version, seen 0 and 60…

  let mesh_count = cursor.read_u8().unwrap();

  let meshes: Vec<(Mesh, Vec<Mesh>)> = (0 .. mesh_count).map(|_| {
    let mesh = mesh::read_mesh(&mut cursor, input.clone());
    let _ = bone_bindings::read_bone_bindings(&mut cursor);
    let blend_shapes = blend_shape::read_blend_shapes(&mut cursor, input.clone());

    (mesh, blend_shapes)
  }).collect();

  let model_count = cursor.read_u8().unwrap();

  let models: Vec<Vec<asset::Object>> = (0 .. model_count).map(|_| {
    let name = read_string(&mut cursor);

    let skeleton = skeleton::read_skeleton(&mut cursor);
    let mesh_bindings = mesh_bindings::read_mesh_bindings(&mut cursor);

    mesh_bindings.iter().map(|i| {
      let (ref mesh, ref blend_shapes) = meshes[mesh_bindings[*i]];

      asset::Object::Model(model::Model {
        name: name.clone(),
        mesh: mesh.clone(),
        blend_shapes: blend_shapes.clone(),
        skeleton: skeleton.clone()
      })
    }).collect()
  }).collect();

  let animation_count = cursor.read_u8().unwrap();

  (0 .. animation_count).map(|_| {
    animation::read_animation(&mut cursor)
  }).last();

  let objects = models.iter().fold(vec![], |mut sum, x| { sum.push_all(&*x); sum });

  return Some(asset::Asset { buffers: vec![input.clone()], objects: objects });
}

fn read_string(cursor: &mut Read) -> String {
  let mut string = String::new();

  let length = cursor.read_u8().unwrap();

  cursor.take(length as u64).read_to_string(&mut string).unwrap();

  return string;
}

fn read_vertex_buffer(cursor: &mut Cursor<&[u8]>, buffer: Arc<Buffer>) -> (usize, Arc<BufferView>, vertex::Descriptor) {
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

  let vertex_count = cursor.read_u32::<LittleEndian>().unwrap() as usize;

  let offset = cursor.seek(SeekFrom::Current(0)).unwrap();
  let length = (vertex_count * vertex_size) as u64;
  let _ = cursor.seek(SeekFrom::Start(offset + length)).unwrap();

  let view = BufferView::new(None, buffer, offset as usize, length as usize);

  let descriptor = vertex::Descriptor {
    attributes: vertex_attributes,
    layouts: vec![vertex::BufferLayout { stride: vertex_size }]
  };

  return (vertex_count, view, descriptor);
}

fn read_index_buffer(cursor: &mut Cursor<&[u8]>, buffer: Arc<Buffer>) -> (usize, Arc<BufferView>, index::Format) {
  let format = match cursor.read_u8().unwrap() {
    0 => index::Format::u16,
    1 => index::Format::u32,
    n => panic!("Unknown format in .wbg ({:?})", n)
  };

  let index_count = cursor.read_u32::<LittleEndian>().unwrap() as usize;

  let offset = cursor.seek(SeekFrom::Current(0)).unwrap();
  let length = (index_count * format.byte_size()) as u64;
  let _ = cursor.seek(SeekFrom::Start(offset + length)).unwrap();

  let view = BufferView::new(None, buffer, offset as usize, length as usize);

  return (index_count, view, format);
}
