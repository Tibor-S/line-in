use crate::utils::{log, tmp_sample_file};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    InputCallbackInfo, SampleFormat, StreamError, SupportedStreamConfig,
};
use hound::WavSpec;
use std::{
    sync::{mpsc::channel, Arc, RwLock},
    thread,
    time::Duration,
};

pub const SAMPLE_LEN: usize = 2048;
pub fn rec_audio(sample_data: Arc<RwLock<[f32; SAMPLE_LEN]>>, sample_stream: Arc<RwLock<bool>>) {
    // Load env variables
    let sample_path = tmp_sample_file();
    log(
        "audio",
        "rec_audio",
        &format!("sample_path is {}", sample_path),
    );

    // Define essentials
    let host = cpal::default_host();
    let device = host.default_input_device().unwrap();
    let config = device.default_input_config().unwrap();

    // For audio sampling
    let spec = cpal_to_hound(config.clone());
    let (tx, rx) = channel::<f32>();

    // Thread variables
    let thread_sample_stream = Arc::clone(&sample_stream);
    let thread_sample_data = Arc::clone(&sample_data);

    // Panic if unsupported
    match config.sample_format() {
        SampleFormat::F32 => {}
        _ => panic!("Unsupported format"),
    };

    // Stream closures
    let data_callback = move |data: &[f32], _: &InputCallbackInfo| {
        let mut new_sample_data = thread_sample_data.write().unwrap();
        let sample_stream = thread_sample_stream.read().unwrap();
        (*new_sample_data).rotate_left(data.len() % SAMPLE_LEN);
        for i in 0..data.len() {
            let nsai = (SAMPLE_LEN as i32) - (data.len() as i32) + (i as i32) - 1i32;
            if nsai >= 0i32 {
                let ui = nsai as usize;
                (*new_sample_data)[ui] = data[i];
            }
            if *sample_stream {
                match tx.send(data[i]) {
                    Ok(_) => (),
                    Err(e) => println!("No destination for data {}", e),
                };
            }
        }
    };

    let error_callback = |e: StreamError| {
        panic!("{}", e);
    };

    // Stream thread
    thread::spawn(move || {
        let stream = device
            .build_input_stream(&config.into(), data_callback, error_callback)
            .unwrap();
        stream.play().expect("error with stream");
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Audio sampling process
    loop {
        if *sample_stream.read().unwrap() {
            let mut writer = hound::WavWriter::create(sample_path.clone(), spec).unwrap();
            while writer.duration() as f32 / spec.sample_rate as f32 <= 5f32 {
                let data = rx.recv().unwrap();
                writer.write_sample(data).unwrap();
            }
            log("audio", "rec_audio", "Saved recording in tmp file");
            writer.finalize().unwrap();
            *sample_stream.write().unwrap() = false;
        }
    }
}

fn cpal_to_hound(config: SupportedStreamConfig) -> WavSpec {
    let channels = config.channels();
    let sample_rate = config.sample_rate().0;
    let bits_per_sample = (config.sample_format().sample_size() * 8) as u16;
    let sample_format = match config.sample_format() {
        // cpal::SampleFormat::I16 => hound::SampleFormat::Int,
        cpal::SampleFormat::F32 => hound::SampleFormat::Float,
        _ => panic!("Unsupported hound sample format!"),
    };
    WavSpec {
        channels,
        sample_rate,
        bits_per_sample,
        sample_format,
    }
}
