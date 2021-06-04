// Imports and allow
    #![allow(unused_must_use, dead_code, unused_mut, unused_imports)]

    mod math;
    use math::Math;

    use std::io;
    use std::io::{Read, Write, stdin, stdout};
// Loop types enum
    pub enum LoopTypes {
        UntilZero,
        UntilEmpty,
        UntilIterZero,
        Null
    }
// Misc methods
    fn parse_vec_ref_str_to_2_tuple_u8(nums: Vec<&str>) -> (u8,u8) {
        let mut op: Vec<u8> = vec![];
        for i in nums {
            match i.parse::<u8>()  {
                Ok(n) => op.push(n),
                Err(_) => ()
            }
        }
        return (op[0], op[1]);
    }
    pub fn get_loop_type(lines: &Vec<String>, code_pointer: &usize) -> LoopTypes {
        let temp = lines[*code_pointer].split(" ").collect::<Vec<&str>>();
        match temp[1] {
            "@e" => return LoopTypes::UntilEmpty,
            "@z" => return LoopTypes::UntilZero,
            "@iz" => return LoopTypes::UntilIterZero,
            n => panic!("{} is not a valid option", n)
        }
    }
    
// In
    // Method for input, runs appropriate method depending on passed option
    pub fn input(stack: &mut Vec<u8>, lines: &Vec<String>, code_pointer: &usize) {
        let temp = lines[*code_pointer].split(" ").collect::<Vec<&str>>();
        match temp[1] {
            "@n" => input_n(stack),
            "@c" => input_c(stack),
            n => panic!("{} is not a valid option", n)
        }
    }
    // Method for taking user input
    fn take_input() -> Result<String, io::Error>{
        let mut buffer = String::new();
        let stdin = io::stdin();

        stdin.read_line(&mut buffer);
        Ok(buffer)
    }
    // Takes char input
    pub fn input_c(stack: &mut Vec<u8>) {
        'get_input: loop {
            print!("Input char: ");
            io::stdout().flush().unwrap();
            let mut output = take_input().unwrap();
            output = String::from(output.strip_suffix("\r\n").unwrap());
            println!("{}", output.chars().collect::<Vec<char>>()[0] as u8);
            match Some(output.chars().collect::<Vec<char>>()[0]) {
                Some(n) => {stack.push(n as u8); break 'get_input;},
                _ => ()
            }
        }
    }
    // Takes number input
    pub fn input_n(stack: &mut Vec<u8>) {
        'get_input: loop {
            print!("Input number: ");
            io::stdout().flush().unwrap();
            let mut output = take_input().unwrap();
            output = String::from(output.strip_suffix("\r\n").unwrap());
            println!("{}", output.chars().collect::<Vec<char>>()[0] as u8);
            match output.parse::<u8>() {
                Ok(n) => {stack.push(n); break 'get_input;},
                _ => ()
            }
        }
    }
// Out
    // Method for output, runs appropriate method depending on passed option
    pub fn out(stack: &mut Vec<u8>, lines: &Vec<String>, code_pointer: &usize) {
        let temp = lines[*code_pointer].split(" ").collect::<Vec<&str>>();
        match temp[1] {
            "@n" => out_n(stack),
            "@c" => out_c(stack),
            n => panic!("{} is not a valid option", n)
        }
    }
    // Method for outputting chars
    pub fn out_c(stack: &mut Vec<u8>){
        print!("{}", match stack.len() {0 => None, n => Some(stack[n-1])}.unwrap() as char);
        stack.remove(stack.len()-1);
    }
    // Method for outputting numbers
    pub fn out_n(stack: &mut Vec<u8>){
        print!("{}", match stack.len() {0 => None, n => Some(stack[n-1])}.unwrap());
        stack.remove(stack.len()-1);
    }
// Math
    pub fn div(stack: &mut Vec<u8>, lines: &Vec<String>, code_pointer: &usize){
        let temp = lines[*code_pointer].split(" ").collect::<Vec<&str>>();
        let (x, y) = parse_vec_ref_str_to_2_tuple_u8(temp);
        stack.push(Math::divide(x,y).unwrap());
    }
// Pushing to stack
    pub fn push(stack: &mut Vec<u8>, lines:&Vec<String>, code_pointer: &usize) {
        let temp = lines[*code_pointer].split(" ").collect::<Vec<&str>>();

        match temp[1].parse::<u8>() {
            Ok(n) => stack.push(n),
            Err(n) => println!("{}", n),
        };
    }

    pub fn pushb(stack: &mut Vec<u8>, lines:&Vec<String>, code_pointer: &usize) -> Vec<u8> {
        let temp = lines[*code_pointer].split(" ").collect::<Vec<&str>>();

        match temp[1].parse::<u8>() {
            Ok(n) => stack.insert(0, n),
            Err(_) => match temp[1] {
                "@t" => {
                    let mut temp_stack = vec![stack[0]];
                    temp_stack.append(stack);
                    return temp_stack
                },
                n => println!("{}", n)
            },
        };
        return stack.to_owned()
    }
//