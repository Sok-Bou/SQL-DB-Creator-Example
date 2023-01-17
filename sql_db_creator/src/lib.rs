#[path = "creator_mysql.rs"]
mod creator_mysql;

pub use creator_mysql::{ Config };

pub fn create_mysql(config: Config) {
    creator_mysql::setup(config);
}
