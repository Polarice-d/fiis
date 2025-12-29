use std::collections::HashMap;
use crate::{parse_utils::arg_exists, types::{AudioBuffer, AudioEffect}};

pub struct Gain;

impl Gain {
    const NAME: &'static str = "gain"; 
    const DB_ARG: &'static str = "db";
}

impl AudioEffect for Gain {
    fn get_name(&self) -> String { Gain::NAME.to_string() }

    fn validate_arguments(&self, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<(), String> {
        let _db = arg_exists(&Gain::DB_ARG.to_string(), arguments)?;
        Ok(())
    }

    fn apply_effect(&self, buffer: &mut AudioBuffer, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<Option<String>, String> {
        let db = arguments.get(Gain::DB_ARG).unwrap();
        let factor = 10.0_f64.powf(db / 20.0);

        for channel in buffer.channels.iter_mut() {

            for sample in channel.iter_mut(){
                *sample = *sample * factor;
            };
        }

        Ok(None)
    }
}
