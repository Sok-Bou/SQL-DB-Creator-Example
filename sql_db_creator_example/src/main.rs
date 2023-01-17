mod secure;

use sql_db_creator::{ Config, create_mysql };
use secure::Credentials;

fn main() {

    let credentials = Credentials::new();

    let config = Config {
        user: credentials.user,
        password: credentials.password,
        host: credentials.host
    };

    create_mysql(config);
}
