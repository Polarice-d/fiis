use std::{i16, i32, path::PathBuf};

use hound::{WavWriter, SampleFormat};
use crate::types::AudioBuffer;

pub fn encode_file(buffer:AudioBuffer, filename:PathBuf) {
    let mut writer = WavWriter::create(filename, buffer.spec).unwrap();
    let samples_per_channel = buffer.channels[0].len();
    let channel_amount = buffer.spec.channels as usize;
    let mut interleaved_buf = Vec::with_capacity(samples_per_channel * channel_amount);

    for i in 0..samples_per_channel {
        for channel in 0..channel_amount {
            interleaved_buf.push(buffer.channels[channel][i]);
        }
    }

    match buffer.spec.bits_per_sample {
       16 => {
        let amplitude = i16::MAX as f64;
        for sample in interleaved_buf {
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
       },
       24 => { // Todo: add proper 24 bit support cause this doesn't work. You might need some bit manipulation
        let amplitude = 8388607.0;
        for sample in interleaved_buf {
            writer.write_sample((sample * amplitude) as i32).unwrap();
        }
       },
       32 => {
        if buffer.spec.sample_format == SampleFormat::Int {
            let amplitude = i32::MAX as f64;
            for sample in interleaved_buf {
                writer.write_sample((sample * amplitude) as i32).unwrap()
            }
        } else {
            for sample in interleaved_buf {
                writer.write_sample(sample as f32).unwrap()
            }
        }
       }
       _ => {
            panic!("cannot encode unsupported format")
       }
    }

    writer.finalize().unwrap();

}

