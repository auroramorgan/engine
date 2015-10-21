use std::sync::Arc;

use buffer;
use model;

#[derive(Debug, Clone)]
pub enum Object {
  Model(model::Model)
}

#[derive(Debug, Clone)]
pub struct Asset {
  pub buffers: Vec<Arc<buffer::Buffer>>,
  pub objects: Vec<Object>
}
