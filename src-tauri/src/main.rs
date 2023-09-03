/*
Prevents additional console window on Windows in release, DO NOT REMOVE!!
*/
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod error;
mod event;
mod model;

use std::fs::create_dir;
use std::fs::write;

use api::language::json::JSONLexer;
use logos::Logos;
use model::file_system::generate;
use model::file_system::DiskEntry;
use tauri::api::dir::read_dir;
use tauri::api::path::data_dir;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_disk_entry(path: &str) -> Result<DiskEntry, String> {
    match generate(path) {
        Ok(entry) => Ok(entry),
        Err(error) => {
            println!("{:#?}", error);
            Err(error.to_string())
        }
    }
}

const APP_DATA_FOLDER_NAME: &str = "Workspace";

#[tauri::command]
async fn log_roaming_data(path_dir: &str) -> Result<String, String> {
    let path = data_dir().unwrap().join(APP_DATA_FOLDER_NAME);
    let _ = create_dir(path.clone());
    let _ = write(path.join("test.txt"), "Some text...");

    let mut str = String::from("");
    for entry in read_dir(path_dir, false).unwrap().iter() {
        let x = entry.name.as_ref().unwrap();
        str.push_str(x);
        str.push_str("/");
    }

    Ok(str)
}

fn main() {
    setup();
}

fn setup() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            log_roaming_data,
            get_disk_entry
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
