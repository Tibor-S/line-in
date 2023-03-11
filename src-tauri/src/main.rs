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

use call_py::TrackData;
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

#[derive(Clone, serde::Serialize)]
struct TrackPayload<'a> {
    track: &'a str,
    artist: &'a str,
    coverart: &'a str,
}

#[tauri::command]
fn emit_audio_data(window: tauri::Window, audio_data: State<SampleData>) {
    // println!("Window: {}", window.emit<Payload>("get-data", {data: (*audio_data.0.read().unwrap()).to_vec()}))
    // println!("{}", window.)
    let audio = (*audio_data.0.read().unwrap()).to_vec();
    window
        .emit_all("get-data", DataPayload { data: audio })
        .unwrap();
}

#[tauri::command]
fn emit_track(window: tauri::Window) {
    let track_data = Arc::new(RwLock::new(TrackData {
        track: "n/a".to_string(),
        artist: "n/a".to_string(),
        coverart: "n/a".to_string(),
    }));
    let thread_track_data = Arc::clone(&track_data);
    let handle = thread::spawn(move || {
        let mut track_data = thread_track_data.write().unwrap();
        println!("Shazam recording");
        let sh = call_py::recognize_track();
        println!("Track: {}", sh.track);
        println!("Artist: {}", sh.artist);
        println!("Cover Art: {}", sh.coverart);
        *track_data = sh;
    });
    handle.join().unwrap();
    let shazam = track_data.read().unwrap();
    window
        .emit_all(
            "get-track",
            TrackPayload {
                track: &shazam.track,
                artist: &shazam.artist,
                coverart: &shazam.coverart,
            },
        )
        .unwrap();
}

fn main() {
    let sample_data = Arc::new(RwLock::new([0f32; audio::SAMPLE_LEN]));

    // let test = *r_data.read().unwrap();
    tauri::Builder::default()
        .manage(SampleData(sample_data))
        .invoke_handler(tauri::generate_handler![
            greet,
            get_audio_data,
            initialize_audio,
            emit_audio_data,
            emit_track
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
