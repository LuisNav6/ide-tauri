// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Write; // Importa el trait Write
use std::sync::Mutex;
use crate::globals::TokenType;
use crate::globals::StateType;
use crate::globals::NodeType;

mod globals;
mod scan;
mod parse;

impl TreeNode {
    fn new(node_type: NodeType) -> Self {
        TreeNode {
            node_type,
            token: None,
            value: None,
            children: Vec::new(),
        }
    }
}

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ERRORS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

#[tauri::command]
fn lexic(content: String) -> Result<(Vec<(TokenType, String, usize, usize)>, Vec<(TokenType, String, usize, usize)>), String> {
    Ok(scan::get_token(&content))
}

#[tauri::command]
fn parse(tokens: Vec<(TokenType, String, usize, usize)>) -> Result<(TreeNode, Vec<String>), String> {
    let mut errors_str: Vec<String> = Vec::new();
    let mut current_token = 0;
    let syntax_tree = match parse::parse_program(&tokens, &mut current_token, &mut errors_str) {
        Ok(tree) => tree,
        Err(err) => {
            parse::log_error(err.to_string());
            TreeNode::new(NodeType::Error)
        }
    };
    let errors_from_global = {
        let mut errors = ERRORS.lock().unwrap();
        let errors_clone = errors.clone();
        errors.clear(); // Clear errors after retrieval
        errors_clone
    };

    // Combine local and global errors
    errors_str.extend(errors_from_global.iter().cloned());
    Ok((syntax_tree, errors_str))
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_file, remove_file, lexic, parse])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn save_file(path: String, contents: String) -> Result<(), String> {
    match save_file_or_save_as(&path, &contents) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Error al guardar el archivo: {}", e)),
    }
}

#[tauri::command]
async fn remove_file(path: String) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

fn save_file_or_save_as(path: &str, contents: &str) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
  }
