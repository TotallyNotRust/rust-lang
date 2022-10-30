use std::any::type_name;

use self::{database::DatabaseTable, user::User};
use rusqlite::{Connection, Statement, NO_PARAMS};

mod database;
pub mod user;

pub struct ORM {
    database: Connection,
}

impl ORM {
    pub fn fromUrl(databaseUrl: &str) -> Result<ORM, rusqlite::Error> {
        match Connection::open(databaseUrl) {
            Ok(connection) => {
                return Ok(ORM {
                    database: connection,
                })
            }
            Err(n) => return Err(n),
        }
    }

    pub fn select<T: DatabaseTable>(&self) -> Vec<T> {
        let mut statement: Statement = match self
            .database
            .prepare(format!("SELECT * FROM {:?}", T::tableName()).as_str())
        {
            Ok(n) => n,
            Err(_) => return vec![],
        };

        let result = match statement.query_map([], |row| Ok(T::fromRow(row))) {
            Ok(n) => n,
            Err(_) => return vec![],
        };

        let mut parsed: Vec<T> =vec![];

        for user in result {
            match user {
                Ok(n) => parsed.push(n),       
                Err(_) => return vec![],
            }
        }

        return parsed;
    }
}
