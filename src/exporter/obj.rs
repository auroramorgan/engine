use mesh;
use asset;
use model;

use vertex;

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
  if let Some(view) = mesh.untyped_view_for(&name) {
    for i in 0 .. view.len() {
      let value: Vec<String> = view.get_f32(i).iter().take(max_elements).map(|x| format!("{}", x)).collect();

      result.push_str(format!("{} {}\n", prefix, value.join(" ")).as_str());
    }

    return true;
  }

  return false;
}

fn write_faces(textures_written: bool, normals_written: bool, submesh: &mesh::Submesh, result: &mut String) {
  result.push_str(format!("o {}\n", submesh.name).as_str());

  for face in submesh.faces() {
    result.push_str("  f");

    for vertex in face {
      write_face_part(textures_written, normals_written, vertex + 1, result);
    }

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
