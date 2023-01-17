#[path = "database.rs"]
mod database;

#[path = "util.rs"]
mod util;

use database::DB;
use util::sub_paths;

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

pub enum DBType {
    MySql,
    SQLight,
    PostgreSQL
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

async fn create_pool(config: &Config, db_name: Option<&str>) -> Result<Pool<MySql>, Error> {
    let url = db_url(config, db_name);

    MySqlPool::connect(&url).await
}

fn create_pools(config: &Config, dbs: &Vec<DB>, pool: &Pool<MySql>) -> Vec::<Pool<MySql>> {
    let mut pools: Vec::<Pool<MySql>>  = Vec::new();

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

async fn create_db(name: &str, pool: &Pool<MySql>) -> Result<MySqlQueryResult, Error> {
    let query = format!("CREATE DATABASE IF NOT EXISTS {name}");
    sqlx::query(&query).execute(pool).await
}

pub fn setup(db_type: DBType, config: Config) {
    let db_name_paths = match sub_paths("./src/db/") {
        Ok(paths) => paths,
        Err(e) => {
            panic!("{e}");
        }
    };

    let mut dbs: Vec<DB> = Vec::new();

    for db_name_path in db_name_paths {
        dbs.push(DB::new(&db_name_path))
    }

    print_db(&dbs);

    let pool_future_result = create_pool(&config, None);

    match block_on(pool_future_result) {
        Ok(pool) => {
            let pools = create_pools(&config, &dbs, &pool);
        },
        Err(e) => println!("Pool could't be created: {e}")
    }
}

fn print_db(dbs: &Vec<DB> ) {
    for db in dbs {
        println!("{}", db.name);

        let tables = &db.tables;

        for table in tables {
            println!("{}", table.name);
            println!("{}", table.path);
        }
    }
}