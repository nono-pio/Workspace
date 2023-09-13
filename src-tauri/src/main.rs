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

use tauri::api::dir::read_dir;
use tauri::api::path::data_dir;

use crate::api::fs::get_disk_entry_from_path;
use crate::api::grammar::fragment::Fragment;
use crate::api::grammar::grammar::Grammar;
use crate::api::grammar::rules::{FragmentRule, SequenceRule, TokenRule};
use crate::api::grammar::token::TokenDefinition;

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
    // setup();
    let grammar = Grammar::new()
        .add_token_definition(TokenDefinition::new_regex("number", "^\\d+"))
        .add_token_definition(TokenDefinition::new_keyword("test", "test"))
        .add_token_definition(TokenDefinition::new_regex("text", "^[a-zA-Z]+"))
        .add_token_definition(TokenDefinition::new_keyword("space", " "))
        .add_fragment(Fragment::new("main fragment", 0))
        .add_fragment(Fragment::new("test fragment", 1))
        .add_fragment(Fragment::new("", 2))
        .add_rule(Box::new(SequenceRule(vec![
            Box::new(TokenRule(0)), // number
            Box::new(TokenRule(3)), // space
            Box::new(TokenRule(0)), // number
        ]))) // rule main
        .add_rule(Box::new(TokenRule(0))) // rule test
        .add_rule(Box::new(TokenRule(0))); //
    println!("{:?}", grammar.parse("12 21"));
}

fn setup() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            log_roaming_data,
            get_disk_entry_from_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
