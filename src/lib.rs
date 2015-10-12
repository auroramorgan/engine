#![feature(convert)]
#![feature(step_by)]
#![feature(iter_arith)]
#![feature(vec_push_all)]
#![feature(link_llvm_intrinsics)]

extern crate xml;
extern crate hyper;
extern crate eventual;
extern crate byteorder;
extern crate rustc_serialize;

pub mod resource_manager;
pub mod resource_loaders;

pub mod index;
pub mod vertex;

pub mod mesh;
pub mod asset;
pub mod model;

pub mod animation {
  pub mod sampler;
  pub mod skeleton;
}

pub mod importer {
  pub mod wbg;
}

pub mod exporter {
  pub mod obj;
}
