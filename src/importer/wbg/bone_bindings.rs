use std::io::Read;

use byteorder::ReadBytesExt;

use importer::wbg::read_string;

pub fn read_bone_bindings(cursor: &mut Read) -> Vec<String> {
  let bone_binding_count = cursor.read_u8().unwrap();

  return (0 .. bone_binding_count).map(|_| { read_string(cursor) }).collect();
}
