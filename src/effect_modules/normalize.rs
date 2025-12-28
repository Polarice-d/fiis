use std::{collections::HashMap};
use crate::types::{AudioBuffer, AudioEffect};

const COMMAND_NAME: &str = "normalize";

pub struct Normalize;

impl AudioEffect for Normalize {
    fn get_name(&self) -> String { COMMAND_NAME.to_string() }

    fn validate_arguments(&self, _arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<(), String> {
        Ok(())
    }

    fn apply_effect(&self, buffer: &mut AudioBuffer, _arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<(), String> {
        for channel in buffer.channels.iter_mut() {

            let mut max = 0.0;
            for sample in channel.iter() {
                if *sample > max {
                    max = *sample;
                }
            };

            if max == 0.0 {
                continue;
            }

            for sample in channel.iter_mut() {
                *sample = (*sample/max).clamp(-1.0, 1.0) ;
            }
        }
        Ok(())
    }
}