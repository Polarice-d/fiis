use crate::types::AudioBuffer;

pub fn sanitize_buffer(buffer: &mut AudioBuffer) -> Result<(), String> {

    let mut max = 0;
    for channel in buffer.channels.iter() {
        for sample in channel.iter() {
            if sample.is_nan() || sample.abs() > f64::MAX {
                return Err("Sample values became invalid (NaN or infinity), cannot process further".to_string());
            }
        }

        let len = channel.len();
        if len > max {
            max = len;
        }
    }
    
    if max == 0 {
        return Err("Audio became zero-length, cannot process further".to_string());
    }

    for channel in buffer.channels.iter_mut() {
        channel.resize(max, 0.0);
    }


    

    Ok(())

}