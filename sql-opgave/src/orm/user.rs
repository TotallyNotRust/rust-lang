use super::database::DatabaseTable;

#[derive(Debug)]
pub struct User {
    id: i32,
    username: String,
}
impl DatabaseTable for User {
    fn tableName() -> String {
        return String::from("Users");
    }
}
