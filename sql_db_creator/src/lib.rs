mod sql_db_creator;

pub use sql_db_creator:: { DBType, Config };
use sql_db_creator:: { setup };

pub fn create(db_type: DBType, config: Config) {

    setup(db_type, config);
}

