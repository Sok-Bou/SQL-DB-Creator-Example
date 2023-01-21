use sql_db_creator::{ DBType, Config, setup };

fn main() {

    let config = Config {
        user: String::from("postgres"),
        password: String::from("admin"),
        host: String::from("localhost")
    };

    setup(DBType::PostgresSql, config);
}
