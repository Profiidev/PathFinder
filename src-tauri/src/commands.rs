use uuid::Uuid;
use crate::tree::{TreeSearchType, Tree};
use crate::file::{File, FileSystem};
use crate::StateData;

#[tauri::command]
pub fn search_partial(name: String, path: String, search_type: String, is_dir: bool, state: tauri::State<StateData>) -> Vec<File> {
  let mut found = Vec::new();
  let path = path.replace("\\", "/");

  let search_type_enum = match search_type.as_str() {
    "exact" => TreeSearchType::Exact,
    "contains" => TreeSearchType::Contains,
    "starts_with" => TreeSearchType::StartsWith,
    "exact_no_type" => TreeSearchType::ExactNoType,
    "contains_no_type" => TreeSearchType::ContainsNoType,
    "starts_with_no_type" => TreeSearchType::StartsWithNoType,
    _ => return found,
  };

  let trees = state.0.lock().unwrap();
  for tree in trees.iter() {
    if !tree.is_in_tree(&path) {
      continue;
    }

    found = tree.search_partial(&path, &File {
      name: name.clone(),
      is_dir: is_dir
    }, &search_type_enum);

    if found.len() > 0 {
      break;
    }
  }

  found
}

#[tauri::command]
pub fn add_location(location: String, state: tauri::State<StateData>) -> bool {
  let mut trees = match state.0.lock() {
    Ok(trees) => trees,
    Err(_) => return false,
  };
  let mut config = match state.2.lock() {
    Ok(config) => config,
    Err(_) => return false,
  };
  if trees.iter().any(|tree| tree.get_root().data.name == location.replace("\\", "/")) {
      return false;
  }

  let tree = FileSystem::gen_tree(&location, Uuid::new_v4().to_string());
  if tree.id == "error" {
    return false;
  }

  config.create_tree(&tree);
  trees.push(tree);
  true
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
pub fn reindex_location(location: String, state: tauri::State<StateData>) -> bool {
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
  if index >= trees.len() {
    return false;
  }

  trees[index] = FileSystem::gen_tree(&location, trees[index].id.clone());
  true
}

#[tauri::command]
pub fn get_tree(location: String, state: tauri::State<StateData>) -> Option<Tree> {
  let trees = state.0.lock().unwrap();
  for tree in trees.iter() {
    if tree.get_root().data.name == location.replace("\\", "/") {
      return Some(tree.clone());
    }
  }
  None
}