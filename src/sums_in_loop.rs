// Uses two equal length vectors with numbers, then adds 
/// :param vector1 Vec<i32>: First vector of numbers
/// :param vector2 Vec<i32>: Second vector of numbers
/// :rtype: Vec<i32>
/// :return: Returns a vector; the result of adding vector1 & vector2 
fn sums_in_loop(vector1: Vec<i32>, vector2: Vec<i32>) -> Vec<i32> {
    let mut final_vector: Vec<i32> = vec![];
    // Make sure there are a equal amount of numbers to add
    if vector1.len() == vector2.len() {
        for index in 0..vector1.len() {
            // Push the result of `vector1 + vector2`
            final_vector.push(vector1[index]+vector2[index]);
        }
    } else {
        panic!("Vectors do not have same length!");
    }
    return final_vector;
}

// Gets input from main.rs
pub fn run(vector1: Vec<i32>, vector2: Vec<i32>) {
    println!("{:?}", sums_in_loop(vector1, vector2));
}
