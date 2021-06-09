// Allows
#![allow(non_snake_case)]
// Imports
mod commands;

use std::time::{Instant};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

//

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let mut file = open_file();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    parse(&mut contents, Option::None)?;
    // Us is micro seconds
    println!("The program took {}us to complete!", now.elapsed().as_micros());
    Ok(())
}

fn open_file() -> File {
    let args: Vec<_> = env::args().collect::<Vec<String>>();
    match Some(&args) {
        None => panic!("No file passed"),
        _ => match File::open(args[1].clone()) {
            Ok(n) => return n,
            _ => panic!("No such file"),
        },
    }
}

fn parse(
    contents: &mut String,
    TEMP_VARIABLES: Option<HashMap<String, Vec<u8>>>,
) -> Result<i32, std::io::Error> {
    // Variables
    let mut STACK: Vec<u8> = vec![];
    let mut CODE_POINTER: usize = 0;
    let mut START_LOOP: usize = 0;
    let mut ITER: u8 = 0;
    let mut LOOP_TYPE: commands::LoopTypes = commands::LoopTypes::Null;
    let mut VARIABLES: HashMap<String, Vec<u8>>;
    match TEMP_VARIABLES {
        Some(n) => {
            VARIABLES = n;
        }
        _ => {
            VARIABLES = HashMap::new();
        }
    }
    //

    let lines: Vec<&str> = contents.split("\r\n").collect::<Vec<&str>>();

    //println!("{:?}", lines);
    loop {
        let line = lines[CODE_POINTER].split(" ").collect::<Vec<&str>>();
        match line.first().copied().unwrap() {
            "COPY" => (),
            "SWITCH" => (),
            "SET" => commands::set(&mut STACK, &mut VARIABLES, line),
            "GET" => commands::get(&mut STACK, &mut VARIABLES, line),
            "OUT" => commands::out(&mut STACK, line),
            "IN" => commands::input(&mut STACK, line),
            "DIV" => commands::div(&mut STACK, line),
            "PUSH" => commands::push(&mut STACK, line),
            "PUSHB" => STACK = commands::pushb(&mut STACK, line),
            "LDITER" => {
                ITER = STACK[STACK.len() - 1];
                STACK.pop();
            }
            "CLRITER" => ITER = 0,
            "MIN" => ITER -= 1,
            "OLOOP" => {
                START_LOOP = CODE_POINTER;
                LOOP_TYPE = commands::get_loop_type(line)
            }
            "CLOOP" => {
                if      (matches!(LOOP_TYPE, commands::LoopTypes::UntilEmpty)       && STACK.len() > 0)
                    ||  (matches!(LOOP_TYPE, commands::LoopTypes::UntilZero)        && STACK[STACK.len() - 1] != 0)
                    ||  (matches!(LOOP_TYPE, commands::LoopTypes::UntilIterZero)    && ITER > 0)
                {
                    CODE_POINTER = START_LOOP
                }
            }
            "\r\n" | "\n" | " " | "" => (),
            n => match n == n.to_uppercase() {
                true => println!("Unrecognized command: {:?}", n),
                _ => (),
            },
        }
        CODE_POINTER += 1;
        if CODE_POINTER == lines.len() {
            break;
        }
    }
    println!();
    Ok(0)
}
