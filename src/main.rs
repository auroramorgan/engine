#![feature(convert)]

extern crate engine;
extern crate getopts;

use std::fs;
use std::env;

use std::io::Write;
use std::sync::Arc;

fn main() {
  let resource_manager = engine::resource_manager::ResourceManager::default();

  let mut sof = engine::resource_manager::sof::Manager::new(resource_manager.clone());

  let args: Vec<String> = env::args().collect();

  match args[1].as_str() {
    "hulls" => {
      println!("{}", sof.hulls().join("\n"));
    },
    "export" => {
      for i in 3 .. args.len() {
        export(&mut sof, resource_manager.clone(), &args[i], &args[2]);
      }
    }
    cmd => panic!("Unknown command {}", cmd)
  }
}

fn export(sof: &mut engine::resource_manager::sof::Manager, resource_manager: Arc<engine::resource_manager::ResourceManager>, arg: &String, destination: &String) {
  let geometry_path = if arg.contains(":/") {
    arg.as_str()
  } else {
    match sof.load_hull(arg.as_str()) {
      None => {
        println!("No such hull {}", arg);

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

  for (path, obj) in objs {
    let dirpath = format!("{}/{}", &destination, arg);

    fs::create_dir_all(&dirpath).unwrap();

    let filename = format!("{}/{}", dirpath, path);
    println!("Writing {} (to {})", path, filename);

    let mut f = fs::File::create(filename).unwrap();

    f.write_all(obj.as_slice()).unwrap();
  }
}
