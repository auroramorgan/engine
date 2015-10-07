pub mod cdn;

pub trait ResourceLoader : Send + Sync {
  fn load(&self, path: &str) -> Option<(String, Vec<u8>)>;

  fn prefetch(&self, path: &str) {
    let _ = self.load(path);
  }
}
