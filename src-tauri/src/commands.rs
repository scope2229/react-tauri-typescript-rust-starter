use tauri::{command, State};

#[command]
pub fn cmd(_argument: String) {}

#[command]
pub fn invoke(_argument: String) {}

#[command]
pub fn message(_argument: String) {}

#[command]
pub fn resolver(_argument: String) {}

#[command]
pub fn simple_command(argument: String) {
  println!("COMMAND RUNNING ");
  println!("{}", argument);
}

#[command]
pub fn stateful_command(argument: Option<String>, state: State<'_, super::AppState>) {
  println!("{:?} {:?}", argument, state.inner());
}

