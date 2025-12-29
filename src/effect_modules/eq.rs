use core::f64;
use std::collections::HashMap;

use crate::{parse_utils::{arg_exists, verify_range, verify_min}, types::{AudioBuffer, AudioEffect}};

pub struct PeakingEQ;

fn apply_df1(buffer: &mut AudioBuffer, a0:f64, a1:f64, a2:f64, b0:f64, b1:f64, b2:f64) {
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
}

impl PeakingEQ {
    pub const NAME: &'static str = "peakingeq";
    pub const DB_ARG: &'static str = "db";
    pub const FREQ_ARG: &'static str = "freq";
    pub const BW_ARG: &'static str = "bw";
}

impl AudioEffect for PeakingEQ {

    fn get_name(&self) -> String {
        PeakingEQ::NAME.to_string()
    }

    fn validate_arguments(&self, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<(), String> {
        arg_exists(&PeakingEQ::DB_ARG.to_string(), arguments)?;
        verify_range(&PeakingEQ::FREQ_ARG.to_string(), 1.0, 20000.0, arguments)?;
        verify_min(&PeakingEQ::BW_ARG.to_string(), 0.01, arguments)?;
        Ok(())
    }

    fn apply_effect(&self, buffer: &mut AudioBuffer, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<Option<String>, String> {
        let f0 = *arguments.get(PeakingEQ::FREQ_ARG).unwrap();
        let db_gain = *arguments.get(PeakingEQ::DB_ARG).unwrap();
        let bw = *arguments.get(PeakingEQ::BW_ARG).unwrap();
        
        let fs = buffer.spec.sample_rate as f64;
        let a= 10.0_f64.powf(db_gain/40.0);
        let w0 = 2.0 * f64::consts::PI * (f0/fs);
        let sinw0 = w0.sin();
        let cosw0 = w0.cos();
        let alpha = sinw0 * (2.0_f64.log2()/2.0 * bw * (w0/sinw0)).sinh();

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cosw0;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha/a;
        let a1 = -2.0 * cosw0;
        let a2 = 1.0 - alpha/a;

        apply_df1(buffer, a0, a1, a2, b0, b1, b2);

        Ok(None)
    }

}

pub struct HShelfEQ;

impl HShelfEQ {
    pub const NAME: &'static str = "hshelfeq";
    pub const S_ARG: &'static str = "s";
}

impl AudioEffect for HShelfEQ {

    fn get_name(&self) -> String {
        HShelfEQ::NAME.to_string()
    }

    fn validate_arguments(&self, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<(), String> {
        arg_exists(&PeakingEQ::DB_ARG.to_string(), arguments)?;
        verify_range(&PeakingEQ::FREQ_ARG.to_string(), 1.0, 20000.0, arguments)?;
        verify_range(&HShelfEQ::S_ARG.to_string(), 0.01, 1.0, arguments)?;
        Ok(())
    }

    fn apply_effect(&self, buffer: &mut AudioBuffer, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<Option<String>, String> {

        let fs = buffer.spec.sample_rate as f64;
        let f0 = *arguments.get(PeakingEQ::FREQ_ARG).unwrap();
        let db_gain = *arguments.get(PeakingEQ::DB_ARG).unwrap();
        let s = *arguments.get(HShelfEQ::S_ARG).unwrap();

        let a= 10.0_f64.powf(db_gain/40.0);
        let w0 = 2.0 * f64::consts::PI * (f0/fs);
        let sinw0 = w0.sin();
        let cosw0 = w0.cos();
        let alpha = (sinw0 / 2.0) * ((a + 1.0/a) * (1.0/s - 1.0) + 2.0).sqrt();

        let b0 = a * ((a + 1.0) + (a - 1.0) * cosw0 + 2.0 * a.sqrt() * alpha);
        let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cosw0);
        let b2 = a * ((a+1.0) + (a - 1.0) * cosw0 - 2.0 * a.sqrt() * alpha);
        let a0 = (a + 1.0) - (a-1.0) * cosw0 + 2.0 * a.sqrt() * alpha;
        let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cosw0);
        let a2 = (a + 1.0) - (a - 1.0) * cosw0 - 2.0 * a.sqrt() * alpha;

        apply_df1(buffer, a0, a1, a2, b0, b1, b2);

        Ok(None)
    }
}

pub struct LShelfEQ;

impl LShelfEQ {
    pub const NAME: &'static str = "lshelfeq";
}

impl AudioEffect for LShelfEQ {

    fn get_name(&self) -> String {
        LShelfEQ::NAME.to_string()
    }

    fn validate_arguments(&self, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<(), String> {
        arg_exists(&PeakingEQ::DB_ARG.to_string(), arguments)?;
        verify_range(&PeakingEQ::FREQ_ARG.to_string(), 1.0, 20000.0, arguments)?;
        verify_range(&HShelfEQ::S_ARG.to_string(), 0.01, 1.0, arguments)?;
        Ok(())
    }

    fn apply_effect(&self, buffer: &mut AudioBuffer, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<Option<String>, String> {

        let fs = buffer.spec.sample_rate as f64;
        let f0 = *arguments.get(PeakingEQ::FREQ_ARG).unwrap();
        let db_gain = *arguments.get(PeakingEQ::DB_ARG).unwrap();
        let s = *arguments.get(HShelfEQ::S_ARG).unwrap();

        let a= 10.0_f64.powf(db_gain/40.0);
        let w0 = 2.0 * f64::consts::PI * (f0/fs);
        let sinw0 = w0.sin();
        let cosw0 = w0.cos();
        let alpha = (sinw0 / 2.0) * ((a + 1.0/a) * (1.0/s - 1.0) + 2.0).sqrt();


        let b0 = a * ((a + 1.0) - (a - 1.0) * cosw0 + 2.0 * a.sqrt() * alpha);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cosw0);
        let b2 = a * ((a+1.0) - (a - 1.0) * cosw0 - 2.0 * a.sqrt() * alpha);
        let a0 = (a + 1.0) + (a-1.0) * cosw0 + 2.0 * a.sqrt() * alpha;
        let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cosw0);
        let a2 = (a + 1.0) + (a - 1.0) * cosw0 - 2.0 * a.sqrt() * alpha;

        apply_df1(buffer, a0, a1, a2, b0, b1, b2);

        Ok(None)
    }
}


pub struct BandPassEQ;

impl BandPassEQ {
    pub const NAME: &'static str = "bandpasseq";
    pub const Q_ARG: &'static str = "q";
}

impl AudioEffect for BandPassEQ {

    fn get_name(&self) -> String {
        BandPassEQ::NAME.to_string()
    }

    fn validate_arguments(&self, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<(), String> {
        verify_range(&PeakingEQ::FREQ_ARG.to_string(), 1.0, 20000.0, arguments)?;
        verify_min(&BandPassEQ::Q_ARG.to_string(), 0.01, arguments)?;
        Ok(())
    }

    fn apply_effect(&self, buffer: &mut AudioBuffer, arguments: &HashMap<String, f64>, _tail_length: &Option<f64>) -> Result<Option<String>, String> {

        let fs = buffer.spec.sample_rate as f64;
        let f0 = *arguments.get(PeakingEQ::FREQ_ARG).unwrap();
        let q = *arguments.get(BandPassEQ::Q_ARG).unwrap();

        let w0 = 2.0 * f64::consts::PI * (f0/fs);
        let sinw0 = w0.sin();
        let cosw0 = w0.cos();
        let alpha = sinw0 /(2.0 * q);

        let b0 = alpha;
        let b1 = 0.0;
        let b2 = -alpha;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cosw0;
        let a2 = 1.0 - alpha;

        apply_df1(buffer, a0, a1, a2, b0, b1, b2);

        Ok(None)
    }
}