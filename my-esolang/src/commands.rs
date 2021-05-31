mod math;
use math::Math;

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

pub fn out(stack: &mut Vec<u8>){
    print!("{}", match stack.len() {0 => None, n => Some(stack[n-1])}.unwrap() as char);
    stack.remove(stack.len()-1);
}
pub fn div(stack: &mut Vec<u8>, lines: &Vec<String>, code_pointer: &usize){
    let temp = lines[*code_pointer].split(" ").collect::<Vec<&str>>();
    let (x, y) = parse_vec_ref_str_to_2_tuple_u8(temp);
    stack.push(Math::divide(x,y).unwrap());
}

pub fn push(stack: &mut Vec<u8>, lines:&Vec<String>, code_pointer: &usize) {
    let temp = lines[*code_pointer].split(" ").collect::<Vec<&str>>();

    match temp[1].parse::<u8>() {
        Ok(n) => stack.push(n),
        Err(n) => println!("{}", n),
    };
}