fn main() {
    println!("Hello, world!");
}

impl Copy for &mut usize {

}

fn quick_sort_control(nums: &mut Vec<i32>) -> Result<Vec<i32>, std::io::Error>{
    let mut start: usize = 0;
    let mut end = &mut nums.len();
    let mut pivot = {|i| nums[start]};
    loop {
        if start > nums.len() {
            return Ok((&nums).to_vec());
        }
        else {
            quick_sort(nums.to_owned(), pivot(end), start, **(&mut end));
        }
    }
}

fn quick_sort(nums: Vec<i32>, pivot: i32, start: usize, end: usize) -> Result<Vec<i32>, std::io::Error>{

    Ok((&nums).to_vec())
}