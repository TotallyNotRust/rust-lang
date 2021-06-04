mod commands;

use std::fs::File;
use std::io::prelude::*;
use std::env;

// Statics
    static mut STACK: Vec<u8> = vec![];
    static mut CODE_POINTER: usize = 0;
    static mut START_LOOP: usize = 0;
    static mut ITER: u8 = 0;
    static mut LOOP_TYPE: commands::LoopTypes = commands::LoopTypes::Null;
//

fn main() {
    match parse() {
        Ok(_) => println!("Succes"),
        Err(n) => println!("Fail!, {}", n)
    }
}

fn open_file() -> File {
    let args: Vec<_> = env::args()
                            .collect::<Vec<String>>();
    match Some(&args) {
        None => panic!("No file passed"),
        _ => match File::open(args[1].clone()) {
            Ok(n) => return n,
            _ => panic!("No such file"),
        }
    }
}

fn parse() -> Result<i32, std::io::Error> {
    let mut file = open_file();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //println!("{}", contents);

    let lines: Vec<String> = contents
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
                "OUT" => commands::out(&mut STACK, &lines, &CODE_POINTER),
                "IN" => commands::input(&mut STACK, &lines, &CODE_POINTER),
                "DIV" => commands::div(&mut STACK, &lines, &CODE_POINTER),
                "PUSH" => commands::push(&mut STACK, &lines, &CODE_POINTER),
                "PUSHB" => STACK = commands::pushb(&mut STACK, &lines, &CODE_POINTER),
                "LDITER" => {ITER = STACK[STACK.len()-1]; STACK.pop();},
                "CLRITER" => ITER = 0,
                "MIN" => ITER -= 1,
                "OLOOP" => {START_LOOP = CODE_POINTER; LOOP_TYPE = commands::get_loop_type(&lines, &CODE_POINTER)},
                "CLOOP" => {
                    if  (matches!(LOOP_TYPE, commands::LoopTypes::UntilEmpty) && STACK.len() > 0) || 
                        (matches!(LOOP_TYPE, commands::LoopTypes::UntilZero)  && STACK[STACK.len()-1] != 0) ||
                        (matches!(LOOP_TYPE, commands::LoopTypes::UntilIterZero)  && ITER > 0) {
                        CODE_POINTER = START_LOOP
                    } 
                    else {START_LOOP = 0}}
                n => println!("Unrecognized command: {}", n)
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