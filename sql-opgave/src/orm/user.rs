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
    fn fromRow(row: &rusqlite::Row) -> Self {
        return User {
            id: row.get(0).unwrap(),
            username: row.get(1).unwrap(),
        }
    }
}
