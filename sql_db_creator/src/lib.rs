#[path = "creator_mysql.rs"]
mod creator_mysql;

mod database;

pub use creator_mysql::{ Config };

pub fn create_mysql(config: Config) {

    creator_mysql::setup(config);
}
