use std::io::Read;

use byteorder::ReadBytesExt;

pub fn read_mesh_bindings(cursor: &mut Read) -> Vec<usize> {
  let mesh_bindings_count = cursor.read_u8().unwrap();

  return (0 .. mesh_bindings_count).map(|_| { cursor.read_u8().unwrap() as usize }).collect();
}
