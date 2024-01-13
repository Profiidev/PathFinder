use serde::{Serialize, Deserialize};
use std::fs;
use crate::{tree::{Tree, Node}, config::Config};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct File {
  pub name: String,
  pub is_dir: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct FileData {
  pub file: File,
  pub path: String,
}

impl File {
  pub fn sort_files(files: &mut Vec<File>) {
    files.sort_by(|a, b| {
      if a.is_dir && !b.is_dir {
        return std::cmp::Ordering::Less;
      } else if !a.is_dir && b.is_dir {
        return std::cmp::Ordering::Greater;
      } else {
        return a.name.cmp(&b.name);
      }
    });
  }
}

pub struct FileSystem;

impl FileSystem {
  pub fn load_locations(config: &mut Config) -> Vec<Tree> {
    let mut trees = Vec::new();
    for id in &config.tree_data.saved_tree_ids.clone() {
      trees.push(config.load_tree(id.to_string()));
    }
    trees
  }

  pub fn gen_tree(location: &str, id: String) -> Tree {
    let root = Node::new(File {
      name: location.to_string().replace("\\", "/"),
      is_dir: true
    });
    let mut tree = Tree::new(root, id);
    let mut children = Vec::new();

    let child_dirs = match fs::read_dir(location.to_string() + "/") {
      Ok(child_dirs) => child_dirs,
      Err(_) => {
        print!("Error reading directory: {}", location);
        tree.id = "error".to_string();
        return tree;
      }
    };
    let mut child_dirs_file = Vec::new();

    for entry in child_dirs {
      let entry = entry.unwrap();
      let path = entry.path();
      let metadata = fs::metadata(&path).unwrap();
      let is_dir = metadata.is_dir();
      child_dirs_file.push(File {
        name: entry.file_name().to_str().unwrap().to_string(),
        is_dir: is_dir
      });
    }

    File::sort_files(&mut child_dirs_file);

    for file in child_dirs_file {
      if file.is_dir && !file.name.starts_with("$") {
        children.push(FileSystem::traverse_files(location, &file.name));
      } else {
        children.push(Node::new(File {
          name: file.name,
          is_dir: false
        }));
      }
    }

    tree.get_root_mut().add_children(children);

    return tree;
  }

  fn traverse_files(parent: &str, name: &str) -> Node {
    let mut root = Node::new(File {
      name: name.to_string(),
      is_dir: true
    });
    let mut children = Vec::new();

    let child_dirs = match fs::read_dir(format!("{}/{}", parent, name)) {
      Ok(child_dirs) => child_dirs,
      Err(_) => {
        print!("Error reading directory: {}/{}", parent, name);
        return root;
      }
    };
    let mut child_dirs_file = Vec::new();

    for entry in child_dirs {
      let entry = entry.unwrap();
      let path = entry.path();
      let metadata = match fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(_) => {
          print!("Error reading metadata for: {}/{}", parent, name);
          continue;
        }
      };
      let is_dir = metadata.is_dir();
      child_dirs_file.push(File {
        name: entry.file_name().to_str().unwrap().to_string(),
        is_dir: is_dir
      });
    }

    File::sort_files(&mut child_dirs_file);

    for file in child_dirs_file {
      if file.is_dir {
        children.push(FileSystem::traverse_files(&format!("{}/{}", parent, name), &file.name));
      } else {
        children.push(Node::new(File {
          name: file.name,
          is_dir: false
        }));
      }
    }

    root.add_children(children);

    return root;
  }
}