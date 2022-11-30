// use std::time::Instant;
use std::fs::File;
use wav_io;
// mod std_thread;
// mod create_thread;
// mod create_process;
use libc::{c_float, size_t};


const FILE_PATH: &str = "wav_files/12.-かめりあ-—-PLANET-SHAPER.wav";
const THREAD_NUMBER: usize = 10;

extern "C" {
    fn gpu_cpv(vec: *mut c_float, N: size_t) -> c_float;
}

fn main() {
    // open file
    let f = File::open(FILE_PATH).unwrap();
    // read from file
    let (_, mut samples) = wav_io::read_from_file(f).unwrap();
    // let a = std_thread::count_positive_values(samples, THREAD_NUMBER);
    // let a = create_thread::count_positive_values(samples, THREAD_NUMBER);
    // let a = create_process::count_positive_values(samples, THREAD_NUMBER);
    let mut a: c_float;
    unsafe {
        a = gpu_cpv(samples.as_mut_ptr(), samples.len());
    }
    println!("Positive values: {}", a);
}