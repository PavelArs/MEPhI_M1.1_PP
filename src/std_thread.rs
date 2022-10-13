use std::thread;
use std::sync::{Arc, Mutex};
use num_integer;
use crate::my_func;

pub fn count_positive_values(arr: Vec<f32>, tn: usize) -> u32 {
    // more than one handle
    // store them in a vec
    // for convenience
    let mut handles = vec![];
    // thread-safe and lockable
    let safe_arr = Arc::new(Mutex::new(arr));
    let number_of_positive_values = Arc::new(Mutex::new(0));
    // looping for threads
    for i in 0..tn {
        // clone the transmitters
        let number_of_positive_values = Arc::clone(&number_of_positive_values);
        let safe_arr = Arc::clone(&safe_arr);
        // create thread
        let handle = thread::spawn(move || {
            let l = safe_arr.lock().unwrap().len();
            let left = num_integer::div_floor(i * l, tn) as usize;
            let right = num_integer::div_floor((i + 1) * l, tn) as usize;
            let positive_values = my_func::count_positive_values(
                (&safe_arr.lock().unwrap()[left..right]).to_vec()
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
