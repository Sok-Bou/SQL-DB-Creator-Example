#[path = "creator_mysql.rs"]
mod creator_mysql;

mod database;

pub use creator_mysql::{ DBType, Config };

// pub fn create(db_type: DBType, config: Config) {

//     creator_mysqlsetup(db_type, config);
// }

pub fn create_mysql(config: Config) {

    creator_mysql::setup(config);
}
