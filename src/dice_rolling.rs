pub fn run(data: Vec<f32>) -> Vec<i32> {
    let mut answer: Vec<i32> = vec![];
    for num in data.iter() {
        answer.push((num*6.0+1.0) as i32);
    }
    answer
}
