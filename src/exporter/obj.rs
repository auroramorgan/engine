use std::io::{Cursor, Seek, SeekFrom};
use std::cmp;

use byteorder::{LittleEndian, ReadBytesExt};

use mesh;
use asset;
use model;

use index;
use vertex;

extern {
  /// The `llvm.convert.from.fp16.f64` intrinsic.
  #[link_name = "llvm.convert.from.fp16.f64"]
  pub fn convert_from_fp16_f64(a: i16) -> f64;
}

pub fn export(asset: &asset::Asset) -> Vec<(String, String)> {
  let mut files = Vec::new();

  for object in &asset.objects {
    match object {
      &asset::Object::Model(ref m) => files.push_all(export_model(m).as_slice())
    }
  }

  return files;
}

pub fn export_model(model: &model::Model) -> Vec<(String, String)> {
  let mut result = String::new();

  let mesh = &model.mesh;

  if !write("v", vertex::AttributeName::Position, 4, mesh, &mut result) {
    panic!("No positions when exporting to .obj");
  }

  let textures_written = write("vt", vertex::AttributeName::TextureCoordinate, 3, mesh, &mut result);
  let normals_written = write("vn", vertex::AttributeName::Normal, 3, mesh, &mut result);

  for submesh in &mesh.submeshes {
    write_faces(textures_written, normals_written, &submesh, &mut result);
  }

  return vec![(mesh.name.clone() + ".obj", result)];
}

fn write(prefix: &str, name: vertex::AttributeName, max_elements: usize, mesh: &mesh::Mesh, result: &mut String) -> bool {
  let attribute = match mesh.attribute_for(&name) {
    Some(s) => s, None => return false
  };

  let vertex_buffer = &mesh.buffers[attribute.buffer_index];
  let mut cursor = Cursor::new(vertex_buffer.as_slice());

  cursor.seek(SeekFrom::Current(attribute.offset as i64)).unwrap();

  let elements = cmp::min(attribute.format.elements(), max_elements);

  println!("Elements {:?} for {:?}", elements, name);

  let stride = &mesh.descriptor.layouts[attribute.buffer_index].stride;

  for i in 0 .. mesh.vertex_count {
    cursor.seek(SeekFrom::Start((stride * i + attribute.offset) as u64)).unwrap();

    result.push_str(prefix);
    for _ in 0 .. elements {
      let value = match attribute.format.scalar() {
        vertex::Scalar::f16 => unsafe { convert_from_fp16_f64(cursor.read_i16::<LittleEndian>().unwrap()) },
        vertex::Scalar::i16 => cursor.read_i16::<LittleEndian>().unwrap() as f64,
        _ => panic!("Unknown ty when exporting .obj")
      };

      result.push_str(format!(" {}", value).as_str());
    }
    result.push_str("\n");
  }

  return true;
}

fn write_faces(textures_written: bool, normals_written: bool, submesh: &mesh::Submesh, result: &mut String) {
  let mut cursor = Cursor::new(submesh.buffer.as_slice());

  let mut indices = Vec::new();

  for _ in 0 .. submesh.index_count {
    let value = match submesh.index_format {
      index::Format::u16 => cursor.read_u16::<LittleEndian>().unwrap() as usize,
      index::Format::u32 => cursor.read_u32::<LittleEndian>().unwrap() as usize,
      _ => panic!("Unknown ty when exporting .obj")
    } + 1;

    indices.push(value);
  }

  let mut triangles = Vec::new();

  for i in 0 .. submesh.index_count / 3 {
    triangles.push([indices[3 * i + 0], indices[3 * i + 1], indices[3 * i + 2]])
  }

  result.push_str(format!("o {}\n", submesh.name).as_str());
  for triangle in triangles {
    result.push_str("  f");

    write_face_part(textures_written, normals_written, triangle[0], result);
    write_face_part(textures_written, normals_written, triangle[1], result);
    write_face_part(textures_written, normals_written, triangle[2], result);

    result.push_str("\n")
  }
}

fn write_face_part(textures_written: bool, normals_written: bool, value: usize, result: &mut String) {
  result.push_str(format!(" {}", value).as_str());

  if textures_written {
    result.push_str(format!("/{}", value).as_str());
  }

  if normals_written {
    if !textures_written { result.push_str("/") };

    result.push_str(format!("/{}", value).as_str());
  }
}
