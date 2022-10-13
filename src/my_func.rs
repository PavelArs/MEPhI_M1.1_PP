pub fn count_positive_values(arr: Vec<f32>) -> u32 {
    return arr.iter().fold(0, |acc, x|
        acc + (x.is_sign_positive()) as u32,
    );
}