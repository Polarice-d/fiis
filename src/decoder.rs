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

    let normalized_samples: Vec<f32> = match spec.bits_per_sample {
        16 => reader
            .samples::<i16>()
            .map(|s| (s.unwrap() as f32) / i16::MAX as f32)
            .collect(),
        24 => reader
            .samples::<i32>()
            .map(|s| (s.unwrap() as f32) / i16::MAX as f32)
            .collect(),
        32 => match spec.sample_format {
            Int => reader
                .samples::<i32>()
                .map(|s| s.unwrap() as f32 / i32::MAX as f32)
                .collect(),
            Float => reader
                .samples::<f32>()
                .map(|s| s.unwrap())
                .collect(),
        },
        _ => return Err("Unsupported .wav format".to_string())
    };

    return Ok(AudioBuffer {
        spec,
        samples: normalized_samples
    });    
}