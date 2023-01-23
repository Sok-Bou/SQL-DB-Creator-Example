use sql_db_creator::{ DBType, Config, setup };

fn main() {

    let config = Config {
        user: String::from("root"),
        password: String::from("SEMFE_2003"),
        host: String::from("localhost")
    };

    setup(DBType::MySql, config);
}
