use std::{fs::File, io::Write, io::Read};
use serde::{Serialize, Deserialize};
use crate::tree::Tree;

pub struct Config {
  pub path: String,
  pub settings: Settings,
  pub tree_data: TreeData,
}

impl Config {
  pub fn new(path: &str) -> Config {
    create_all_dirs(path);
    Config::load(path)
  }

  fn load(path: &str) -> Config {
    let config = Config {
      path: path.to_string(),
      settings: match path_exists(&(path.to_string() + "/settings.json")) {
        true => Settings::load(&(path.to_string() + "/settings.json")),
        false => Settings::new(),
      },
      tree_data: match path_exists(&(path.to_string() + "/tree_data.json")) {
        true => TreeData::load(&(path.to_string() + "/tree_data.json")),
        false => TreeData::new(),
      },
    };
    config
  }

  pub fn save(&mut self, trees: &Vec<Tree>) {
    create_all_dirs(&(self.path.to_string() + "/trees/"));
    for tree in trees {
      self.save_tree(&tree);
    }
    self.settings.save(&(self.path.to_string() + "/settings.json"));
    self.tree_data.save(&(self.path.to_string() + "/tree_data.json"));
  }

  pub fn save_tree(&mut self, tree: &Tree) {
    let path = self.path.to_string() + "/trees/" + &tree.id + ".json";
    save_to_file(&tree, &path);
  }

  pub fn load_tree(&mut self, id: String) -> Tree {
    let path = self.path.to_string() + "/trees/" + &id + ".json";
    load_from_file(&path)
  }

  pub fn create_tree(&mut self, tree: &Tree) {
    self.tree_data.saved_tree_ids.push(tree.id.to_string());
    self.save_tree(&tree);
    self.tree_data.save(&(self.path.to_string() + "/tree_data.json"));
  }

  pub fn remove_tree(&mut self, id: String) {
    self.tree_data.saved_tree_ids.retain(|x| x != &id);
    let path = self.path.to_string() + "/trees/" + &id + ".json";
    std::fs::remove_file(path).unwrap();
    self.tree_data.save(&(self.path.to_string() + "/tree_data.json"));
  }
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
  pub settings: String
}

impl Settings {
  pub fn new() -> Settings {
    Settings {settings: "".to_string()}
  }

  pub fn load(file: &str) -> Settings {
    load_from_file(file)
  }

  pub fn save(&mut self, file: &str) {
    save_to_file(&self, file);
  }
}

#[derive(Serialize, Deserialize)]
pub struct TreeData {
  pub saved_tree_ids: Vec<String>
}

impl TreeData {
  pub fn new() -> TreeData {
    TreeData {
      saved_tree_ids: Vec::new(),
    }
  }

  pub fn load(file: &str) -> TreeData {
    load_from_file(file)
  }

  pub fn save(&mut self, file: &str) {
    save_to_file(&self, file);
  }
}

fn create_all_dirs(path: &str) {
  let path = std::path::Path::new(path);
  if !path.exists() {
    std::fs::create_dir_all(path).unwrap();
  }
}

fn path_exists(path: &str) -> bool {
  let path = std::path::Path::new(path);
  path.exists()
}

fn load_from_file<T: for<'de> Deserialize<'de>>(file: &str) -> T {
  let mut file = File::open(file).unwrap();
  let mut json = String::new();
  file.read_to_string(&mut json).unwrap();
  serde_json::from_str(&json).unwrap()
}

fn save_to_file<T: Serialize>(data: &T, file: &str) {
  let json = serde_json::to_string(&data).unwrap();
  let mut file = File::create(file).unwrap();
  file.write_all(json.as_bytes()).unwrap();
}