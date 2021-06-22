use std::time::{Instant}; 

fn main() {
    let mut vec1: Vec<i32> = vec![112412,412,412,412,412,412,4124,21,4];
    println!("{:?}", vec1);
    let now = Instant::now();
    vec1 = vec1.iter().map(|_| 0).collect::<Vec<i32>>();
    let time = now.elapsed();
    println!("{:?}", vec1);
    println!("The program took {:?} to complete!", time);
}
