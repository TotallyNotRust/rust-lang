mod commands;

use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

static mut STACK: Vec<u8> = vec![];
static mut CODE_POINTER: usize = 0;
static mut START_LOOP: usize = 0;

fn main() {
    match parse() {
        Ok(_) => println!("Succes"),
        Err(_) => println!("Fail!")
    }
}

fn parse() -> Result<i32, std::io::Error> {
    let mut file = File::open("program.l")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //println!("{}", contents);

    let mut lines: Vec<String> = contents
                        .split("\n")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|&i| {
                            let mut output = String::from(i);
                            match output.chars().last() {
                                Some(n) => if n == '\r' {output.pop();},
                                _ => ()
                            }
                            output
                        })
                        .collect::<Vec<String>>();

    //println!("{:?}", lines);
    loop {
        unsafe {
            match lines[CODE_POINTER].split(" ").collect::<Vec<&str>>().first().copied().unwrap() {
                "OUT" => commands::out(&mut STACK),
                "DIV" => commands::div(&mut STACK, &lines, &CODE_POINTER),
                "PUSH" => commands::push(&mut STACK, &lines, &CODE_POINTER),
                "[" => START_LOOP = CODE_POINTER,
                "]" => {if STACK.len() > 0 {CODE_POINTER = START_LOOP} else {START_LOOP = 0}}
                _ => ()
            }
            CODE_POINTER += 1;
            if CODE_POINTER == lines.len() {
                break;
            }
        }
    }
    println!();
    Ok(0)
}