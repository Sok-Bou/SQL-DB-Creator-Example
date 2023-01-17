mod sql_db_creator;

pub use sql_db_creator:: { DBType, Config };
use sql_db_creator:: { setup };

pub fn create(config: Config) {
    
    setup(DBType::MySql, Some(config));
}

