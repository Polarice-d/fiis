use crate::types::AudioBuffer;
use std::{path::PathBuf};
use hound::WavReader;
use hound::SampleFormat::{Float,Int};

pub fn read_and_normalize_wav(path:&PathBuf) -> Result<AudioBuffer, String> {
    let mut reader = match WavReader::open(&path) {
        Ok(val) => val,
        Err(val) => {return Err(val.to_string());}
    };

    let spec = reader.spec();
    println!("Reading file {:#?}", path);
    println!("   Sample rate: {},\n   Bit depth: {},\n   Sample format: {:#?},\n   Channels: {}", spec.sample_rate, spec.bits_per_sample, spec.sample_format, spec.channels);

    let samples: Vec<f64> = match spec.bits_per_sample {
        16 => reader
            .samples::<i16>()
            .map(|s| (s.unwrap() as f64) / i16::MAX as f64)
            .collect(),
        24 => reader
            .samples::<i32>()
            .map(|s| (s.unwrap() as f64) / i16::MAX as f64)
            .collect(),
        32 => match spec.sample_format {
            Int => reader
                .samples::<i32>()
                .map(|s| s.unwrap() as f64 / i32::MAX as f64)
                .collect(),
            Float => reader
                .samples::<f32>()
                .map(|s| s.unwrap() as f64)
                .collect(),
        },
        _ => return Err("Unsupported .wav format".to_string())
    };

    let channel_amount = spec.channels as usize;
    let samples_per_channel = samples.len() / channel_amount;
    let mut result = vec![Vec::with_capacity(samples_per_channel); channel_amount];
    
    for (i, sample) in samples.iter().enumerate() {
        let channel = i % channel_amount;
        result[channel].push(*sample);
    }

    return Ok(AudioBuffer {
        spec,
        channels: result
    });    
}