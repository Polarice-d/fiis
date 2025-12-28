use core::f64;
use std::{collections::HashMap};
use crate::{parse_utils::arg_exists, types::{AudioBuffer, AudioEffect}};
use crate::parse_utils::verify_range;

pub struct PeakingEQ;

const COMMAND_NAME: &str = "peakingeq";
const DB_NAME: &str = "db";
const FREQ_NAME :&str = "freq";
const Q_NAME: &str = "q";

impl AudioEffect for PeakingEQ {
    fn get_name(&self) -> String {
        COMMAND_NAME.to_string()
    }

    fn validate_arguments(&self, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<(), String> {
        arg_exists(&DB_NAME.to_string(), arguments)?;
        verify_range(&FREQ_NAME.to_string(), 1.0, 20000.0, arguments)?;
        verify_range(&Q_NAME.to_string(), 0.1, 100.0, arguments)?;
        Ok(())
    }

    fn apply_effect(&self, buffer: &mut AudioBuffer, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<(), String> {
        let fs = buffer.spec.sample_rate as f64;
        let f0 = *arguments.get(FREQ_NAME).unwrap();
        let db_gain = *arguments.get(DB_NAME).unwrap();
        let q = *arguments.get(Q_NAME).unwrap();
        let a= 10.0_f64.powf(db_gain/40.0);
        let w0 = 2.0 * f64::consts::PI * (f0/fs);
        let sinw0 = w0.sin();
        let cosw0 = w0.cos();
        let alpha = sinw0 /(2.0 * q);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cosw0;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha/a;
        let a1 = -2.0 * cosw0;
        let a2 = 1.0 - alpha/a;

        for channel in buffer.channels.iter_mut() {

            let mut x1 = 0.0;  // x[n-1]
            let mut x2 = 0.0;  // x[n-2]
            let mut y1 = 0.0;  // y[n-1] 
            let mut y2 = 0.0;  // y[n-2]

            for sample in channel {
                let x0 = *sample;
                let y0 = (b0/a0) * x0 + (b1/a0) * x1 + (b2/a0) * x2 - (a1/a0) * y1 - (a2/a0) * y2;

                *sample = y0;  

                x2 = x1;
                x1 = x0;
                y2 = y1;
                y1 = y0;
            }
        }

        Ok(())

    }

}
