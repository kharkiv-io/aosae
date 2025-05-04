use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn record_audio_samples(duration: Duration) -> Result<Vec<f32>> {
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or_else(|| anyhow::anyhow!("No microphone on any input device found?"))?;
    let config = device.default_input_config()?;
    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;
    let samples = Arc::new(Mutex::new(Vec::new()));
    let samples_clone = Arc::clone(&samples);
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &_| {
            let mut samples = samples_clone.lock().unwrap();
            samples.extend_from_slice(data);
        },
        move |err| eprintln!("Stream died? Why? -> {}", err),
        None,
    )?;
    stream.play()?;
    std::thread::sleep(duration);
    stream.pause()?;
    let recorded_samples = samples.lock().unwrap().clone();
    Ok(recorded_samples)
}