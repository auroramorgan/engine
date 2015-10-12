#[derive(Debug, PartialEq, Clone)]
pub struct Skeleton {
  pub bones: Vec<Bone>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bone {
  pub name: String,
  pub parent_index: Option<usize>,

  pub position: [f32; 3],
  pub orientation: [f32; 4],
  pub scale_shear: [f32; 9]
}