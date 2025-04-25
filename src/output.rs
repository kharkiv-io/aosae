use std::io::{Error, ErrorKind};
use anyhow::Result;
use cpal;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};
use crate::decoder::METADATA;

pub fn play_samples(samples: Vec<f32>, duration: Duration, wav_info: METADATA, stereo: bool) -> Result<()> {
    let host = cpal::default_host();
    let device = host.default_output_device()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Can't find default output device!"))?;
    let config = StreamConfig {
        channels: wav_info.channels,
        sample_rate: cpal::SampleRate(wav_info.sample_rate),
        buffer_size: cpal::BufferSize::Default,
    };
    let playing = Arc::new(AtomicBool::new(true));
    let playing_clone = playing.clone();
    let samples = Arc::new(samples);
    let samples_clone = samples.clone();
    let mut sample_position = 0.0;
    let stereo_ = if stereo { 2.0 } else { 1.0 };
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(wav_info.channels as usize) {
                if sample_position >= samples_clone.len() as f32 {
                    playing_clone.store(false, Ordering::SeqCst);
                    break;
                }
                let current_index = sample_position as usize;
                for (channel, sample) in frame.iter_mut().enumerate() {
                    *sample = if channel < wav_info.channels as usize {
                        samples_clone[current_index]
                    } else {
                        0.0
                    };
                }
                sample_position += stereo_;
            }
        },
        move |err| {
            eprintln!("Audio stream confused due to -> {}", err);
        },
        None,
    ).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
    stream.play().map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
    if duration.is_zero() {
        while playing.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(100));
        }
    } else {
        thread::sleep(duration);
    }
    Ok(())
}