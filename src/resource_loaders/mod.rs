pub mod cdn;

use hyper::mime::Mime;

pub trait ResourceLoader : Send + Sync {
  fn load(&self, path: &str) -> Option<(Mime, Vec<u8>)>;

  fn prefetch(&self, path: &str) {
    let _ = self.load(path);
  }
}
