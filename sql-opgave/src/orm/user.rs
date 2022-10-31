use std::{collections::HashMap, hash::Hash};

use super::database::DatabaseTable;

#[derive(Debug)]
pub struct User {
    id: i32,
    username: String,
}
impl DatabaseTable for User {
    fn table_name() -> String {
        return String::from("Users");
    }
    fn from_row(row: &rusqlite::Row) -> Self {
        return User {
            id: row.get(0).unwrap(),
            username: row.get(1).unwrap(),
        };
    }

    fn to_string(&self) -> String {
        return format!("User {:?} goes by {:?}", self.id, self.username);
    }
    fn get_values(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        map.insert("id".to_string(), self.id.to_string());
        map.insert("username".to_string(), self.username.to_string());

        return map;
    }

    fn empty() -> User {
        User {
            id: -1,
            username: String::from(""),
        }
    }
    fn get_fields() -> Vec<String> {
        return vec![String::from("id"), String::from("username")];
    }
    fn from_map(map: HashMap<String, String>) -> Self {
        let id = match map.get::<String>(&String::from("id")) {
            None => panic!("Could not find value id"),
            Some(n) => match String::from(n).parse::<i32>() {
                Ok(n) => n,
                Err(_) => panic!("Could not parse id"),
            },
        };

        let username = match map.get::<String>(&String::from("username")) {
            None => panic!("Could not find value username"),
            Some(n) => String::from(n),
        };

        return User {
            id: id,
            username: username,
        };
    }
}
