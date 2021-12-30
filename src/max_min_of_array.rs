// /src/max_min_of_array.rs

// Returns max and min of array
/// :param vector1 Vec<i32>: Vector of numbers
/// :rtype: [i32; 2]
/// :return: An array containing `max, min`
fn max_min_of_array(vector1: Vec<i32>) -> [i32; 2] {
    let max_value = vector1.iter().max().unwrap();
    let min_value = vector1.iter().min().unwrap();
    return [*max_value, *min_value];
}

// Takes input from /src/main.rs
pub fn run(vector1: Vec<i32>) {
   println!("{:?}", max_min_of_array(vector1));
}
