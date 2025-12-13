use hound::{WavSpec, WavWriter, SampleFormat};

pub fn encode_file(samples:Vec<f32>, spec:WavSpec, filename:String) {
    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    for sample in samples.iter() {
        writer.write_sample(*sample).unwrap();
    }
    
    writer.finalize().unwrap();
}

