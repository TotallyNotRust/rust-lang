use rand;
use std::time::{Instant};

fn main() {
    let mut vec: Vec<u32> = Vec::with_capacity(1000);
    for _ in 0..vec.capacity() {
        vec.push(rand::random());
    };
    let now = Instant::now();  
    match bubble_sort(&mut vec) {
        Ok(_) => println!("Completed in {}ms", now.elapsed().as_millis()),
        Err(_) => (),
    }
}

fn bubble_sort(nums: &mut Vec<u32>) -> Result<Vec<u32>, std::io::Error> {
    'sort_loop: loop {
        let mut this_round = 0;
        for ind in 0..nums.len()-1{
            if nums[ind] > nums[ind+1 as usize] {
                nums.swap(ind, ind+1);
                this_round += 1;
            }
        }
        if this_round == 0 {
            break 'sort_loop;
        }
    }
    Ok((&nums).to_vec())
}