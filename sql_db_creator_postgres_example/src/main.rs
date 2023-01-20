mod secure;

use sql_db_creator::{ ConfigPostgresSql, create_postgres_sql };
use secure::Credentials;

fn main() {

    let credentials = Credentials::new();

    let config = ConfigPostgresSql {
        user: credentials.user,
        password: credentials.password,
        host: credentials.host
    };

    create_postgres_sql(config);
}
