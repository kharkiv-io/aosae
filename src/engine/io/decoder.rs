use cpal::SampleFormat;
use std::fs::File;
use std::path::Path;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;
#[derive(Debug)]

pub struct METADATA {
    pub sample_format: SampleFormat,
    pub sample_rate: u32,
    pub channels: u16,
}

impl METADATA {
    pub fn clone(&self) -> METADATA {
        todo!()
    }
}

#[derive(Debug)]
pub enum AudioError {
    UnsupportedFormat(u16),
    UnsupportedBitsPerSample(u16),
    IoError(std::io::Error),
    DecoderError(String),
    ProbeError(String),
    NoAudioStream,
}

impl From<std::io::Error> for AudioError {
    fn from(err: std::io::Error) -> Self {
        AudioError::IoError(err)
    }
}

pub fn get_audio_bit_rate<P: AsRef<Path>>(path: P) -> Result<u16, AudioError> {
    let file = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let hint = Hint::new();
    let format_opts = FormatOptions::default();
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &Default::default())
        .map_err(|e| AudioError::ProbeError(e.to_string()))?;
    let track = probed
        .format
        .tracks()
        .get(0)
        .ok_or(AudioError::NoAudioStream)?;
    let bits = track.codec_params.bits_per_sample.unwrap_or(16) as u16;
    match bits {
        8 | 16 | 24 | 32 | 64 | 96 | 128 | 192 => Ok(bits),
        _ => Err(AudioError::UnsupportedBitsPerSample(bits)),
    }
}

pub fn decode_audio_samples<P: AsRef<Path>>(path: P) -> Result<Vec<f32>, AudioError> {
    let file = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let hint = Hint::new();
    let format_opts = FormatOptions::default();
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &Default::default())
        .map_err(|e| AudioError::ProbeError(e.to_string()))?;
    let mut format = probed.format;
    let track = format.tracks().get(0).ok_or(AudioError::NoAudioStream)?;
    let decoder_opts = DecoderOptions::default();
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)
        .map_err(|e| AudioError::DecoderError(e.to_string()))?;
    let mut samples = Vec::new();
    let mut sample_buf = None;
    while let Ok(packet) = format.next_packet() {
        let decoded = decoder
            .decode(&packet)
            .map_err(|e| AudioError::DecoderError(e.to_string()))?;
        if sample_buf.is_none() {
            sample_buf = Some(SampleBuffer::new(
                decoded.capacity() as u64,
                *decoded.spec(),
            ));
        }
        if let Some(buf) = &mut sample_buf {
            buf.copy_interleaved_ref(decoded);
            samples.extend(buf.samples().iter().copied().map(|s| match s {
                s if s > 1.0 => 1.0,
                s if s < -1.0 => -1.0,
                s => s,
            }));
        }
    }

    if samples.is_empty() {
        return Err(AudioError::DecoderError("File is empty!".to_string()));
    }
    Ok(samples)
}

pub fn read_audio_info<P: AsRef<Path>>(path: P) -> Result<METADATA, AudioError> {
    let file = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let hint = Hint::new();
    let format_opts = FormatOptions::default();
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &Default::default())
        .map_err(|e| AudioError::ProbeError(e.to_string()))?;
    let track = probed
        .format
        .tracks()
        .get(0)
        .ok_or(AudioError::NoAudioStream)?;
    let params = &track.codec_params;
    let sample_format = match params.bits_per_sample.unwrap_or(16) {
        8 => SampleFormat::U8,
        16 => SampleFormat::I16,
        24 | 32 => SampleFormat::I32,
        64 => SampleFormat::F64,
        96 | 128 | 192 => SampleFormat::F64,
        bits => return Err(AudioError::UnsupportedBitsPerSample(bits as u16)),
    };

    Ok(METADATA {
        sample_format,
        sample_rate: params.sample_rate.unwrap_or(44100),
        channels: params.channels.map(|c| c.count()).unwrap_or(2) as u16,
    })
}
