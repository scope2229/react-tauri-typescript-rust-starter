## react-tauri-typescript-rust-starter

## Initialize

* Requirements
Rust 1.57 or higher
Cargo 1.59
node v16.14.0

## Start the applciation

`yarn tauri dev`

This starts react-scripts on port 3006 and then connects tauri to that port.

## To build

`yarn tauri build`

Builds are sent to `src-tauri/target/release/[appName]`

## ToDo

* Use ESBuild import maps to handle node packages
* Separate commands from `main.rs` to own file `cmds.rs` and init inside `main.rs`
* Add SASS to frontend
* Add tailwindCSS support
* Add persistent state storage using rust
