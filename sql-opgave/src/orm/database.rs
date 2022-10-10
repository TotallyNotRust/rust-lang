use rusqlite::Row;

pub trait DatabaseTable {
    fn tableName() -> String;
    fn fromRow(row: &Row) -> Self;
}
