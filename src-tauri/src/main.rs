/*
Prevents additional console window on Windows in release, DO NOT REMOVE!!
*/
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod error;
mod event;
mod model;

use serde_json::json;
use std::fs::create_dir;
use std::fs::write;

use tauri::api::dir::read_dir;
use tauri::api::path::data_dir;

use crate::api::fs::get_disk_entry_from_path;
use crate::api::grammar::json_to_grammar::json_to_grammar;

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
    let grammar = json_to_grammar(json!(
    {
        "grammarName": "Test",
        "tokenDefinitions": {
            "number": {
                "regex": "[0-9]+"
            },
            "space": {
                "regex": "[ ]+"
            }
        },
        "fragments": {
            "number": {
                "rule": {
                    "type" : "token",
                    "value" : 0
                }
            },
            "numberSpace": {
                "rule": {
                    "type": "sequence",
                    "values": [
                        {
                            "type": "fragment",
                            "value": 0
                        },
                        {
                            "type": "fragment",
                            "value": 3
                        }
                    ]
                }
            },
            "numberSpaceOrNumber": {
                "rule": {
                    "type": "or",
                    "values": [
                        {
                            "type": "fragment",
                            "value": 1
                        },
                        {
                            "type": "fragment",
                            "value": 0
                        }
                    ]
                }
            },
            "space": {
                    "rule": {
                        "type" : "token",
                        "value" : 1
                    }
                },
        }
    }))
    .unwrap();

    println!("{:?}", grammar);

    let result = grammar.parse("12 21");
    println!("{:?}", result);
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
