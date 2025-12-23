use hound::WavSpec;
use std::collections::HashMap;

pub trait AudioEffect {
    fn get_name(&self) -> String;
    fn validate_arguments(&self, arguments: &HashMap<String, f32>, tail_length: &Option<f32>) -> Result<(), String>;
    fn apply_effect(&self, buffer: &mut AudioBuffer, arguments: &HashMap<String, f32>, tail_length: &Option<f32>) -> Result<(), String>;
}

pub struct AudioBuffer {
    pub spec: WavSpec,
    pub samples: Vec<f32>
}

pub struct EffectSpec {
    pub name: String,
    pub arguments: HashMap<String, f32>
}
