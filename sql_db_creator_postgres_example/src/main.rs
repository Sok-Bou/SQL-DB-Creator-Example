// mod secure;

// use sql_db_creator::{ ConfigPostgresSql, create_postgres_sql };
// use secure::Credentials;

// fn main() {

//     let credentials = Credentials::new();

//     let config = ConfigPostgresSql {
//         user: credentials.user,
//         password: credentials.password,
//         host: credentials.host
//     };

//     create_postgres_sql(config);
// }

mod secure;
use secure::Credentials;
use sql_db_creator::{ ConfigPostgresSql, setup };

fn main() {

    let credentials = Credentials::new();

    let config = ConfigPostgresSql {
        user: credentials.user,
        password: credentials.password,
        host: credentials.host
    };

    setup(config);
}

// use sql_db_creator::{ module3_fu, call_module3_fn };

// fn main() {
//     module3_fu();
//     call_module3_fn();
// }