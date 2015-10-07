#![feature(convert)]

extern crate engine;
extern crate getopts;

use std::env;

fn main() {
  let resource_manager = engine::resource_manager::ResourceManager::default();

  let sof = engine::resource_manager::sof::Manager::new(resource_manager.clone());

  let args: Vec<String> = env::args().collect();

  match args[1].as_str() {
    "hulls" => {
      println!("{}", sof.hulls().join("\n"));
    },
    "export" => {
      let geometry_path = if args[2].starts_with("res:/") {
        args[2].as_str()
      } else {
        match sof.load_hull(args[2].as_str()) {
          None => {
            println!("No such hull {}", args[2]);

            return;
          }
          Some(s) => s
        }.geometry_path()
      };

      let geometry = resource_manager.load(geometry_path).unwrap();
      
      let obj = match *geometry {
        engine::resource_manager::Resource::Mesh(ref m) => engine::exporter::obj::export(m),
        _ => panic!("{:?} is not a mesh, sorry.", geometry_path)
      };

      println!("{}", obj);
    }
    cmd => panic!("Unknown command {}", cmd)
  }
}