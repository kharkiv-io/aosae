use anyhow::Result;
use hound::{WavWriter, WavSpec};
use crate::engine::io::decoder::METADATA;

pub fn samples_to_wav(samples: Vec<f32>, bps: u16, metadata: METADATA, filename: &str) -> Result<()> {
    let spec = WavSpec {
        channels: metadata.channels,
        sample_rate: metadata.sample_rate,
        bits_per_sample: bps,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = WavWriter::create(filename, spec)
        .map_err(|e| anyhow::anyhow!("Failed to create a wav file due to -> {}", e))?;
    for sample in samples {
        writer.write_sample(sample)
            .map_err(|e| anyhow::anyhow!("Failure while writing sample -> {}", e))?;
    }
    writer.finalize()
        .map_err(|e| anyhow::anyhow!("Can't finalize WAV -> {}", e))?;
    Ok(())
}
