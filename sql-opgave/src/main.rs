use orm::database::DatabaseTable;
use orm::{user::User, ORM};
use rusqlite::types::Type;
use rusqlite::DatabaseName;
mod orm;
use std::any::type_name;
use std::collections::HashMap;
use std::io::{self, Write};
use std::mem::take;

fn main() -> ! {
    let database: ORM = match ORM::from_url("database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    loop {
        println!("What do you want to do? (Enter action id)");
        println!("1: View database");
        println!("2: Insert to database");
        println!("---------");
        let input = match take_input_single() {
            Ok(n) => n,
            Err(_) => continue,
        };
        match input.as_str() {
            "1" => view_database(&database),
            "2" => insert_database(&database),
            n => {
                println!("Invalid input {:?}!", n);
                continue;
            }
        }
    }
}

fn take_input_single() -> Result<String, ()> {
    match take_input() {
        Ok(n) => {
            return Ok(String::from_utf8(vec![n.as_bytes()[0]]).expect("Could not parse input"))
        }
        Err(_) => return Err(()),
    }
}

fn take_input() -> Result<String, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    match stdin.read_line(&mut buffer) {
        Ok(_) => {
            return Ok(buffer);
        }
        Err(_) => return Err(()),
    }
}

fn view_database(database: &ORM) {
    let users: Vec<User> = database.select::<User>();

    println!("{:?}", users);
}

fn insert_database(orm: &ORM) {
    println!("Input what type of object you want to insert");
    println!("1: User");
    'constructLoop: loop {
        loop {
            let input = match take_input_single() {
                Ok(n) => n,
                _ => continue,
            };

            match input.as_str() {
                "1" => {
                    construct_insert::<User>(orm);
                    break 'constructLoop;
                }
                _ => {
                    println!("Invalid input");
                    continue;
                }
            }
        }
    }
}

fn construct_insert<'a, T: DatabaseTable>(orm: &ORM) {
    let keys = T::get_fields();

    let mut values: HashMap<String, String> = HashMap::new();

    for key in keys {
        print!("Input value for {}: ", key);
        io::stdout().flush();
        let input: String = take_input().expect("").replace("\r\n", "");

        println!("{:?}: {:?}", key, input);

        values.insert(key, input);
    }

    let item = T::from_map(values);

    orm.insert::<T>(item);
}
