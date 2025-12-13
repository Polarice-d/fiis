use std::{path::PathBuf};
use hound::{WavReader, WavSpec};

pub struct NormalizedWavData {
    pub spec: WavSpec,
    pub samples: Vec<f32>
}

pub fn read_and_normalize_wav(path:PathBuf) -> Result<NormalizedWavData, String> {
    let mut reader = match WavReader::open(&path) {
        Ok(val) => val,
        Err(val) => {return Err(val.to_string());}
    };

    let spec = reader.spec();

    println!("Reading file {:#?}.\n{:#?}", &path, spec);
    
    // TODO: Fix this garbage match statement
    let normalized_samples: Vec<f32> = match spec.bits_per_sample {
        // you have no idea how long it took me to figure out how to turn this into a vector 💀
        // NOTE: 8 bit PCM wav files typically use *unsigned* 8 bit integers
        8 => {let vector_buf:Vec<i8> = reader.samples::<i8>().map(|s| s.unwrap()).collect();
              vector_buf.into_iter().map(|i8_sample| ((i8_sample as i16) - 128) as f32 / i8::MAX as f32).collect()
            },
        16 => {let vector_buf:Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();
              vector_buf.into_iter().map(|i16_sample| i16_sample as f32 / i16::MAX as f32).collect()},

        32 => {match spec.sample_format {
            hound::SampleFormat::Int => {
              let vector_buf:Vec<i32> = reader.samples::<i32>().map(|s| s.unwrap()).collect();
              vector_buf.into_iter().map(|i32_sample| i32_sample as f32 / i32::MAX as f32).collect()
            },
            hound::SampleFormat::Float => {reader.samples::<f32>().map(|s| s.unwrap()).collect()}
        }},
        _ => {return Err("Unsupported format!".to_string());}
    };

    return Ok(NormalizedWavData {
        spec: spec,
        samples: normalized_samples
    });
    
}