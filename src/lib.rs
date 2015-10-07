#![feature(convert)]

extern crate hyper;
extern crate eventual;

pub mod resource_manager;
pub mod resource_loaders;

#[test]
fn main() {
  use resource_manager;
  use resource_loaders;

  use eventual::Async;

  let cdn = resource_loaders::cdn::Loader::new("http://developers.eveonline.com/ccpwgl/assetpath/967762", ".cache");

  let mut resource_manager = resource_manager::ResourceManager::new();

  resource_manager.insert("res", Box::new(cdn));

  resource_manager.load_async("res:/dx9/scene/universe/a01_cube.red").and_then(|v| {
    println!("{:?}", v);
  }).await().unwrap();
}

