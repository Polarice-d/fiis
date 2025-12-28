use std::collections::HashMap;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use crate::types::{AudioBuffer, AudioEffect};
use crate::parse_utils::verify_range;

pub struct Delay;

const COMMAND_NAME: &str = "delay";
const MIX_NAME: &str = "mix";
const FEEDBACK_NAME: &str = "fb";
const TIME_NAME: &str = "time";
const MIN_DELAY_ENERGY: f64 = 0.0001; // This is equivalent to -80 dB in energy

impl AudioEffect for Delay {

    fn get_name(&self) -> String { COMMAND_NAME.to_string() }

    fn validate_arguments(&self, arguments: &HashMap<String, f64>, tail_length: &Option<f64>) -> Result<(), String> {
        verify_range(&MIX_NAME.to_string(), 0.0, 100.0, arguments)?;
        verify_range(&TIME_NAME.to_string(), 1.0, 1000000.0, arguments)?;
        let feedback = verify_range(&FEEDBACK_NAME.to_string(), 0.0, 100.0, arguments)?;

        if feedback >= 1.0 && tail_length.is_none() {
            return Err("Tail length (--tail, -t) is required for delay feedback >= 1 to avoid infinite feedback cycles".to_string());
        }

        if feedback < 1.0 && feedback > 0.9 && tail_length.is_none() {
            println!("Warning, delay feedback level ~1 was requested! Your file may be quite large");
        }

        Ok(())
    }

    fn apply_effect(&self, audio_buffer: &mut AudioBuffer, arguments: &HashMap<String,f64>, tail_length: &Option<f64>) -> Result<(), String> {
        let mix = arguments.get(MIX_NAME).unwrap();
        let feedback = arguments.get(FEEDBACK_NAME).unwrap();
        let time = arguments.get(TIME_NAME).unwrap();
        let sample_rate = audio_buffer.spec.sample_rate as i32;
        
        for channel in audio_buffer.channels.iter_mut() {
            let ringbuffer_size = ((time / 1000.0) * sample_rate as f64) as usize;
            let mut buffer: AllocRingBuffer<f64> = AllocRingBuffer::new(ringbuffer_size);
        
            buffer.enqueue(0.0);
        
            for sample in channel.iter_mut() {
                let delayed = *buffer.front().unwrap();
                buffer.enqueue(*sample + delayed * feedback);
                *sample += delayed * mix;
            }
        
            let mut square_sum= buffer.iter().map(|val| val * val ).sum::<f64>();
            let normalizing_factor = (1.0/ringbuffer_size as f64).sqrt();
        
            if tail_length.is_some() {
                let tail_samples = audio_buffer.spec.channels as i32 * tail_length.unwrap() as i32 * sample_rate;
                for _ in 0 .. tail_samples {
                    buffer.enqueue(buffer.front().unwrap() * feedback);
                    channel.push((buffer.front().unwrap() * mix).clamp(-1.0, 1.0));
                }
            } else {
                while normalizing_factor * square_sum.sqrt() > MIN_DELAY_ENERGY { 
                    let front_val = *buffer.front().unwrap();
                    square_sum -= front_val * front_val;
                    buffer.enqueue(front_val * feedback);

                    let back_val = buffer.back().unwrap();
                    square_sum += back_val * back_val;

                    channel.push((buffer.front().unwrap() * mix).clamp(-1.0, 1.0));
                }
            }
        }

        Ok(())
    } 
}