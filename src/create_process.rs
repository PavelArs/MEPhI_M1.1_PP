use std::fs;
use std::process::{Command, Stdio};
use std::string::ToString;

// fn vf_to_u8(v: &[f32]) -> &[u8] {
//     unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
// }

fn get_binary_from_values(values: &[f32]) -> String {
    let bits: Vec<_> = values.iter().map(|v| v.to_bits().to_string()).collect();
    bits.join(";")
}

// fn get_values_from_binary(bin: &str) -> Vec<f32> {
//     bin.split(";")
//         .map(|bits| f32::from_bits(bits.parse().unwrap()))
//         .collect()
// }

pub fn count_positive_values(arr: Vec<f32>, tn: usize) -> u32 {
    let l = arr.len();
    let mut left: usize;
    let mut right: usize;
    let text_file_dir = "text_files/";

    for i in 0..tn {
        let text_file_path = &(text_file_dir.to_owned() + &i.to_string() + &*String::from(".bin"));
        left = num_integer::div_floor(i * l, tn);
        right = num_integer::div_floor((i + 1) * l, tn);
        let str_arr = get_binary_from_values(&arr[left..right]);
        fs::write(text_file_path, str_arr)
            .expect("Unable to write file");
    }

    let mut count: u32 = 0;
    let mut handles = vec![];
    let mut process = Command::new("cmd");
    process.args([
            "python",
            "src\\calculations.py"
        ]);

    for i in 0..tn {
        let text_file_path = &(text_file_dir.to_owned() + &i.to_string() + &*String::from(".txt"));
        fs::read_to_string(text_file_path)
            .expect("Unable to read file");
        let child_process = process
            .args([
                "-f",
                text_file_path
            ])
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");

        handles.push(child_process);
    }

    for _i in 0..tn {
        let output = handles
            .pop()
            .unwrap()
            .wait_with_output()
            .expect("failed to wait on child");
        count += output.stdout[0] as u32;
    }

    return count;
}