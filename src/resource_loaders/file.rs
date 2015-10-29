use std::env;
use std::fs;

use std::io::Read;

use resource_loaders;

pub struct Loader {
  prefix: String
}

impl Loader {
  pub fn new(prefix: &str) -> Loader {
    return Loader {
      prefix: prefix.to_owned()
    };
  }

  pub fn default() -> Loader {
    return Loader::new(env::current_dir().unwrap().to_str().unwrap());
  }
}

impl resource_loaders::ResourceLoader for Loader {
  fn load(&self, path: &str) -> Option<(String, Vec<u8>)> {
    let path = format!("{}{}", self.prefix, path);

    if let Ok(mut file) = fs::File::open(&path) {
      let mut result = Vec::new();

      let _ = file.read_to_end(&mut result).unwrap();

      let mime = resource_loaders::path_to_mime(&path).unwrap_or("application/octet-stream");

      return Some((mime.to_owned(), result));
    }

    return None;
  }
}
