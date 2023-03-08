#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod audio;
mod call_py;
use std::{
    self,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use tauri::{Manager, State};

struct SampleData(Arc<RwLock<[f32; audio::SAMPLE_LEN]>>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    // format!("Hello, {}! You've been greeted from Rust!", name)
    // unsafe {
    return format!("Hello, {}! You've been greeted from Rust!", name);
    // }
}
#[tauri::command]
fn get_audio_data(audio_data: State<SampleData>) -> Vec<f32> {
    let reader = audio_data.0.read().unwrap();
    println!("{}", (*reader).len());
    return Vec::from(*reader);
}

#[tauri::command]
fn initialize_audio(sample_data: State<SampleData>) {
    let thread_sample_data = Arc::clone(&sample_data.0);
    thread::spawn(|| {
        audio::rec_audio(thread_sample_data);
    });
}
#[derive(Clone, serde::Serialize)]
struct DataPayload {
    data: Vec<f32>,
}

#[tauri::command]
fn test(window: tauri::Window, audio_data: State<SampleData>) {
    // println!("Window: {}", window.emit<Payload>("get-data", {data: (*audio_data.0.read().unwrap()).to_vec()}))
    // println!("{}", window.)
    let test = (*audio_data.0.read().unwrap()).to_vec();
    window
        .emit_all("get-data", DataPayload { data: test })
        .unwrap();
}

fn main() {
    let sample_data = Arc::new(RwLock::new([0f32; audio::SAMPLE_LEN]));
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(6));
        println!("Shazam recording");
        let sh = call_py::shazam();
        println!("Track: {}", sh.track);
        println!("Artist: {}", sh.artist);
        println!("Cover Art: {}", sh.coverart);
    });
    // let test = *r_data.read().unwrap();
    tauri::Builder::default()
        .manage(SampleData(sample_data))
        .invoke_handler(tauri::generate_handler![
            greet,
            get_audio_data,
            initialize_audio,
            test
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
