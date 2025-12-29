use std::collections::HashMap;
use crate::{parse_utils::{arg_exists, verify_range, verify_min}, types::{AudioBuffer, AudioEffect}};

pub struct Template;

impl Template {
    pub const NAME: &'static str = "";
    pub const _ARG: &'static str = "";
}

impl AudioEffect for Template {
    fn get_name(&self) -> String { Template::NAME.to_string() }

    fn validate_arguments(&self, arguments: &HashMap<String, f64>, tail_length: &Option<f64>) -> Result<(), String> {
        Ok(())
    }

    fn apply_effect(&self, buffer: &mut AudioBuffer, arguments: &HashMap<String, f64>, tail_length: &Option<f64>) -> Result<Option<String>, String> {
        Ok(())
    }
}