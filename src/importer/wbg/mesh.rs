use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use mesh;
use index;

use importer::wbg::{read_string, read_vertex_buffer, read_index_buffer};

pub fn read_mesh(cursor: &mut Read) -> mesh::Mesh {
  let name = read_string(cursor);

  let (vertex_count, vertex_buffer, descriptor) = read_vertex_buffer(cursor);
  let (_, index_buffer, index_format) = read_index_buffer(cursor);

  let area_count = cursor.read_u8().unwrap();

  let submeshes = (0 .. area_count).map(|_| {
    read_mesh_area(cursor, &index_buffer, index_format)
  }).collect();

  return mesh::Mesh {
    name: name,
    vertex_count: vertex_count,
    descriptor: descriptor,
    buffers: vec![vertex_buffer],
    submeshes: submeshes
  };
}

fn read_mesh_area(cursor: &mut Read, index_buffer: &Vec<u8>, index_format: index::Format) -> mesh::Submesh {
  let name = read_string(cursor);

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
  };
}
