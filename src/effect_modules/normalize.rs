use std::{collections::HashMap};
use crate::types::{AudioBuffer, AudioEffect};

pub struct Normalize;

impl Normalize {
    const NAME: &'static str = "normalize";
}

impl AudioEffect for Normalize {
    fn get_name(&self) -> String { Normalize::NAME.to_string() }

    fn validate_arguments(&self, _arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<(), String> {
        Ok(())
    }

    fn apply_effect(&self, buffer: &mut AudioBuffer, _arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<Option<String>, String> {
        let mut max = 0.0;
        for channel in buffer.channels.iter() {           
            for sample in channel.iter() {
                if sample.abs() > max {
                    max = sample.abs();
                }
            };
        }

        if max == 0.0 {
            return Ok(Some("audio buffer only has 0 amplitude samples".to_string()));
        }

        for channel in buffer.channels.iter_mut() {
            for sample in channel.iter_mut() {
                *sample = (*sample/max).clamp(-1.0, 1.0) ;
            }
        }

        if max.abs() > 1.0 {
            let message = format!("peak was +{:.1} dB", 20.0 * max.log10());
            Ok(Some(message))
        } else {
            Ok(None)
        }
        
    }
}