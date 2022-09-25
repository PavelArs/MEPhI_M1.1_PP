use std::time::Instant;
mod wav;

fn main() {
    const THREAD_NUMBER: usize = 1;
    const FILE_PATH: &str = "wav_files/12.-かめりあ-—-PLANET-SHAPER.wav";
    // to calculate time spent on execution
    let now = Instant::now();
    let n = wav::wav(THREAD_NUMBER, &FILE_PATH);
    println!("Amount of positive values in wav: {}", n); //18640325
    println!("Time spent: {} ms", now.elapsed().as_millis());
}