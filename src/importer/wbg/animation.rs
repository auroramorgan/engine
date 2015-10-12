use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use animation::sampler;

use importer::wbg::read_string;

pub fn read_animation(cursor: &mut Read) {
  let name = read_string(cursor);
  let duration = cursor.read_f32::<LittleEndian>().unwrap();

  println!("Animation Name: {:?} ({:?})", name, duration);

  let group_count = cursor.read_u8().unwrap();

  (0 .. group_count).map(|_| { read_group(cursor) }).last();
}

fn read_group(cursor: &mut Read) -> Vec<(String, (Option<sampler::Sampler>, Option<sampler::Sampler>, Option<sampler::Sampler>))> {
  let name = read_string(cursor);

  let transform_track = cursor.read_u8().unwrap();

  return (0 .. transform_track).map(|_| { (name.clone(), read_transform_track(cursor)) }).collect();
}

fn read_transform_track(cursor: &mut Read) -> (Option<sampler::Sampler>, Option<sampler::Sampler>, Option<sampler::Sampler>) {
  let name = read_string(cursor);

  let mut orientation = read_curves(cursor);
  let position = read_curves(cursor);
  let scale_shear = read_curves(cursor);

  if let Some(ref mut o) = orientation {
    let indices_to_flip = indices_to_flip(o);

    for output in &mut o.outputs {
      for i in &indices_to_flip {
        output[*i] = -output[*i];
      }
    }
  }

  println!("  Transform track: {:?}", name);

  return (orientation, position, scale_shear);
}

fn read_curves(cursor: &mut Read) -> Option<sampler::Sampler> {
  // TODO: Figure out what this type value does, it is ignored in CCP WebGL.
  if cursor.read_u8().unwrap() == 0 {
    return None;
  };

  let dimension = cursor.read_u8().unwrap() as usize;

  let degree = cursor.read_u8().unwrap() as usize;

  let knot_count = cursor.read_u32::<LittleEndian>().unwrap() as usize;

  let knots: Vec<f32> = (0 .. knot_count).map(|_| {
    cursor.read_f32::<LittleEndian>().unwrap()
  }).collect();

  let control_count = cursor.read_u32::<LittleEndian>().unwrap() as usize;

  assert_eq!(control_count, dimension * knot_count);

  let controls: Vec<f32> = (0 .. control_count).map(|_| {
    cursor.read_f32::<LittleEndian>().unwrap()
  }).collect();

  let outputs = (0 .. dimension).map(|i| {
    (i .. control_count).step_by(dimension).map(|j| { controls[j] }).collect()
  }).collect();

  return Some(sampler::Sampler {
    pre_behavior: sampler::Behavior::Undefined,
    post_behavior: sampler::Behavior::Undefined,
    degree: degree,
    input: knots,
    outputs: outputs
  });
}

fn indices_to_flip(orientation: &sampler::Sampler) -> Vec<usize> {
  let mut indices = Vec::new();
  let mut last: Vec<f32> = orientation.outputs.iter().map(|_| 0.0f32).collect();

  for i in 0 .. orientation.outputs[0].len() {
    let current: Vec<f32> = orientation.outputs.iter().map(|x| x[i]).collect();

    let sum: f32 = last.iter().zip(current.iter()).map(|(x, y)| { x * y }).sum();

    if sum < 0.0f32 { indices.push(i) }

    last = current;
  }

  return indices;
}
