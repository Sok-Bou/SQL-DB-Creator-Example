mod secure;

use sql_db_creator::{ create, Config, DBType };
use secure::Credentials;

fn main() {

    let credentials = Credentials::new();

    let config = Config {
        user: credentials.user,
        password: credentials.password,
        host: credentials.host
    };

    create(DBType::MySql, config);
}
