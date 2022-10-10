use std::any::type_name;

use self::{database::DatabaseTable, user::User};
use sqlite::{Error, State};

mod database;
pub mod user;

pub struct ORM {
    database: sqlite::Connection,
}

impl ORM {
    pub fn fromUrl(databaseUrl: &str) -> Result<ORM, sqlite::Error> {
        match (sqlite::open(databaseUrl)) {
            Ok(connection) => {
                return Ok(ORM {
                    database: connection,
                })
            }
            Err(n) => return Err(n),
        }
    }

    pub fn select<T: DatabaseTable>(&self) -> Vec<T> {
        final result = match self.database
            .prepare(format!("SELECT * FROM {:?}", T::tableName())) {
                Ok(n) => return n,
                Err(_) => return vec![],
            };

        //
        return self.database.;
    }
}
