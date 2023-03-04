#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod audio;
use std::sync::RwLock;
use std::{
    self,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    // format!("Hello, {}! You've been greeted from Rust!", name)
    // unsafe {
    return format!("Hello, {}! You've been greeted from Rust!", name);
    // }
}

struct Amplitude {
    amp: Arc<Mutex<f32>>,
}

// impl Amplitude {
//     pub fn new(amp: f32) -> Amplitude {
//         Amplitude {
//             amp: ,
//         }
//     }

//     pub fn get_arc(self) -> Arc<Mutex<f32>> {
//         Arc::clone(&(self.amp))
//     }

//     pub fn set(self, na: f32) {
//         let thread_a = self.get_arc();
//         thread::spawn(move || {
//             let mut amp = thread_a.lock().unwrap();
//             *amp = na
//         });
//     }
// }

fn main() {
    // std::thread::spawn();

    let val = Arc::new(RwLock::new(0f32));
    let c_val = Arc::clone(&val);
    let f = Arc::new(Mutex::new(move |t: f32| (t / 100f32).sin()));
    thread::spawn(move || {
        let mut i = 0f32;
        loop {
            i += 0.001;
            let new_val = 0.1 * i.sin();
            let mut val_writer = c_val.write().unwrap();
            *val_writer = new_val;
            // thread::sleep(Duration::from_micros(100))
        }
    });

    audio::ply_audio(f);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn simple_sine(t: f32) -> f32 {
    (t / 100f32).sin()
}
