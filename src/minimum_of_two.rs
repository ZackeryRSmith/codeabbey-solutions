// /src/minimum_of_two.rs
// code recycled from /src/sums_in_loop.rs

// Uses two equal length vectors with numbers, then compares
/// :param vector1 Vec<i32>: First vector of numbers
/// :param vector2 Vec<i32>: Second vector of numbers
/// :rtype: Vec<i32>
/// :return: Returns a vector; the result of comparing vector1 & vector2 
fn minimum_of_two(vector1: Vec<i32>, vector2: Vec<i32>) -> Vec<i32> {
    let mut final_vector: Vec<i32> = vec![];
    // Make sure there are a equal amount of numbers to add
    if vector1.len() == vector2.len() {
        for index in 0..vector1.len() {
            // Push the comparison to `final_vector`
            if vector1[index]<vector2[index] {
                final_vector.push(vector1[index]);
            } else {
                final_vector.push(vector2[index]);
            }
        }
    } else {
        panic!("Vectors do not have same length!");
    }
    return final_vector;
}

// Gets input from main.rs
pub fn run(vector1: Vec<i32>, vector2: Vec<i32>) {
    println!("{:?}", minimum_of_two(vector1, vector2));
}   
                                                                                 
                                                                                 

