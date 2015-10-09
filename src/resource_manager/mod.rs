pub mod sof;

use std::sync::Arc;

use std::thread;

use std::collections::HashMap;

use eventual::Future;

use asset;
use importer;

use resource_loaders;
use resource_loaders::ResourceLoader;

#[derive(Debug)]
pub enum Resource {
  Binary(Vec<u8>),
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
      Some((l, r)) => to_resource(l.load(r.as_str())),
      _ => None
    };
  }

  pub fn load_async(&self, path: &str) -> Future<Arc<Resource>, ()> {
    let p = self.parse_path(path);

    return match p {
      Some((l, r)) => {
        let loader = l.clone();

        Future::spawn(move || { to_resource(loader.load(r.as_str())).unwrap() })
      },
      None => {
        Future::error(())
      }
    };
  }
}

// TODO: Make this pluggable
fn to_resource(raw: Option<(String, Vec<u8>)>) -> Option<Arc<Resource>> {
  if let Some((mime, data)) = raw {
    let result = match mime.as_str() {
      "application/octet-stream" => Resource::Binary(data),
      "text/xml" => Resource::Text(String::from_utf8(data).unwrap()),
      "text/html" => Resource::Text(String::from_utf8(data).unwrap()),
      "application/x-ccp-red" => Resource::Binary(data),
      "application/x-ccp-wbg" => Resource::Asset(importer::wbg::import(data).unwrap()),
      _ => {
        println!("Unknown MIME {:?}, interpreting as Binary", mime);
        Resource::Binary(data)
      }
    };

    return Some(Arc::new(result));
  } else {
    return None;
  }
}
