use std::io::BufRead;

// Main file, executes run() function in .rs files (Keeps code clean)

/*
 * Modules
 */
//mod sum_a_plus_b;
//mod sum_in_loop;
//mod sums_in_loop;
//mod minimum_of_two;
//mod max_min_of_array;
mod dice_rolling;

fn main() {
    /*
    let (a, b) = (data, here);
    sum_a_plus_b::run(&a, &b);
    */

    /* sum_in_loop
    let raw_array_string = String::from("data here");
    let mut num_vector: Vec<i32> = vec![];
    for n in raw_array_string.split_whitespace() {
        num_vector.push(n.parse::<i32>().unwrap())
    }
    sum_in_loop::run(&num_vector);
    */

    /* sums_in_loop
    let raw_data = String::from("data here");
    let mut vector1: Vec<i32> = vec![];
    let mut vector2: Vec<i32> = vec![];
    for (index, number) in raw_data.split_whitespace().into_iter().enumerate() {
        if (index+1)%2!=0 {
            vector1.push(number.parse::<i32>().unwrap());
        } else {
            vector2.push(number.parse::<i32>().unwrap());
        }
    }
    sums_in_loop::run(vector1, vector2);
    */
    
    /* minimum_of_two
    let raw_data = String::from("data here");
    let mut vector1: Vec<i32> = vec![];
    let mut vector2: Vec<i32> = vec![];
    for (index, number) in raw_data.split_whitespace().into_iter().enumerate() {
        if (index+1)%2!=0 {
            vector1.push(number.parse::<i32>().unwrap());
        } else {
            vector2.push(number.parse::<i32>().unwrap());
        }
    }
    minimum_of_two::run(vector1, vector2);
    */
    
    /* max_min_of_array  (Vector can be removed and swapped from a array)
    let raw_data = String::from("data here");
    let mut vector1: Vec<i32> = vec![];
    for (_, number) in raw_data.split_whitespace().into_iter().enumerate() {
        vector1.push(number.parse::<i32>().unwrap())
    }
    max_min_of_array::run(vector1);
    */
    
    let raw_data = String::from("data here");
    let mut data: Vec<f32> = vec![];
    for number in raw_data.split_whitespace().into_iter() {
        data.push(number.parse::<f32>().unwrap());
    }
    let answer: Vec<i32> = dice_rolling::run(data);
    
    //println!("{:?}", dice_rolling::run(vector1));
    answer.iter().for_each(|val| print!("{} ", val));
}
