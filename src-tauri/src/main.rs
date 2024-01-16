// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tree;
mod file;
mod file_update_handler;
mod commands;
mod config;

use std::sync::Mutex;
use config::Config;
use tree::Tree;
use file_update_handler::FileUpdateHandler;
use file::FileSystem;
use tauri::Manager;
use tauri::WindowEvent;

static APP: TauriApp = TauriApp {
  handle: Mutex::new(None),
};

fn main() {
  let mut config = Config::load("../test_config");

  let trees = FileSystem::load_locations(&mut config);
  let event_handler = FileUpdateHandler::inti_locations(&trees.iter().map(|tree| tree.get_root_location()).collect::<Vec<String>>());
  
  tauri::Builder::default()
    .manage(StateData(Mutex::new(trees), Mutex::new(event_handler), Mutex::new(config)))
    .invoke_handler(tauri::generate_handler![commands::search_partial, commands::add_location, commands::remove_location, commands::reindex_location, commands::get_tree, commands::get_locations])
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
      tauri::RunEvent::ExitRequested { .. } => {
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