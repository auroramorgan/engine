use std::fs;

use std::io;
use std::io::{BufRead, Read, Write};

use std::path;

use hyper;
use hyper::mime::Mime;

use resource_loaders;

pub struct Loader {
  prefix: String,
  cache_dir: String,
  client: hyper::Client
}

impl Loader {
  pub fn new(prefix: &str, cache_dir: &str) -> Loader {
    fs::create_dir_all(cache_dir).unwrap();

    return Loader {
      prefix: prefix.to_owned(),
      cache_dir: cache_dir.to_owned(),
      client: hyper::Client::new()
    };
  }

  fn load_from_cache(&self, path: &str) -> Option<(Mime, Vec<u8>)> {
    let file = self.cache_dir.clone() + path;

    let mut f = match fs::File::open(file) {
      Ok(f) => io::BufReader::new(f), _ => return None
    };

    let mut mime_string = String::new();
    f.read_line(&mut mime_string).unwrap();
    let mime = mime_string.parse().unwrap();

    return Some((mime, load_from_stream(&mut f)));
  }

  fn load_from_cdn(&self, path: &str) -> Option<(Mime, Vec<u8>)> {
    let url = self.prefix.clone() + path;

    let mut response = match self.client.get(&url).send() {
      Ok(r) => r, _ => return None
    };

    let headers = response.headers.clone();
    let mime = match headers.get::<hyper::header::ContentType>() {
      Some(&hyper::header::ContentType(ref s)) => s.clone(),
      None => "application/octet-stream".parse().unwrap()
    };

    let data = load_from_stream(&mut response);

    let file = self.cache_dir.clone() + path;

    fs::create_dir_all(path::Path::new(file.as_str()).parent().unwrap()).unwrap();
    let mut f = fs::File::create(file).unwrap();

    write_to_stream(&mut f, &mime, &data);

    return Some((mime, data));
  }
}

impl resource_loaders::ResourceLoader for Loader {
  fn load(&self, path: &str) -> Option<(Mime, Vec<u8>)> {
    let result = self.load_from_cache(path);

    return match result {
      None => self.load_from_cdn(path),
      Some(s) => Some(s)
    };
  }
}

fn load_from_stream(read: &mut Read) -> Vec<u8> {
  let mut body = Vec::new();

  read.read_to_end(&mut body).unwrap();

  return body;
}

fn write_to_stream(write: &mut Write, mime: &Mime, data: &Vec<u8>) {
  write.write_all(format!("{}\n", mime).as_bytes()).unwrap();
  write.write_all(data.as_slice()).unwrap();
}