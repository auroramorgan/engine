use model;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
  Model(model::Model)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Asset {
  pub objects: Vec<Object>
}
