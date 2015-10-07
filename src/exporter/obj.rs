use std::io::{Cursor, Seek, SeekFrom};
use std::cmp;

use byteorder::{LittleEndian, ReadBytesExt};

use mesh;

extern {
  /// The `llvm.convert.from.fp16.f64` intrinsic.
  #[link_name = "llvm.convert.from.fp16.f64"]
  pub fn convert_from_fp16_f64(a: i16) -> f64;
}

pub fn export(mesh: &mesh::Mesh) -> String {
  let mut result = String::new();

  if !write("v", mesh::Name::Position, 4, mesh, &mut result) {
    panic!("No positions when exporting to .obj");
  }

  let textures_written = write("vt", mesh::Name::TextureCoordinate, 3, mesh, &mut result);
  let normals_written = write("vn", mesh::Name::Normal, 3, mesh, &mut result);

  write_faces(textures_written, normals_written, mesh, &mut result);

  return result;
}

fn write(prefix: &str, name: mesh::Name, max_elements: usize, mesh: &mesh::Mesh, result: &mut String) -> bool {
  let attribute = match mesh.attribute_for(name) {
    Some(s) => s, None => return false
  };

  let vertex_buffer = &mesh.buffers[attribute.buffer_index];

  let mut cursor = Cursor::new(vertex_buffer.buffer.as_slice());

  cursor.seek(SeekFrom::Current(attribute.offset as i64)).unwrap();

  let elements = cmp::min(attribute.elements, max_elements);

  let seek_distance = (vertex_buffer.stride - elements * attribute.ty.byte_size()) as i64;

  for _ in 0 .. mesh.vertex_count {
    result.push_str(prefix);
    for _ in 0 .. elements {
      let value = match attribute.ty {
        mesh::VertexFormat::f16 => unsafe { convert_from_fp16_f64(cursor.read_i16::<LittleEndian>().unwrap()) },
        mesh::VertexFormat::i16 => cursor.read_i16::<LittleEndian>().unwrap() as f64,
        _ => panic!("Unknown ty when exporting .obj")
      };

      result.push_str(format!(" {}", value).as_str());
    }
    result.push_str("\n");

    cursor.seek(SeekFrom::Current(seek_distance)).unwrap();
  }

  return true;
}

fn write_faces(textures_written: bool, normals_written: bool, mesh: &mesh::Mesh, result: &mut String) {
  let index_buffer = &mesh.submeshes[0];

  let mut cursor = Cursor::new(index_buffer.buffer.as_slice());

  let mut indices = Vec::new();

  for _ in 0 .. index_buffer.count {
    let value = match index_buffer.ty {
      mesh::IndexFormat::u16 => cursor.read_u16::<LittleEndian>().unwrap() as usize,
      mesh::IndexFormat::u32 => cursor.read_u32::<LittleEndian>().unwrap() as usize,
      _ => panic!("Unknown ty when exporting .obj")
    } + 1;

    indices.push(value);
  }

  let mut triangles = Vec::new();
  
  for i in 0 .. index_buffer.count / 3 {
    triangles.push([indices[3 * i + 0], indices[3 * i + 1], indices[3 * i + 2]])
  }

  for triangle in triangles {
    result.push_str("f");

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
