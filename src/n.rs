extern crate rand;


fn main() {
    println!("{:?}", run_prime())
}

fn run_multiply() {
    let mut lst = vec![1,2,3,4];
    let ind = 4;
    lst.push(221);
    multiply(lst[ind as usize], 4);
    println!("{}", lst.last().unwrap());
}

fn multiply(number: i32, by: i32) -> i32 {
    for i in 1..=10 {
        println!("{}", i as i32)
    }
    
    return number * by;
}

fn run_prime() {
    let mut s = String::new();
    use std::io::{stdin, stdout, Write};
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("No!");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    println!("{:?}", is_prime(s.parse::<i32>().unwrap()));
}

fn is_prime(num: i32) -> bool {
    let mut bool_is_prime = true;
    for i in 2..num {
        if num % i == 0{
            bool_is_prime = false;
            break;
        }
    }
    return bool_is_prime;
}


