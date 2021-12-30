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
    
<<<<<<< HEAD
    let raw_data = String::from("data here");
    let mut vector1: Vec<f32> = vec![];
    for number in raw_data.split_whitespace().into_iter() {
        vector1.push(number.parse::<f32>().unwrap());
    }
    
    println!("{:?}", dice_rolling::run(vector1));
=======
    let raw_data = String::from("0.19024707609788
0.57775597367436
0.76489249337465
0.08663949277252
0.30960991838947
0.44025089824572
0.58970281574875
0.26440902566537
0.38951328024268
0.59888464538381
0.042100549209863
0.559707055334
0.043924713972956
0.16023826412857
0.53505248716101
0.057690093759447
0.48253987496719
0.39592135604471
0.76049414370209
0.82838627509773
0.9889919841662
0.31147349299863
0.67904654843733
0.33152874559164
0.3242226280272
0.51369008002803");
    let mut data: Vec<f32> = vec![];
    for number in raw_data.split_whitespace().into_iter() {
        data.push(number.parse::<f32>().unwrap());
    }
    let answer: Vec<i32> = dice_rolling::run(data);
    
    //println!("{:?}", dice_rolling::run(vector1));
    answer.iter().for_each(|val| print!("{} ", val));
>>>>>>> master
}
