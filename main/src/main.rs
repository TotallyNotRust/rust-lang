use std::io;
use strum_macros;

#[derive(strum_macros::ToString, Debug)]
enum Gender {
    Male,
    Female,
    NonBinary,
}

struct People {
    people: Vec<Person>,
}

impl People {
    fn print_people(&self) {
        for i in &self.people {
            println!("{}", i.get_name());
        }
    }
}

struct Person {
    age: i32,
    name: String,
    gender: Gender,
}

impl Person{
    fn get_age(&self) -> i32 {
        return self.age;
    }
    fn get_name(&self) -> &String {
        return &self.name;
    }
    fn get_gender(&self) -> String {
        return self.gender.to_string();
    }
}

fn main() {
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");
    let age: usize = age 
                      .trim()
                      .parse()
                      .expect("Not number\n");            
    let person_male = Person{age: age as i32, name: String::from("Jørgen"), gender: Gender::Male};
    let person_female = Person{age: age as i32, name: String::from("Jørgenessa"), gender: Gender::Female};
    let age: i32 = person_male.get_age();
    let name: &String = person_male.get_name();
    let gender: String = person_male.get_gender();
    println!("{}", age);
    println!("{}", name);
    println!("{}", gender);
}