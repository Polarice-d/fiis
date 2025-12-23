use core::f32;
use std::collections::HashMap;
use crate::{types::{AudioBuffer, AudioEffect}};

pub struct Gain;

const COMMAND_NAME: &str = "gain";
const DB_NAME: &str = "db";

impl AudioEffect for Gain {
    fn get_name(&self) -> String { COMMAND_NAME.to_string() }

    fn validate_arguments(&self, arguments: &HashMap<String, f32>, _tail_length: &Option<f32>) -> Result<(), String> {
        let _db = arguments.get(DB_NAME).ok_or_else(|| format!("Missing gain argument '{DB_NAME}' (add '{DB_NAME}=x' to 'gain:')"))?;
        Ok(())
    }

    fn apply_effect(&self, buffer: &mut AudioBuffer, arguments: &HashMap<String, f32>, _tail_length: &Option<f32>) -> Result<(), String> {
        let db = arguments.get(DB_NAME).unwrap();
        
        let factor = 10.0_f32.powf(db / 20.0);
        for buf in buffer.samples.iter_mut() {
            *buf = *buf * factor;
            if buf.abs() > f32::MAX {
                return Err("Amplitude exceeded the 32 bit float range while processing.".to_string());
            }
        };

        Ok(())
    }
}
