use orm::{user::User, ORM};
mod orm;

fn main() {
    let database: ORM = match ORM::fromUrl("database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}",n),
    };
    let users: Vec<User> = database.select::<User>();

    println!("{:?}", users);
}
