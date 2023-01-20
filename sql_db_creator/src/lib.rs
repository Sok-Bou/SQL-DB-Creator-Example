#[path = "creator_mysql.rs"]
mod creator_mysql;

#[path = "creator_postgres_sql.rs"]
mod creator_postgres_sql;

pub use creator_mysql::{ ConfigMySql };
pub use creator_postgres_sql::{ ConfigPostgresSql };

pub fn create_mysql(config: ConfigMySql) {
    creator_mysql::setup(config);
}

pub fn create_postgres_sql(config: ConfigPostgresSql) {
    creator_postgres_sql::setup(config);
}
