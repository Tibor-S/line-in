extern crate bindgen;

use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    tauri_build::build();
}
