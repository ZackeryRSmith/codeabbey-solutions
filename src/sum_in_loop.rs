// Adds numbers in a list to the sum

/// :param num_array [i32]: The array of numbers that will be added to the sum
/// :rtype: i64
/// :return: Returns a sum of all numbers in an array
fn sum_in_list(num_array: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for (index, _) in num_array.iter().enumerate() {
        sum += num_array[index];
    }
    return sum;
}

// will be imported and ran in main.rs
pub fn run(num_array: &[i32]) {
    println!("{}", sum_in_list(num_array));
}
