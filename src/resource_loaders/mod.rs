use std::path;

pub mod cdn;
pub mod file;

pub trait ResourceLoader : Send + Sync {
  fn load(&self, path: &str) -> Option<(String, Vec<u8>)>;

  fn prefetch(&self, path: &str) {
    let _ = self.load(path);
  }
}

pub fn path_to_mime(path: &str) -> Option<&'static str>{
  let extension = path::Path::new(path).extension().map(|x| x.to_str().unwrap()).unwrap_or("");

  let mime = match extension {
    "wbg" => "application/x-ccp-wbg",
    "red" => "application/x-ccp-red",
    "obj" => "application/x-wavefront-obj",
    _ => return None
  };

  return Some(mime);
}