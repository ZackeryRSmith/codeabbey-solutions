<<<<<<< HEAD
pub fn run(vector1: Vec<f32>) -> Vec<f32> {
    let mut vector2: Vec<f32> = vec![];
    for num in vector1.iter() {
        let test = num*6.0+1.0;
        //vector2.push(test.floor().round());
        vector2.push(test.floor().round());
    }
    return vector2;
=======
pub fn run(data: Vec<f32>) -> Vec<i32> {
    let mut answer: Vec<i32> = vec![];
    for num in data.iter() {
        answer.push((num*6.0+1.0) as i32);
    }
    answer
>>>>>>> master
}
