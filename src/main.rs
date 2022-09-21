use std::fs::File;
use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Instant;
use wav_io;

fn main() {
    const THREAD_NUMBER: usize = 1;
    const FILE_PATH: &str = "wav_files/12.-かめりあ-—-PLANET-SHAPER.wav";
    // open file
    let f = File::open(FILE_PATH).unwrap();
    // read from file
    let (_, samples) = wav_io::read_from_file(f).unwrap();
    // more than one handle
    // store them in a vec
    // for convenience
    let mut handles = vec![];
    // thread-safe and lockable
    let safe_samples = Arc::new(Mutex::new(samples));
    let number_of_positive_values = Arc::new(Mutex::new(0));
    // to calculate time spent on execution
    let now = Instant::now();
    // looping for threads
    for i in 0..THREAD_NUMBER {
        // clone the transmitters
        let number_of_positive_values = Arc::clone(&number_of_positive_values);
        let safe_samples = Arc::clone(&safe_samples);
        // create thread
        let handle = thread::spawn(move || {
            println!("Thread {} spawned", i);
            let positive_values = safe_samples.lock().unwrap()
                .iter()
                .skip(i) // sets an offset for the each thread
                .step_by(THREAD_NUMBER) // iterates over every THREAD_NUMBER'th value in sequence
                .fold(0, |acc, s|
                acc + (s.is_sign_positive()) as u32
            );
            // lock the value
            let mut counter = number_of_positive_values.lock().unwrap();
            // mutate the value
            *counter += positive_values;
        });
        // push the handle into the handles
        // vector so we can join them
        handles.push(handle);
    }
    // join the handles in the vector
    for handle in handles {
        handle.join().unwrap();
    }
    // lock the value when accessing it
    println!("Amount of positive values in wav: {}", *number_of_positive_values.lock().unwrap()); // 18640325
    println!("Time spent: {} ms", now.elapsed().as_millis());
}