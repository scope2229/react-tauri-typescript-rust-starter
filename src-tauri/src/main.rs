#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
use commands::{cmd, invoke, message, resolver};

use serde::Deserialize;
use tauri::{command, State, Window};

#[derive(Debug)]
pub struct AppState {
  #[allow(dead_code)]
  value: u64,
  #[allow(dead_code)]
  label: String,
}

#[derive(Debug, serde::Serialize)]
enum MyError {
  FooError,
}

// ------------------------ Commands using Window ------------------------
#[command]
fn window_label(window: Window) {
  println!("window label: {}", window.label());
}

// Async commands

#[command]
async fn async_simple_command(argument: String) {
  println!("{}", argument);
}

#[command]
async fn async_stateful_command(
  argument: Option<String>,
  state: State<'_, AppState>,
) -> Result<(), ()> {
  println!("{:?} {:?}", argument, state.inner());
  Ok(())
}

// Raw future commands
#[command(async)]
fn future_simple_command(argument: String) -> impl std::future::Future<Output = ()> {
  println!("{}", argument);
  std::future::ready(())
}

#[command(async)]
fn future_simple_command_with_return(
  argument: String,
) -> impl std::future::Future<Output = String> {
  println!("{}", argument);
  std::future::ready(argument)
}

#[command(async)]
fn future_simple_command_with_result(
  argument: String,
) -> impl std::future::Future<Output = Result<String, ()>> {
  println!("{}", argument);
  std::future::ready(Ok(argument))
}

// ------------------------ Commands returning Result ------------------------

#[command]
fn increment(argument: i64) -> Result<i64, MyError> {
  println!("{}", argument);

  (!argument.to_string().is_empty())
    .then(|| argument + 1)
    .ok_or(MyError::FooError)
}

#[command]
fn stateful_command_with_result(
  argument: Option<String>,
  state: State<'_, AppState>,
) -> Result<String, MyError> {
  println!("{:?} {:?}", argument, state.inner());
  dbg!(argument.ok_or(MyError::FooError))
}

// Async commands

#[command]
async fn async_simple_command_with_result(argument: String) -> Result<String, MyError> {
  println!("{}", argument);
  Ok(argument)
}

#[command]
async fn async_stateful_command_with_result(
  argument: Option<String>,
  state: State<'_, AppState>,
) -> Result<String, MyError> {
  println!("{:?} {:?}", argument, state.inner());
  Ok(argument.unwrap_or_else(|| "".to_string()))
}

// Non-Ident command function arguments

#[command]
fn command_arguments_wild(_: Window) {
  println!("we saw the wildcard!")
}

#[derive(Deserialize)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

#[command]
fn command_arguments_struct(Person { name, age }: Person<'_>) {
  println!("received person struct with name: {} | age: {}", name, age)
}

#[derive(Deserialize)]
struct InlinePerson<'a>(&'a str, u8);

#[command]
fn command_arguments_tuple_struct(InlinePerson(name, age): InlinePerson<'_>) {
  println!("received person tuple with name: {} | age: {}", name, age)
}

#[command]
fn decrement(argument: i64) -> Result<i64, MyError> {
  println!("{}", argument);

  (!argument.to_string().is_empty())
    .then(|| argument - 1)
    .ok_or(MyError::FooError)
}

fn main() {
  tauri::Builder::default()
    .manage(AppState {
        value: 0,
        label: "".to_string(),
    })
    .invoke_handler(tauri::generate_handler![
        window_label,
        commands::simple_command,
        commands::stateful_command,
        decrement,
        cmd,
        invoke,
        message,
        resolver,
        async_simple_command,
        future_simple_command,
        async_stateful_command,
        command_arguments_wild,
        command_arguments_struct,
        increment,
        stateful_command_with_result,
        command_arguments_tuple_struct,
        async_simple_command_with_result,
        future_simple_command_with_return,
        future_simple_command_with_result,
        async_stateful_command_with_result,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
