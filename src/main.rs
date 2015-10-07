#![feature(convert)]

extern crate engine;
extern crate getopts;

use std::env;

fn main() {
  let resource_manager = engine::resource_manager::ResourceManager::default();

  let manager = engine::resource_manager::sof::Manager::new(resource_manager);

  let args: Vec<String> = env::args().collect();

  match args[1].as_str() {
    "hulls" => {
      println!("{}", manager.hulls().join("\n"));
    },
    "export" => {
      println!("{:?}", manager.load_hull(args[2].as_str()));
    }
    cmd => panic!("Unknown command {}", cmd)
  }
}