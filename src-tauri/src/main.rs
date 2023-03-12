#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod audio;
mod call_py;
mod utils;
use call_py::TrackData;
use dotenv::dotenv;
use std::{
    self,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};
use tauri::{Manager, State};
use utils::log;

// States
struct SampleDataState(Arc<RwLock<[f32; audio::SAMPLE_LEN]>>);
struct SampleStreamState(Arc<RwLock<bool>>);

// Payloads
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

// Tauri commands
#[tauri::command]
fn initialize_audio(sample_data: State<SampleDataState>, sample_stream: State<SampleStreamState>) {
    let thread_sample_data = Arc::clone(&sample_data.0);
    let thread_sample_stream = Arc::clone(&sample_stream.0);
    thread::spawn(|| {
        audio::rec_audio(thread_sample_data, thread_sample_stream);
    });
}

#[tauri::command]
fn emit_audio_data(window: tauri::Window, audio_data: State<SampleDataState>) {
    // println!("Window: {}", window.emit<Payload>("get-data", {data: (*audio_data.0.read().unwrap()).to_vec()}))
    // println!("{}", window.)
    let audio = (*audio_data.0.read().unwrap()).to_vec();
    window
        .emit_all("get-data", DataPayload { data: audio })
        .unwrap();
}

#[tauri::command]
fn emit_track(window: tauri::Window, sample_stream_state: State<SampleStreamState>) {
    let thread_sample_stream = Arc::clone(&sample_stream_state.0);
    let window_handle = window.app_handle();

    thread::spawn(move || {
        *thread_sample_stream.write().unwrap() = true;
        log("main", "emit_track", "set sample_stream to true");
        while *thread_sample_stream.read().unwrap() == true {}
        log("main", "emit_track", "sample_stream is false");

        log("main", "emit_track", "Recognizing file");
        let sh = call_py::recognize_track();
        log("main", "emit_track", &format!("Track is {}", sh.track));
        log("main", "emit_track", &format!("Artist is {}", sh.artist));
        log(
            "main",
            "emit_track",
            &format!("Cover art is {}", sh.coverart),
        );

        window_handle.emit_all(
            "get-track",
            TrackPayload {
                track: &sh.track,
                artist: &sh.artist,
                coverart: &sh.coverart,
            },
        )
    });
}

fn main() {
    match dotenv() {
        Ok(_) => log("main", "main", "Loaded env variables"),
        Err(e) => {
            log("main", "main", "Could not load env variables");
            panic!("{}", e)
        }
    }

    let sample_data = [0f32; audio::SAMPLE_LEN];
    let sample_stream = false;

    // let test = *r_data.read().unwrap();
    tauri::Builder::default()
        .manage(SampleDataState(Arc::new(RwLock::new(sample_data))))
        .manage(SampleStreamState(Arc::new(RwLock::new(sample_stream))))
        .invoke_handler(tauri::generate_handler![
            initialize_audio,
            emit_audio_data,
            emit_track
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
