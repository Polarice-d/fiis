use hound;

fn main() {
    let mut reader = hound::WavReader::open("samples/41khz_16bit.wav").unwrap();
    let num = reader.samples::<i16>().count(); // I think this is the number of samples but i'm not sure
    println!("{num}");
}
