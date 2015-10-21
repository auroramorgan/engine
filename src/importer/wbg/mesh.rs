use std::io::Cursor;
use std::sync::Arc;

use byteorder::{LittleEndian, ReadBytesExt};

use buffer;
use mesh;
use index;

use importer::wbg::{read_string, read_vertex_buffer, read_index_buffer};

pub fn read_mesh(cursor: &mut Cursor<&[u8]>, buffer: Arc<buffer::Buffer>) -> mesh::Mesh {
  let name = read_string(cursor);

  let (vertex_count, vertex_buffer, descriptor) = read_vertex_buffer(cursor, buffer.clone());
  let (_, index_buffer, index_format) = read_index_buffer(cursor, buffer);

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

fn read_mesh_area(cursor: &mut Cursor<&[u8]>, index_buffer: &Arc<buffer::BufferView>, index_format: index::Format) -> mesh::Submesh {
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
  let offset = index_buffer.offset + index_format.byte_size() * start;
  let length = index_format.byte_size() * index_count;

  assert!(offset >= index_buffer.offset);
  assert!(offset + length <= index_buffer.offset + index_buffer.length);

  let buffer = index_buffer.buffer.clone();
  let view = buffer::BufferView::new(index_buffer.name.clone(), buffer, offset, length);

  return mesh::Submesh {
    name: name,
    view: view,
    index_count: index_count,
    index_format: index_format,
    geometry: index::Geometry::Triangles
  };
}
