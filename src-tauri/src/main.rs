// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tree;
mod file;
mod file_update_handler;
mod commands;
mod config;

use std::sync::Mutex;
use commands::get_username;
use config::Config;
use tree::Tree;
use file_update_handler::FileUpdateHandler;
use file::FileSystem;
use tauri::{WindowEvent, Manager};
use uuid::Uuid;

static APP: TauriApp = TauriApp {
  handle: Mutex::new(None),
};
static SEARCH_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
  let mut config = Config::new(&("C:/Users/".to_string() + &get_username().to_string() + "/AppData/Roaming/PathFinder/"));

  let trees = FileSystem::load_locations(&mut config);
  let event_handler = FileUpdateHandler::inti_locations(&trees.iter().map(|tree| tree.get_root_location()).collect::<Vec<String>>());
  
  tauri::Builder::default()
    .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
      let handle = match APP.handle.lock() {
        Ok(handle) => match handle.as_ref() {
          Some(handle) => handle.clone(),
          None => return,
        },
        Err(_) => return,
      };
      
      tauri::WindowBuilder::new(
        &handle, 
        Uuid::new_v4().to_string(),
        tauri::WindowUrl::App("index.html".into())
      )
      .title(&app.package_info().name)
      .inner_size(1200f64, 675f64)
      .build().unwrap();
    }))
    .manage(StateData(Mutex::new(trees), Mutex::new(event_handler), Mutex::new(config)))
    .invoke_handler(tauri::generate_handler![
      commands::search_partial, 
      commands::add_location, 
      commands::remove_location, 
      commands::reindex_location, 
      commands::get_tree, 
      commands::get_locations, 
      commands::get_settings, 
      commands::save_settings,
      commands::get_files,
      commands::get_username,
    ])
    .setup(|app| {
      let handle = app.handle();
      APP.handle.lock().unwrap().replace(handle);
      Ok(())
    })
    .on_window_event(|e| {
      if let WindowEvent::Resized(_) = e.event() {
          std::thread::sleep(std::time::Duration::from_nanos(1));
      }
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|_app_handle, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => {
        let handle = APP.handle.lock().unwrap();
        let state = match handle.as_ref() {
          Some(handle) => {
            handle.state::<StateData>()
          },
          None => return,
        };
        let mut trees = state.0.lock().unwrap();
        let mut config = state.2.lock().unwrap();
        config.save(&mut *trees);
        api.prevent_exit();
      }
      _ => {}
    });
}

pub struct StateData(Mutex<Vec<Tree>>, Mutex<FileUpdateHandler>, Mutex<Config>);

pub struct TauriApp {
  pub handle: Mutex<Option<tauri::AppHandle>>,
}

impl Drop for TauriApp {
  fn drop(&mut self) {
    println!("Dropping app handle");
  }
}