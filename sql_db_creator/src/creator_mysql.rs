#[path = "db.rs"]
mod db;

#[path = "dbs.rs"]
mod dbs;

use db::DB;
use dbs::DBs;

use futures::executor::block_on;

use sqlx::mysql::MySqlPool;
use sqlx::Pool;
use sqlx::MySql;
use sqlx::Error;

use sqlx::mysql::MySqlQueryResult;
// use sqlx::postgres::PgPoolOptions;

pub struct Config {
    pub user: String,
    pub password: String,
    pub host: String
}

async fn create_connection(config: &Config) -> Result<Pool<MySql>, Error> {
    let user = &config.user;
    let password = &config.password;
    let host = &config.host;

    let url = format!("mysql://{user}:{password}@{host}");

    MySqlPool::connect(&url).await
}

async fn create_db(name: &str, pool: &Pool<MySql>) -> Result<MySqlQueryResult, Error> {
    let query = format!("CREATE DATABASE IF NOT EXISTS {name}");
    sqlx::query(&query).execute(pool).await
}

async fn create_pool_for_db(config: &Config, db_name: &str) -> Result<Pool<MySql>, Error> {
    let user = &config.user;
    let password = &config.password;
    let host = &config.host;

    let url = format!("mysql://{user}:{password}@{host}/{db_name}");

    MySqlPool::connect(&url).await
}

fn create_dbs(config: &Config, dbs: &DBs, pool: &Pool<MySql>) -> Vec::<Pool<MySql>> {
    let mut pools: Vec::<Pool<MySql>>  = Vec::new();

    let dbs = &dbs.dbs;
    for db in dbs {
        match block_on(create_db(&db.name, &pool)) {
            Ok(_) => {
                println!("New Database with name \"{}\" created", &db.name);
                match block_on(create_pool_for_db(config, &db.name)) {
                    Ok(pool) => {
                        pools.push(pool);
                    },
                    Err(e) => {
                        println!("Error: {e}");
                        println!("Something went wron by trying to create the pool for the \"{}\" database", &db.name);
                    }
                }
            },
            Err(e) => {
                println!("Error: {e}");
                println!("Database with name \"{}\" couldn't be created", &db.name);
            }
        }
    }

    pools
}

// fn create_table(pool: &Pool<MySql>) {

// }

pub fn setup(config: Config) {
    let dbs = DBs::new();

    //dbs.print_db();

    let pool_future_result = create_connection(&config);

    match block_on(pool_future_result) {
        Ok(pool) => {
            let pools = create_dbs(&config, &dbs, &pool);

            //println!("Connection to Database established");
        },
        Err(e) => println!("One or mor pools couldn't be created")
    }
}
