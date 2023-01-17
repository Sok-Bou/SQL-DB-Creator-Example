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

fn db_url(config: &Config, db_name: Option<&str>) -> String {
    let user = &config.user;
    let password = &config.password;
    let host = &config.host;

    match db_name {
        Some(db_name) => return format!("mysql://{user}:{password}@{host}/{db_name}"),
        None => return format!("mysql://{user}:{password}@{host}")
    }
}

async fn create_db(name: &str, pool: &Pool<MySql>) -> Result<MySqlQueryResult, Error> {
    let query = format!("CREATE DATABASE IF NOT EXISTS {name}");
    sqlx::query(&query).execute(pool).await
}

async fn create_pool(config: &Config, db_name: Option<&str>) -> Result<Pool<MySql>, Error> {
    let url = db_url(config, db_name);

    MySqlPool::connect(&url).await
}

fn create_pools(config: &Config, dbs: &DBs, pool: &Pool<MySql>) -> Vec::<Pool<MySql>> {
    let mut pools: Vec::<Pool<MySql>>  = Vec::new();

    let dbs = &dbs.dbs;
    for db in dbs {
        match block_on(create_db(&db.name, &pool)) {
            Ok(_) => {
                println!("Database created: {}", &db.name);

                match block_on(create_pool(config, Some(&db.name))) {
                    Ok(pool) => pools.push(pool),
                    Err(e) => println!("Something went wron by trying to create a pool: {e}")
                }
            },
            Err(e) => println!("Database could't be created: {e}")
        }
    }

    pools
}

pub fn setup(config: Config) {
    let dbs = DBs::new();

    //dbs.print_db();

    let pool_future_result = create_pool(&config, None);

    match block_on(pool_future_result) {
        Ok(pool) => {
            let pools = create_pools(&config, &dbs, &pool);
        },
        Err(e) => println!("Pool could't be created: {e}")
    }
}
