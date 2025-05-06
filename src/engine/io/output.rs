use cpal;
use cpal::{
    StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use std::thread;
use std::time::Duration;

use cpal::{Device, FromSample, SampleFormat, SizedSample};
use fundsp::hacker::wave::Wave;
use fundsp::hacker::wavech;
use fundsp::prelude::AudioUnit;
use std::sync::Arc;

pub fn create_samples_box(samples: Vec<f32>) -> Box<dyn AudioUnit> {
    let (left_samples, right_samples): (Vec<f32>, Vec<f32>) =
        samples.chunks(2).map(|chunk| (chunk[0], chunk[1])).unzip();

    let left_wave = Arc::new(Wave::from_samples(48000.0, &left_samples));
    let right_wave = Arc::new(Wave::from_samples(48000.0, &right_samples));

    let left_channel = wavech(&left_wave.clone(), 0, None);
    let right_channel = wavech(&right_wave.clone(), 0, None);
    let synth = left_channel | right_channel;

    Box::new(synth)
}

pub fn play_samples_graph(audio_graph: Box<dyn AudioUnit>) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => run_synth::<f32>(audio_graph, device, config.into()),
        SampleFormat::I16 => run_synth::<i16>(audio_graph, device, config.into()),
        SampleFormat::U16 => run_synth::<u16>(audio_graph, device, config.into()),
        SampleFormat::I32 => run_synth::<i32>(audio_graph, device, config.into()),
        SampleFormat::U32 => run_synth::<u32>(audio_graph, device, config.into()),
        SampleFormat::F64 => run_synth::<f64>(audio_graph, device, config.into()),
        SampleFormat::U8 => run_synth::<u8>(audio_graph, device, config.into()),
        SampleFormat::I8 => run_synth::<i8>(audio_graph, device, config.into()),
        _ => panic!("Unsupported samples format?"),
    }
}

pub fn run_synth<T: SizedSample + FromSample<f64>>(
    mut audio_graph: Box<dyn AudioUnit>,
    device: Device,
    config: StreamConfig,
) {
    thread::spawn(move || {
        let sample_rate = config.sample_rate.0 as f64;
        audio_graph.set_sample_rate(sample_rate);
        let mut g_stereo = move || -> (f64, f64) {
            let g_stereo_f32 = audio_graph.get_stereo();
            (g_stereo_f32.0 as f64, g_stereo_f32.1 as f64)
        };
        let mut next_value = move || g_stereo();
        let channels = config.channels as usize;
        println!("Channels count -> {}", channels);
        let err_fn = |err| eprintln!("An error occurred on stream -> {err}");
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_data(data, channels, &mut next_value)
                },
                err_fn,
                None,
            )
            .unwrap();

        stream.play().unwrap();
        loop {
            thread::sleep(Duration::from_millis(1));
        }
    });
}

pub fn write_data<T: SizedSample + FromSample<f64>>(
    output: &mut [T],
    channels: usize,
    next_sample: &mut dyn FnMut() -> (f64, f64),
) {
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left: T = T::from_sample(sample.0);
        let right: T = T::from_sample(sample.1);

        for (channel, sample) in frame.iter_mut().enumerate() {
            *sample = if channel & 1 == 0 { left } else { right };
        }
    }
}
