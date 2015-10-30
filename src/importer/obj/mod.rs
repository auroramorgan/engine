use std::str;
use std::str::FromStr;
use std::sync::Arc;

use asset;
use buffer::{Buffer, BufferView};

struct ObjVertex {
  v: [f32; 4],
  vn: [f32; 3],
  vt: [f32; 3]
}

pub fn import(input: Arc<Buffer>) -> Option<asset::Asset> {
  let source = str::from_utf8(&input[..]).unwrap();

  let mut v: Vec<Vec<f32>> = Vec::new();
  let mut vn: Vec<Vec<f32>> = Vec::new();
  let mut vt: Vec<Vec<f32>> = Vec::new();
  let mut f: Vec<Vec<Vec<Option<u32>>>> = Vec::new();

  for line in source.lines() {
    let mut line = line.split_whitespace();

    let prefix = line.next();

    match prefix {
      Some("f") => {
        let mut result = Vec::new();
        for vertex in line {
          result.push(vertex.split('/').map(|x| FromStr::from_str(x).ok()).collect());
        }
        f.push(result);
      }
      Some("v") => v.push(line.filter_map(|x| FromStr::from_str(x).ok()).collect()),
      Some("vn") => vn.push(line.filter_map(|x| FromStr::from_str(x).ok()).collect()),
      Some("vt") => vt.push(line.filter_map(|x| FromStr::from_str(x).ok()).collect()),
      Some(_) | None => ()
    }
  }

  let mut vertices = Vec::new();
  for face in f {
    if face.len() != 3 {
      panic!("Can only handle triangles right now");
    }

    for i in 0 .. 3 {
      let face_vertex = face[i];
      let v = face_vertex.get(0).map(|x| v[x.unwrap() as usize - 1]).unwrap();
      // let vn = face_vertex.get(1).map(|x| x.map(|x| v[x - 1])).unwrap_or([0.0, 0.0, 0.0]);
      // let vt = face_vertex.get(2).map(|x| x.map(|x| v[x - 1])).unwrap_or([0.0, 0.0, 0.0]);

      vertices.push(ObjVertex { v: v, vn: [0.0, 0.0, 0.0], vt: [0.0, 0.0, 0.0] });
    }
  }

  println!("{:?}", vertices);

  return None;
}

#[cfg(test)]
mod tests {
  use super::*;

  use std::fs;
  use std::io::Read;

  use asset;
  use buffer;

  #[test]
  fn import_cube() {
    let mut file = fs::File::open("fixtures/cube.obj").unwrap();
    let mut result = Vec::new();
    let _ = file.read_to_end(&mut result).unwrap();

    let result = import(buffer::Buffer::new(None, None, result)).unwrap();

    assert_eq!(result.objects.len(), 1);

    let asset::Object::Model(ref model) = result.objects[0];

    assert_eq!(model.name, "cube");
    assert_eq!(model.blend_shapes.len(), 0);
    assert_eq!(model.skeleton, None);

    let mesh = &model.mesh;

    assert_eq!(mesh.name, "cube");
    assert_eq!(mesh.vertex_count, 36);
    // descriptor
    // buffers
    // submeshes
  }
}