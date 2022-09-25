use std::fs::File;
use std::thread;
use std::sync::{Mutex, Arc};
use wav_io;

pub fn wav (tn: usize, p: &str) -> u32 {
    // open file
    let f = File::open(p).unwrap();
    // read from file
    let (_, samples) = wav_io::read_from_file(f).unwrap();
    // more than one handle
    // store them in a vec
    // for convenience
    let mut handles = vec![];
    // thread-safe and lockable
    let safe_samples = Arc::new(Mutex::new(samples));
    let number_of_positive_values = Arc::new(Mutex::new(0));
    // looping for threads
    for i in 0..tn {
        // clone the transmitters
        let number_of_positive_values = Arc::clone(&number_of_positive_values);
        let safe_samples = Arc::clone(&safe_samples);
        // create thread
        let handle = thread::spawn(move || {
            // println!("Thread {} spawned", i);
            let positive_values = safe_samples.lock().unwrap()
                .iter()
                .skip(i) // sets an offset for the each thread
                .step_by(tn) // iterates over every THREAD_NUMBER'th value in sequence
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
    return *number_of_positive_values.lock().unwrap();
}