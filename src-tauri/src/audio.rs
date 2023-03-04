use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

pub fn ply_audio(f: Arc<Mutex<dyn FnMut(f32) -> f32 + Send + Sync>>) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device found");
    let config = device.default_output_config().unwrap();
    let mut clock = 0f32;
    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for frame in data.iter_mut() {
                    *frame = f.lock().unwrap()(clock);
                    clock += 1f32;
                }
            },
            |_| {},
        )
        .unwrap();
    stream.play();
    loop {
        thread::sleep(Duration::from_secs(5))
    }
}
