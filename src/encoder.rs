use std::{i16, i32, path::PathBuf};

use hound::{WavWriter, SampleFormat};
use crate::types::{AudioBuffer, I24_MAX};

pub fn encode_file(buffer: &AudioBuffer, filename:PathBuf) -> Result<i32, hound::Error> {
    let mut writer = WavWriter::create(filename, buffer.spec)?;
    let samples_per_channel = buffer.channels[0].len();
    let channel_amount = buffer.spec.channels as usize;
    let mut interleaved_buf = Vec::with_capacity(samples_per_channel * channel_amount);
    let mut clip_count = 0;

    for i in 0..samples_per_channel {
        for channel in 0..channel_amount {
            interleaved_buf.push(buffer.channels[channel][i]);
        }
    }

    for sample in interleaved_buf.iter_mut() {
        if sample.abs() > 1.0 {
            clip_count += 1;
        }
        *sample = sample.clamp(-1.0, 1.0);
    }

    match buffer.spec.bits_per_sample {
       16 => {
        let amplitude = i16::MAX as f64;
        for sample in interleaved_buf {
            writer.write_sample((sample * amplitude) as i16)?;
        }
       },
       24 => {
        let amplitude = I24_MAX as f64;
        for sample in interleaved_buf {
            writer.write_sample((sample * amplitude) as i32)?;
        }
       },
       32 => {
        if buffer.spec.sample_format == SampleFormat::Int {
            let amplitude = i32::MAX as f64;
            for sample in interleaved_buf {
                writer.write_sample((sample * amplitude) as i32)?
            }
        } else {
            for sample in interleaved_buf {
                writer.write_sample(sample as f32)?
            }
        }
       }, 
       
       _ => {
            return Err(hound::Error::Unsupported);
       }
    }

    writer.finalize()?;

    Ok(clip_count)

}

