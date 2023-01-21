mod db;
mod sql;
mod util;

pub struct Config {
    pub user: String,
    pub password: String,
    pub host: String
}

pub enum DBType {
    MySql,
    PostgresSql
}

pub use sql::setup_my_sql;
pub use sql::setup_progres_sql;

pub fn setup(db_type: DBType, config: Config) {
    match db_type {
        DBType::MySql => setup_my_sql(config),
        DBType::PostgresSql => setup_progres_sql(config)
    }
}

