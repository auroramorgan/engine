#![feature(convert)]

extern crate engine;
extern crate getopts;

use std::fs;
use std::env;

use std::io::Write;

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
      
      let objs = match *geometry {
        engine::resource_manager::Resource::Asset(ref m) => engine::exporter::obj::export(m),
        _ => panic!("{:?} is not an asset, sorry.", geometry_path)
      };

      println!("{:?}", geometry);

      let dir = args[3].clone();
      fs::create_dir_all(&dir).unwrap();

      for (path, obj) in objs {
        println!("{:?}", dir.clone() + path.as_str());
        let mut f = fs::File::create(dir.clone() + path.as_str()).unwrap();
        f.write_all(obj.as_bytes()).unwrap();
      }
    }
    cmd => panic!("Unknown command {}", cmd)
  }
}