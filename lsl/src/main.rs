use lsl;
use lsl::Pullable;

fn main() -> Result<(), lsl::Error> {

    // resolve a data stream and create an inlet to read from it
    let res = lsl::resolve_bypred("name='BioSemi' and type='EEG'", 1, lsl::FOREVER)?;
    let inl = lsl::StreamInlet::new(&res[0], 360, 0, true)?;

    // read the streaming data and print the multi-channel samples 
    loop {
        let (sample, ts): (Vec<f32>, _) = inl.pull_sample(lsl::FOREVER)?;
        println!("got {:?} at time {}", sample, ts);
    }
}