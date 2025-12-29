use std::collections::HashMap;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use crate::types::{AudioBuffer, AudioEffect};
use crate::parse_utils::verify_min;

pub struct Delay;

const MIN_DELAY_ENERGY: f64 = 0.0001; // This is equivalent to -80 dB in energy
const MAX_TAIL_LENGTH_SECONDS: i32 = 3600;
                                      
impl Delay {
    const NAME: &str = "delay";
    const WET_ARG: &str = "wet";
    const FEEDBACK_ARG: &str = "fb";
    const TIME_ARG: &str = "time";
}

impl AudioEffect for Delay {

    fn get_name(&self) -> String { Delay::NAME.to_string() }

    fn validate_arguments(&self, arguments: &HashMap<String, f64>, tail_length: &Option<f64>) -> Result<(), String> {
        verify_min(&Delay::WET_ARG.to_string(), 0.0, arguments)?;
        verify_min(&Delay::TIME_ARG.to_string(), 1.0, arguments)?;
        let feedback = verify_min(&Delay::FEEDBACK_ARG.to_string(), 0.0, arguments)?;

        if feedback >= 1.0 && tail_length.is_none() {
            return Err("Tail length (--tail, -t) is required for feedback >= 1 to avoid infinite feedback cycles".to_string());
        }

        Ok(())
    }

    fn apply_effect(&self, audio_buffer: &mut AudioBuffer, arguments: &HashMap<String,f64>, tail_length: &Option<f64>) -> Result<Option<String>, String> {
        let wet = arguments.get(Delay::WET_ARG).unwrap();
        let feedback = arguments.get(Delay::FEEDBACK_ARG).unwrap();
        let time = arguments.get(Delay::TIME_ARG).unwrap();
        let sample_rate = audio_buffer.spec.sample_rate as i32;
        
        for channel in audio_buffer.channels.iter_mut() {
            let ringbuffer_size = ((time / 1000.0) * sample_rate as f64) as usize;
            let mut buffer: AllocRingBuffer<f64> = AllocRingBuffer::new(ringbuffer_size);
        
            for _ in 0..ringbuffer_size {
                buffer.enqueue(0.0);
            }
        
            for sample in channel.iter_mut() {
                let delayed = *buffer.front().unwrap();
                buffer.enqueue(*sample + delayed * feedback);
                *sample += delayed * wet;
            }
        
            let mut square_sum= buffer.iter().map(|val| val * val).sum::<f64>();
            let normalizing_factor = (1.0/ringbuffer_size as f64).sqrt();
            

            if tail_length.is_some() {
                let length = tail_length.unwrap();
                let tail_samples = length as i32 * sample_rate;
                for _ in 0 .. tail_samples {
                    if channel.len() as f64 / audio_buffer.spec.sample_rate as f64 > length {
                        break;
                    }
                    buffer.enqueue(buffer.front().unwrap() * feedback);
                    channel.push(buffer.front().unwrap() * wet);
                }
            } else {
                let mut count = 0;
                let max = MAX_TAIL_LENGTH_SECONDS * sample_rate;
                while normalizing_factor * square_sum.sqrt() > MIN_DELAY_ENERGY {
                    if count >= max {
                        return Ok(Some("maximum tail length reached!".to_string()));
                    }

                    let front_val = *buffer.front().unwrap();
                    square_sum -= front_val * front_val;
                    
                    buffer.enqueue(front_val * feedback);

                    let back_val = buffer.back().unwrap();
                    square_sum += back_val * back_val;

                    channel.push(buffer.front().unwrap() * wet);

                    count += 1;
                }
            }
        }

        Ok(None)
    } 
}