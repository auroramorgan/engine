use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use mesh;
use index;

use importer::wbg::{read_string, read_vertex_buffer, read_index_buffer};

pub fn read_blend_shapes(cursor: &mut Read) -> Vec<mesh::Mesh> {
  let blend_shape_count = cursor.read_u16::<LittleEndian>().unwrap();

  return (0 .. blend_shape_count).map(|_| { read_blend_shape(cursor) }).collect();
}

fn read_blend_shape(cursor: &mut Read) -> mesh::Mesh {
  let name = read_string(cursor);

  let (vertex_count, vertex_buffer, descriptor) = read_vertex_buffer(cursor);
  let (index_count, index_buffer, index_format) = read_index_buffer(cursor);

  println!("  Descriptor: {:?}", descriptor);

  return mesh::Mesh {
    name: name.clone(),
    vertex_count: vertex_count,
    descriptor: descriptor,
    buffers: vec![vertex_buffer],
    submeshes: vec![mesh::Submesh {
      name: name,
      buffer: index_buffer,
      index_count: index_count,
      index_format: index_format,
      geometry: index::Geometry::Points
    }]
  };
}
