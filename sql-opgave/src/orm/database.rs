use std::collections::HashMap;

use rusqlite::Row;

pub trait DatabaseTable {
    fn table_name() -> String;
    fn from_row(row: &Row) -> Self;
    fn to_string(self: &Self) -> String;
    fn get_values(self: &Self) -> HashMap<String, String>;
    fn get_fields() -> Vec<String>;
    fn empty() -> Self;
    fn table_name_from_instance(&self) -> String {
        return Self::table_name();
    }
    fn from_map(map: HashMap<String, String>) -> Self;
}
