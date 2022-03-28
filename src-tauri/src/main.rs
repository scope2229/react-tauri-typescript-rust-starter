#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{command};

#[derive(Debug, serde::Serialize)]
enum AppError {
  EmptyArgError,
}

#[command]
fn increment(argument: i64) -> Result<i64, AppError> {
  println!("{}", argument);

  (!argument.to_string().is_empty())
    .then(|| argument + 1)
    .ok_or(AppError::EmptyArgError)
}

#[command]
fn decrement(argument: i64) -> Result<i64, AppError> {
  println!("{}", argument);

  (!argument.to_string().is_empty())
    .then(|| argument - 1)
    .ok_or(AppError::EmptyArgError)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        decrement,
        increment,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
