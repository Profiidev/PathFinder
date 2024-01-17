use uuid::Uuid;
use crate::tree::{TreeSearchType, Tree};
use crate::file::{File, FileSystem, FileData};
use crate::StateData;
use crate::FileUpdateHandler;

#[tauri::command]
pub fn save_settings(settings: String, state: tauri::State<StateData>) -> bool {
  let mut config = match state.2.lock() {
    Ok(config) => config,
    Err(_) => return false,
  };

  config.settings.settings = settings;
  
  let config_path = config.path.clone() + "/settings.json";
  config.settings.save(&(config_path));
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
pub async fn search_partial(name: String, path: String, search_type: String, is_dir: bool, index_start: i32, index_end: i32, state: tauri::State<'_,StateData>) -> Result<Vec<FileData>, ()> {
  let mut found = Vec::new();
  let path = path.replace("\\", "/");

  let search_type_enum = match search_type.as_str() {
    "exact" => TreeSearchType::Exact,
    "contains" => TreeSearchType::Contains,
    "starts_with" => TreeSearchType::StartsWith,
    "exact_no_type" => TreeSearchType::ExactNoType,
    "contains_no_type" => TreeSearchType::ContainsNoType,
    "starts_with_no_type" => TreeSearchType::StartsWithNoType,
    _ => return Ok(found),
  };

  let trees = match state.0.lock() {
    Ok(trees) => trees,
    Err(_) => return Ok(found),
  };
  for tree in trees.iter() {
    if !tree.is_in_tree(&path) {
      continue;
    }

    found = tree.search_partial(&path, &File {
      name: name.clone(),
      is_dir: is_dir,
      size: 0,
    }, &search_type_enum);

    if found.len() > 0 {
      break;
    }
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
pub async fn add_location(location: String, state: tauri::State<'_,StateData>) -> Result<bool, ()> {
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

  Ok(true)
}

#[tauri::command]
pub fn remove_location(location: String, state: tauri::State<StateData>) -> bool {
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