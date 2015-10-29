pub mod sof;

use std;
use std::thread;

use std::collections::HashMap;
use std::sync::Arc;

use asset;
use buffer;
use importer;

use resource_loaders;
use resource_loaders::ResourceLoader;

#[derive(Debug)]
pub enum Resource {
  Binary(Arc<buffer::Buffer>),
  Text(String),
  Asset(asset::Asset)
}

pub struct ResourceManager {
  loaders: HashMap<&'static str, Arc<Box<ResourceLoader>>>
}

impl ResourceManager {
  pub fn new() -> ResourceManager {
    return ResourceManager {
      loaders: HashMap::new()
    };
  }

  pub fn default() -> Arc<ResourceManager> {
    let mut resource_manager = ResourceManager::new();

    resource_manager.insert("res", Box::new(resource_loaders::cdn::Loader::default()));
    resource_manager.insert("file", Box::new(resource_loaders::file::Loader::default()));

    return Arc::new(resource_manager);
  }

  pub fn insert(&mut self, k: &'static str, loader: Box<ResourceLoader>) {
    self.loaders.insert(k, Arc::new(loader));
  }

  fn parse_path<'a>(&'a self, path: &str) -> Option<(&'a Arc<Box<ResourceLoader>>, String)> {
    let mut split = path.splitn(2, ':');

    let prefix = match split.next() {
      Some(s) => s, None => return None
    };

    let rest = match split.next() {
      Some(s) => s, None => return None
    };

    let loader = match self.loaders.get(prefix) {
      Some(s) => s, None => return None
    };

    return Some((loader, rest.to_owned()));
  }

  pub fn prefetch(&self, path: &str) {
    let p = self.parse_path(path);

    match p {
      Some((l, r)) => {
        let loader = l.clone();

        let _ = thread::spawn(move || { loader.prefetch(r.as_str()) });
      },
      None => ()
    };
  }

  pub fn load(&self, path: &str) -> Option<Arc<Resource>> {
    let p = self.parse_path(path);

    return match p {
      Some((l, r)) => {
        if let Some((mime, data)) = l.load(r.as_str()) {
          Some(to_resource(mime, buffer::Buffer::new(Some(path.to_owned()), None, data)))
        } else {
          None
        }
      }
      _ => None
    };
  }
}

// TODO: Make this pluggable
fn to_resource(mime: String, data: Arc<buffer::Buffer>) -> Arc<Resource> {
  let result = match mime.as_str() {
    "application/octet-stream" => Resource::Binary(data),
    "text/xml" => Resource::Text(std::str::from_utf8(&data[..]).unwrap().to_owned()),
    "text/html" => Resource::Text(std::str::from_utf8(&data[..]).unwrap().to_owned()),
    "application/x-ccp-red" => Resource::Binary(data),
    "application/x-ccp-wbg" => {
      Resource::Asset(importer::wbg::import(data).unwrap())
    }
    _ => {
      println!("Unknown MIME {:?}, interpreting as Binary", mime);
      Resource::Binary(data)
    }
  };

  return Arc::new(result);
}
