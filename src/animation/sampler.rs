#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Behavior {
  Undefined, Constant, Gradient, Cycle, CycleRelative, Oscillate
}

#[derive(Debug, PartialEq, Clone)]
pub struct Sampler {
  pub pre_behavior: Behavior,
  pub post_behavior: Behavior,
  pub degree: usize,
  pub input: Vec<f32>,
  pub outputs: Vec<Vec<f32>>
}