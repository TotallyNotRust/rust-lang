use std::collections::HashMap;

use self::database::DatabaseTable;
use rusqlite::{Connection, Statement};

pub mod database;
pub mod user;

pub struct ORM {
    database: Connection,
}

impl ORM {
    pub fn from_url(database_url: &str) -> Result<ORM, rusqlite::Error> {
        match Connection::open(database_url) {
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
            .prepare(format!("SELECT * FROM {:?}", T::table_name()).as_str())
        {
            Ok(n) => n,
            Err(_) => return vec![],
        };

        let result = match statement.query_map([], |row| Ok(T::from_row(row))) {
            Ok(n) => n,
            Err(_) => return vec![],
        };

        let mut parsed: Vec<T> = vec![];

        for user in result {
            match user {
                Ok(n) => parsed.push(n),
                Err(_) => return vec![],
            }
        }

        return parsed;
    }
    pub fn insert<T: DatabaseTable>(&self, item: T) -> Result<(), ()> {
        let fields = item.get_values();

        let mut keys = vec![];
        let mut values: Vec<String> = vec![];

        for (k, v) in fields {
            keys.push(k);
            values.push(v);
        }

        let mut sql = format!(
            "INSERT INTO {} {:?} VALUES {:?}",
            T::table_name(),
            keys,
            values,
        );

        sql = sql.replace("[", "(").replace("]", ")").replace("\"", "");

        let statement = self.database.execute(sql.as_str(), []);

        println!("{:?}\n{:?}", statement, sql);

        match statement {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
