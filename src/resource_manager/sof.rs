#![allow(non_snake_case)]

use std::io;

use std::sync::Arc;

use std::collections::HashMap;

use xml::reader::{EventReader, XmlEvent};

use rustc_serialize::json;

use resource_manager;

#[derive(Debug, RustcDecodable, PartialEq)]
struct SofData {
  race: HashMap<String, Race>,
  faction: HashMap<String, Faction>,
  hull: HashMap<String, Hull>
}

#[derive(Debug, RustcDecodable, PartialEq)]
pub struct Race {
  name: String
}

#[derive(Debug, RustcDecodable, PartialEq)]
pub struct Faction {
  name: String
}

#[derive(Debug, RustcDecodable, PartialEq)]
pub struct Hull {
  name: String,
  description: String,
  geometryResFilePath: String
}

impl Hull {
  pub fn geometry_path(&self) -> &str {
    return self.geometryResFilePath.as_str();
  }
}

pub struct Manager {
  manager: Arc<resource_manager::ResourceManager>,
  data: Option<SofData>
}

static SOF_PATH: &'static str = "res:/dx9/model/spaceobjectfactory/data.red";

impl Manager {
  pub fn new(manager: Arc<resource_manager::ResourceManager>) -> Manager {
    return Manager { manager: manager, data: None };
  }

  fn initialize(&mut self) {
    if let Some(_) = self.data { return };

    let data = self.manager.load(SOF_PATH).unwrap();

    let string = match *data {
      resource_manager::Resource::Binary(ref s) => s.clone(),
      _ => panic!("Silly input!")
    };

    let mut json = String::new();

    for e in EventReader::new(io::Cursor::new(&string[..])) {
      match e {
        Err(e) => panic!("I/O Error while reading from memory? {:?}", e),
        Ok(XmlEvent::Characters(s)) => json.push_str(s.as_str()),
        _ => ()
      }
    }

    self.data = Some(json::decode(json.as_str()).unwrap());
  }

  pub fn hulls<'a>(&'a mut self) -> Vec<&'a str> {
    return match self.data {
      Some(ref data) => data.hull.keys().map(|x| x.as_str()).collect::<Vec<&'a str>>(),
      None => {
        self.initialize();
        self.hulls()
      }
    };
  }

  pub fn load_hull<'a>(&'a mut self, name: &str) -> Option<&'a Hull> {
    return match self.data {
      Some(ref data) => data.hull.get(name),
      None => {
        self.initialize();
        self.load_hull(name)
      }
    };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use resource_manager;

  #[test]
  fn test_sof_manager() {
    let resource_manager = resource_manager::ResourceManager::default();

    let manager = Manager::new(resource_manager);

    assert_eq!(manager.load_hull("ai2_t2"), Some(&Hull {
      name: "ai2_t2".to_owned(),
      description: "ship/amarr/industrial/ai2".to_owned(),
      geometryResFilePath: "res:/dx9/model/ship/amarr/industrial/ai2/ai2_t2.wbg".to_owned()
    }));
  }
}
