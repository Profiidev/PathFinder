use serde::{Serialize, Deserialize};
use std::{fs, panic, thread, sync::atomic::{AtomicUsize, Ordering}, cell::RefCell, time::UNIX_EPOCH, os::windows::prelude::*, path::PathBuf};
use filesize::PathExt;
use crate::{tree::{Tree, Node}, config::Config};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct File {
  pub name: String,
  pub is_dir: bool,
  pub size: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct FileData {
  pub file: File,
  pub path: String,
	pub last_modified_date: Option<u64>,
	pub created_date: Option<u64>,
	pub permissions: Option<bool>,
  pub hidden: Option<bool>,
}

impl FileData {
  pub fn update_file_data(&mut self) {
    let metadata = match fs::metadata(&self.path) {
      Ok(metadata) => metadata,
      Err(_) => return,
    };
    
    let modified = match metadata.modified() {
        Ok(modified) => match modified.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => 0,
        },
        Err(_) => 0,
    };

    let created = match metadata.created() {
        Ok(created) => match created.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => 0,
        },
        Err(_) => 0,
    };

    let attributes = metadata.file_attributes();

    self.last_modified_date = Some(modified);
    self.created_date = Some(created);
    self.permissions = Some(metadata.permissions().readonly());
    self.hidden = Some(attributes & 2 == 2);
  }
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

static THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

impl FileSystem {
  pub fn load_locations(config: &mut Config) -> Vec<Tree> {
    let mut trees = Vec::new();
    for id in &config.tree_data.saved_tree_ids.clone() {
      trees.push(config.load_tree(id.to_string()));
    }
    trees
  }

  pub async fn gen_tree(location: &str, id: &str) -> Tree {
    let root = Node::new(File {
      name: location.to_string().replace("\\", "/"),
      is_dir: true,
      size: 0
    });
    let mut tree = Tree::new(root, id.to_string());
    let mut children = Vec::new();

    let child_dirs = match fs::read_dir(location.to_string() + "/") {
      Ok(child_dirs) => child_dirs,
      Err(_) => {
        println!("Error reading directory: {}", location);
        tree.id = "error".to_string();
        return tree;
      }
    };
    let mut child_dirs_file = Vec::new();

    for entry in child_dirs {
      let entry = entry.unwrap();
      let path = entry.path();
      let metadata = match fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(_) => {
          println!("Error reading metadata for: {}", location);
          continue;
        }
      };
      let is_dir = metadata.is_dir();
      child_dirs_file.push(File {
        name: entry.file_name().to_str().unwrap().to_string(),
        is_dir: is_dir,
        size: 0
      });
    }

    File::sort_files(&mut child_dirs_file);

    let mut child_threads = Vec::new();

    for file in child_dirs_file {
      if file.is_dir && !file.name.starts_with("$") {
        THREAD_COUNT.fetch_add(1, Ordering::SeqCst);

        let location = RefCell::new(location.to_string());
        let file_name = RefCell::new(file.name.clone());

        child_threads.push(thread::spawn(move || {
          let result = panic::catch_unwind(move || {

            let location = location.borrow();
            let file_name = file_name.borrow();

            FileSystem::traverse_files(&location, &file_name)
          });

          match result {
              _ => {}
          }
          THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);

          result
        }));
      } else {
        let metadata = match fs::metadata(format!("{}/{}", location, file.name)) {
          Ok(metadata) => metadata,
          Err(_) => {
            println!("Error reading metadata for: {}", location);
            continue;
          }
        };
        let mut size = metadata.len();
        if size > 100000 {
          size = match PathBuf::from(format!("{}/{}", location, file.name)).size_on_disk_fast(&metadata) {
            Ok(size) => size,
            Err(_) => 0,
          };
        }
        children.push(Node::new(File {
          name: file.name,
          is_dir: false,
          size: size
        }));
      }
    }

    while THREAD_COUNT.load(Ordering::SeqCst) > 0 {
      thread::sleep(std::time::Duration::from_millis(1));
    }

    for child_thread in child_threads {
      let child = child_thread.join().unwrap();
      match child {
        Ok(child) => {
          children.extend(child);
        },
        Err(_) => {
          println!("Error reading directory: {}", location);
          tree.id = "error".to_string();
          return tree;
        }
      }
    }

    children.sort_by(|a, b| {a.compare(&b.data)});

    tree.get_root_mut().data.size = children.iter().map(|x| x.data.size).sum();
    tree.get_root_mut().add_children(children);

    return tree;
  }

  fn traverse_files(parent: &str, name: &str) -> Vec<Node> {
    let mut root = Node::new(File {
      name: name.to_string(),
      is_dir: true,
      size: 0
    });
    let mut children = Vec::new();

    let child_dirs = match fs::read_dir(format!("{}/{}", parent, name)) {
      Ok(child_dirs) => child_dirs,
      Err(err) => {
        println!("Error: {:?}", err);
        println!("Error reading directory: {}/{}", parent, name);
        return Vec::new();
      }
    };
    let mut child_dirs_file = Vec::new();

    for entry in child_dirs {
      let entry = entry.unwrap();
      let path = entry.path();
      let metadata = match fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(_) => {
          println!("Error reading metadata for: {}/{}", parent, name);
          continue;
        }
      };
      let is_dir = metadata.is_dir();
      let mut size = metadata.len();
      if size > 100000 {
        size = match path.size_on_disk_fast(&metadata) {
          Ok(size) => size,
          Err(_) => 0,
        };
      }

      child_dirs_file.push(File {
        name: entry.file_name().to_str().unwrap().to_string(),
        is_dir: is_dir,
        size: size
      });
    }

    File::sort_files(&mut child_dirs_file);

    for file in child_dirs_file {
      if file.is_dir {
        children.extend(FileSystem::traverse_files(&format!("{}/{}", parent, name), &file.name));
      } else {
        children.push(Node::new(File {
          name: file.name,
          is_dir: false,
          size: file.size
        }));
      }
    }

    root.data.size = children.iter().map(|x| x.data.size).sum();
    root.add_children(children);

    return Vec::from([root]);
  }
}