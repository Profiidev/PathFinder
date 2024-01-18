use tauri::Manager;
use uuid::Uuid;
use crate::tree::Tree;
use crate::file::{FileSystem, FileData};
use crate::StateData;
use crate::FileUpdateHandler;
use crate::SEARCH_ID;

#[tauri::command]
pub fn save_settings(settings: String, state: tauri::State<StateData>, app_handle: tauri::AppHandle) -> bool {
  let mut config = match state.2.lock() {
    Ok(config) => config,
    Err(_) => return false,
  };

  config.settings.settings = settings;
  
  let config_path = config.path.clone() + "/settings.json";
  config.settings.save(&(config_path));

  app_handle.emit_all("update-settings", None::<String>).unwrap();
  true
}

#[tauri::command]
pub fn get_settings(state: tauri::State<StateData>) -> String {
  let config = match state.2.lock() {
    Ok(config) => config,
    Err(_) => return String::new(),
  };

  config.settings.settings.clone()
}

#[tauri::command]
pub async fn search_partial(name: String, path: String, use_regex: bool, index_start: i32, index_end: i32, search_id: i32, state: tauri::State<'_,StateData>) -> Result<Vec<FileData>, ()> {
  SEARCH_ID.store(search_id, std::sync::atomic::Ordering::Relaxed);

  let mut found = Vec::new();
  let path = path.replace("\\", "/");

  let trees = match state.0.lock() {
    Ok(trees) => trees,
    Err(_) => return Ok(found),
  };
  for tree in trees.iter() {
    if !tree.is_in_tree(&path) {
      continue;
    }

    found = tree.search_partial(&path, &name, use_regex, search_id);

    if found.len() > 0 {
      break;
    }
  }

  if SEARCH_ID.load(std::sync::atomic::Ordering::Relaxed) != search_id {
      return Ok(Vec::new());
  }

  if index_end == -1 {
    found.iter_mut().for_each(|file| file.update_file_data());
    return Ok(found);
  }

  let index_start = index_start as usize;
  let index_end = index_end as usize;

  if index_start >= found.len() {
    return Ok(Vec::new());
  }

  if index_end >= found.len() {
    found = found[index_start..].to_vec();
  } else {
    found = found[index_start..index_end].to_vec();
  }

  found.iter_mut().for_each(|file| file.update_file_data());
  Ok(found)
}

#[tauri::command]
pub async fn add_location(location: String, state: tauri::State<'_,StateData>, app_handle: tauri::AppHandle) -> Result<bool, ()> {
  {
    let trees = match state.0.lock() {
      Ok(trees) => trees,
      Err(_) => return Ok(false),
    };
    let config = match state.2.lock() {
      Ok(config) => config,
      Err(_) => return Ok(false),
    };
    if trees.iter().any(|tree| tree.get_root().data.name == location.replace("\\", "/")) {
        return Ok(false);
    }

    std::mem::drop(trees);
    std::mem::drop(config);
  }

  let tree = FileSystem::gen_tree(&location, &Uuid::new_v4().to_string()).await;
  if tree.id == "error" {
    return Ok(false);
  }

  let mut trees = match state.0.lock() {
    Ok(trees) => trees,
    Err(_) => return Ok(false),
  };

  let mut watcher = match state.1.lock() {
    Ok(watcher) => watcher,
    Err(_) => return Ok(false),
  };

  let mut config = match state.2.lock() {
    Ok(config) => config,
    Err(_) => return Ok(false),
  };

  config.create_tree(&tree);
  trees.push(tree);

  watcher.watcher = FileUpdateHandler::add_location(&trees.iter().map(|tree| tree.get_root_location()).collect::<Vec<String>>());

  app_handle.emit_all("update-locations", None::<String>).unwrap();
  Ok(true)
}

#[tauri::command]
pub fn remove_location(location: String, state: tauri::State<StateData>, app_handle: tauri::AppHandle) -> bool {
  let mut trees = match state.0.lock() {
    Ok(trees) => trees,
    Err(_) => return false,
  };
  let mut index = 0;
  for tree in trees.iter() {
    if tree.get_root().data.name == location.replace("\\", "/") {
      break;
    }
    index += 1;
  }
  let mut config = match state.2.lock() {
    Ok(config) => config,
    Err(_) => return false,
  };

  let tree = trees.remove(index);
  config.remove_tree(tree.id);

  app_handle.emit_all("update-locations", None::<String>).unwrap();
  true
}

#[tauri::command]
pub async fn reindex_location(location: String, state: tauri::State<'_,StateData>) -> Result<bool, ()> {
  let tree_index;
  let id ={
    let trees = match state.0.lock() {
      Ok(trees) => trees,
      Err(_) => return Ok(false),
    };
    let mut index = 0;
    for tree in trees.iter() {
      if tree.get_root().data.name == location.replace("\\", "/") {
        break;
      }
      index += 1;
    }
    if index >= trees.len() {
      return Ok(false);
    }
    let id = trees[index].id.clone();
    tree_index = index;

    std::mem::drop(trees);

    id
  };
  
  let tree = FileSystem::gen_tree(&location, &id).await;

  if tree.id == "error" {
    return Ok(false);
  }

  let mut trees = match state.0.lock() {
    Ok(trees) => trees,
    Err(_) => return Ok(false),
  };

  let mut config = match state.2.lock() {
    Ok(config) => config,
    Err(_) => return Ok(false),
  };
  
  trees[tree_index] = tree;
  config.save_tree(&trees[tree_index]);

  Ok(true)
}

#[tauri::command]
pub fn get_locations(state: tauri::State<StateData>) -> Vec<String> {
  let trees = match state.0.lock() {
    Ok(trees) => trees,
    Err(_) => return Vec::new(),
  };
  let mut locations = Vec::new();
  for tree in trees.iter() {
    locations.push(tree.get_root().data.name.clone());
  }
  locations
}

#[tauri::command]
pub fn get_tree(location: String, state: tauri::State<StateData>) -> Option<Tree> {
  let trees = match state.0.lock() {
    Ok(trees) => trees,
    Err(_) => return None,
  };
  for tree in trees.iter() {
    if tree.get_root().data.name == location.replace("\\", "/") {
      return Some(tree.clone());
    }
  }
  None
}

#[tauri::command]
pub fn get_files(location: String, state: tauri::State<StateData>) -> Vec<FileData> {
  let trees = match state.0.lock() {
    Ok(trees) => trees,
    Err(_) => return Vec::new(),
  };
  for tree in trees.iter() {
    if tree.is_in_tree(&location) {
      return tree.get_files(&location);
    }
  }
  Vec::new()
}

#[tauri::command]
pub fn get_username() -> String {
  whoami::username()
}