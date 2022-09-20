use std::thread;
use std::time::Instant;
use std::sync::{Mutex, Arc};
use hound;


fn main() {
    const THREAD_NUMBER: usize = 6;
    let filepath = "wav_files/12.-かめりあ-—-PLANET-SHAPER.wav";
    let reader = Arc::new(Mutex::new(hound::WavReader::open(filepath).unwrap()));
    // more than one handle
    // store them in a vec
    // for convenience
    let mut handles = vec![];
    // thread-safe and lockable
    let number_of_positive_values = Arc::new(Mutex::new(0));
    let now = Instant::now();
    for i in 0..THREAD_NUMBER {
        // clone the transmitters
        let number_of_positive_values = Arc::clone(&number_of_positive_values);
        let reader = Arc::clone(&reader);
        // create thread
        let handle = thread::spawn(move || {
            let positive_values = reader.lock().unwrap()
                .samples::<i16>()
                .step_by(i+1) // iterator of ith thread takes only i+1th value
                .fold(0, |acc, s| {
                let sample = s.unwrap() as i16;
                acc + (sample.is_positive()) as u32
            });
            // lock the value
            let mut counter = number_of_positive_values.lock().unwrap();
            // mutate the value
            *counter += positive_values;
            println!("Thread {} spawned", i);
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
    println!("Amount of positive values in wav: {}", *number_of_positive_values.lock().unwrap());
    println!("Time spent: {} ms", now.elapsed().as_millis());
}