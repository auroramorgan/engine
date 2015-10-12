use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use animation::skeleton;

use importer::wbg::read_string;

pub fn read_skeleton(cursor: &mut Read) -> skeleton::Skeleton {
  let bone_count = cursor.read_u8().unwrap();

  let bones = (0 .. bone_count).map(|_| { read_bone(cursor) }).collect();

  return skeleton::Skeleton {
    bones: bones
  };
}

pub fn read_bone(cursor: &mut Read) -> skeleton::Bone {
  let name = read_string(cursor);

  let flags = cursor.read_u8().unwrap();

  let parent_index = match cursor.read_u8().unwrap() {
    0xFF => None,
    i => Some(i as usize)
  };

  let position = if flags & 0x01 == 0x01 {
    [
      cursor.read_f32::<LittleEndian>().unwrap(),
      cursor.read_f32::<LittleEndian>().unwrap(),
      cursor.read_f32::<LittleEndian>().unwrap()
    ]
  } else {
    [0.0, 0.0, 0.0]
  };

  let orientation = if flags & 0x02 == 0x02 {
    [
      cursor.read_f32::<LittleEndian>().unwrap(),
      cursor.read_f32::<LittleEndian>().unwrap(),
      cursor.read_f32::<LittleEndian>().unwrap(),
      cursor.read_f32::<LittleEndian>().unwrap()
    ]
  } else {
    [0.0, 0.0, 0.0, 1.0]
  };

  let scale_shear = if flags & 0x04 == 0x04 {
    [
      cursor.read_f32::<LittleEndian>().unwrap(),
      cursor.read_f32::<LittleEndian>().unwrap(),
      cursor.read_f32::<LittleEndian>().unwrap(),
      cursor.read_f32::<LittleEndian>().unwrap(),
      cursor.read_f32::<LittleEndian>().unwrap(),
      cursor.read_f32::<LittleEndian>().unwrap(),
      cursor.read_f32::<LittleEndian>().unwrap(),
      cursor.read_f32::<LittleEndian>().unwrap(),
      cursor.read_f32::<LittleEndian>().unwrap()
    ]
  } else {
    [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
  };

  return skeleton::Bone {
    name: name,
    parent_index: parent_index,

    position: position,
    orientation: orientation,
    scale_shear: scale_shear
  };
}
