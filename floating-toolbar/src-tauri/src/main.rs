#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::Command;
use std::path::PathBuf;
use tauri::api::path::document_dir;
use std::fs;

#[tauri::command]
fn open_external_app(fileFullName:String) {
    let path = PathBuf::from(fileFullName);

    match Command::new(&path).spawn() {
        Ok(_) => println!("Application started successfully."),
        Err(e) => eprintln!("Failed to open application: {}", e),
    }
}

#[tauri::command]
fn run_cmd_file(fileFullName: &str) {
    let path = PathBuf::from(fileFullName);
    match Command::new("cmd")
        .arg("/k")
        .arg(path)
        .spawn()
    {
        Ok(_child) => {}
        Err(e) => eprintln!("Failed to execute command: {}", e),
    }
}

#[tauri::command]
fn read_json_file(fileName: String) -> Result<String, String> {
    let file_path = document_dir().unwrap().join(fileName);
    fs::read_to_string(file_path).map_err(|err| err.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_external_app,run_cmd_file,read_json_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

