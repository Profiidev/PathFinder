use std::fs;
use std::path::{PathBuf, Path};
use notify::{Event, EventKind, RecursiveMode, Watcher, ReadDirectoryChangesWatcher};
use notify::event::{ModifyKind, RenameMode, CreateKind};
use tauri::Manager;
use filesize::PathExt;
use crate::file::{File, FileData};
use crate::tree::Tree;
use crate::StateData;
use crate::APP;

pub struct FileUpdateHandler {
  pub paths: Vec<PathBuf>,
  pub watcher: Option<notify::RecommendedWatcher>,
}

impl FileUpdateHandler {
    pub fn inti_locations(locations: &Vec<String>) -> Self {
      let mut watcher = match get_watcher() {
        Some(watcher) => watcher,
        None => return FileUpdateHandler {
          paths: Vec::new(),
          watcher: None,
        },
      };

      for location in locations {
        match watcher.watch(Path::new(&(location.replace("\\", "/").to_string() + "/")), RecursiveMode::Recursive) {
          Ok(_) => (),
          Err(_) => {
            println!("Error watching directory");
            return FileUpdateHandler {
              paths: Vec::new(),
              watcher: None,
            };
          }
        };
      }

      FileUpdateHandler {
        paths: Vec::new(),
        watcher: Some(watcher),
      }
    }

    pub fn add_location(locations: &Vec<String>) -> Option<ReadDirectoryChangesWatcher> {
        let mut watcher = match get_watcher() {
            Some(watcher) => watcher,
            None => return None,
        };

        for location in locations {
            match watcher.watch(Path::new(&(location.replace("\\", "/").to_string() + "/")), RecursiveMode::Recursive) {
            Ok(_) => (),
            Err(_) => {
                println!("Error watching directory");
                return None;
            }
            };
        }

        Some(watcher)
    }

    pub fn handle_file_updates(&mut self, event: &Event, trees: &mut Vec<Tree>) {
        match event.kind {
            EventKind::Create(create_kind) => {
                match create_kind {
                    CreateKind::Any | CreateKind::File | CreateKind::Folder => {
                        handle_file_creation(event.paths.clone(), trees);
                    },
                    _ => (),
                }
            },
            EventKind::Remove(_) => {
                handle_file_deletion(event.paths.clone(), trees);
            },
            EventKind::Modify(modify_kind) => {
                match modify_kind {
                    ModifyKind::Name(rename_mode) => {
                        match rename_mode {
                            RenameMode::From => {
                                self.paths = event.paths.clone();
                            },
                            RenameMode::To => {
                                let mut paths = Vec::new();
                                paths.push(self.paths[0].clone());
                                paths.push(event.paths[0].clone());
                                handle_file_rename(paths, trees);
                            },
                            RenameMode::Both => {
                                handle_file_rename(event.paths.clone(), trees);
                            },
                            _ => {
                                handle_file_update(event.paths.clone(), trees);
                            },
                        }
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }
}

fn get_watcher() -> Option<ReadDirectoryChangesWatcher> {
    match notify::recommended_watcher(|res| {
        let handle = APP.handle.lock().unwrap();
        let state = match handle.as_ref() {
        Some(handle) => {
            handle.state::<StateData>()
        },
        None => return,
        };
        match res {
            Ok(event) => {
            let mut event_handler = state.1.lock().unwrap();
            let mut trees = state.0.lock().unwrap();
            event_handler.handle_file_updates(&event, &mut *trees);
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }) {
        Ok(watcher) => Some(watcher),
        Err(_) => {
            println!("Error creating watcher");
            return None;
        }
    }
}

fn handle_file_creation(paths: Vec<PathBuf>, trees: &mut Vec<Tree>) {
    let file = get_file(paths[0].clone());
    let path = file.path.clone();

    for tree in trees {
        if !tree.is_in_tree(&path) {
            continue;
        }

        tree.add_node(&path, file.file.clone());
    }
}

fn handle_file_deletion(paths: Vec<PathBuf>, trees: &mut Vec<Tree>) {
    let path = match paths[0].to_str() {
        Some(path) => path.replace("\\", "/"),
        None => return,
    };

    for tree in trees {
        if !tree.is_in_tree(&path) {
            continue;
        }

        tree.remove_node(&path);
    }
}

fn handle_file_rename(paths: Vec<PathBuf>, trees: &mut Vec<Tree>) {
    let file = get_file(paths[1].clone());
    let old_path = match paths[0].to_str() {
        Some(path) => path.replace("\\", "/"),
        None => return,
    };

    for tree in trees {
        if !tree.is_in_tree(&old_path) {
            continue;
        }

        tree.edit_node(&old_path, file.file.clone());
    }
}

fn handle_file_update(paths: Vec<PathBuf>, trees: &mut Vec<Tree>) {
    let file = get_file(paths[0].clone());
    let path = file.path.clone();

    let metadata = match fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(_) => {
            println!("Error reading metadata for file: {}", path);
            return;
        }
    };

    let mut size = metadata.len();
      if size > 100000 {
        size = match PathBuf::from(&path).size_on_disk_fast(&metadata) {
          Ok(size) => size,
          Err(_) => 0,
        };
      }

    for tree in trees {
        if !tree.is_in_tree(&path) {
            continue;
        }

        tree.edit_file_size(&path, size - tree.get_file_size(&path));
    }
}

fn get_file(path: PathBuf) -> FileData {
    let path = path.to_str().unwrap().replace("\\", "/");

    let mut file = File {
        name: path.split("/").last().unwrap().to_string(),
        is_dir: false,
        size: 0,
    };

    let metadata = match fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(_) => {
            println!("Error reading metadata for file: {}", path);
            return FileData {
                file,
                path,
                last_modified_date: None,
                created_date: None,
                permissions: None,
                hidden: None,
            };
        }
    };
    let is_dir = metadata.is_dir();
    file.is_dir = is_dir;

    FileData {
        file,
        path,
        last_modified_date: None,
        created_date: None,
        permissions: None,
        hidden: None,
    }
}