use std::collections::HashMap;

use crate::{parse_utils::arg_exists, types::{AudioBuffer, AudioEffect}};

pub struct Softclip;

impl Softclip {
    const NAME: &str = "softclip";
    const DB_ARG: &str = "db";
}

impl AudioEffect for Softclip {
    fn get_name(&self) -> String { Softclip::NAME.to_string() }

    fn validate_arguments(&self, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<(), String> {
        arg_exists(&Softclip::DB_ARG.to_string(), arguments)?;
        Ok(())
    }

    fn apply_effect(&self, buffer: &mut AudioBuffer, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<Option<String>, String> {
        let db = arguments.get(Softclip::DB_ARG).unwrap();
        let factor = 10.0_f64.powf(db / 20.0);

        for channel in buffer.channels.iter_mut() {
            for sample in channel.iter_mut() {
                *sample = (*sample * factor).tanh();
            };
        }
        
        Ok(None)
    }
}