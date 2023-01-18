#[path = "dbs.rs"]
mod dbs;

use dbs::DBs;
use dbs::DB;
use dbs::Table;

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

async fn drop_db(name: &str, pool: &Pool<MySql>) -> Result<MySqlQueryResult, Error> {
    let query = format!("DROP DATABASE IF EXISTS {name}");
    sqlx::query(&query).execute(pool).await
}

async fn create_pool_for_db(config: &Config, db_name: &str) -> Result<Pool<MySql>, Error> {
    let user = &config.user;
    let password = &config.password;
    let host = &config.host;

    let url = format!("mysql://{user}:{password}@{host}/{db_name}");

    MySqlPool::connect(&url).await
}

fn create_pools_for_dbs<'a>(config: &Config, dbs: &'a DBs, pool: &Pool<MySql>) -> Vec::<(Pool<MySql>, &'a DB)> {
    let mut pools: Vec::<(Pool<MySql>, &DB)> = Vec::new();

    let dbs = &dbs.dbs;
    for db in dbs {
        match block_on(create_db(&db.name, &pool)) {
            Ok(_) => {
                println!("New Database with name \"{}\" created", &db.name);
                match block_on(create_pool_for_db(config, &db.name)) {
                    Ok(pool) => {
                        pools.push((pool, db));
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

async fn create_table(pool: &Pool<MySql>, table: &Table) -> Result<MySqlQueryResult, Error> {
    let mut query = String::from("CREATE TABLE ");
    query.push_str(&table.name);
    query.push_str(" (");

    let scheme = &table.scheme;
    for (key, value) in scheme {
        let value_new = &value[1..value.len() - 1];

        let line = format!("{} {}, ", key, value_new);
        query.push_str(&line);
    }

    let query_str = &query;
    let query_str_new = &query_str[0..query_str.len() - 2];
    
    let mut query = String::from(query_str_new);

    query.push_str(");");

    sqlx::query(&query).execute(pool).await
}

async fn create_table_data(pool: &Pool<MySql>, table: &Table) -> Result<MySqlQueryResult, Error> {
    let mut query = String::from("INSERT INTO ");
    query.push_str(&table.name);
    query.push_str("(");

    let scheme = &table.scheme;
    for (key, _) in scheme {

        let line = format!("{}, ", key);
        query.push_str(&line);
    }

    let query_str = &query;
    let query_str_new = &query_str[0..query_str.len() - 2];
    
    let mut query = String::from(query_str_new);

    query.push_str(") VALUES ");

    for data_set in &table.data {
        query.push_str("(");
        for (_, value) in data_set {

            let value_new = &value[1..value.len() - 1];

            let line = format!("{}, ", value_new);
            query.push_str(&line);
        }
        let query_str = &query;
        let query_str_new = &query_str[0..query_str.len() - 2];

        let mut query_new = String::from(query_str_new);

        query_new.push_str("), ");

        query = query_new;
    }

    let query_str = &query;
    let query_str_new = &query_str[0..query_str.len() - 2];
    
    let mut query = String::from(query_str_new);

    query.push_str(";");

    println!("{query}");

    //return Err(Error::PoolClosed);

    sqlx::query(&query).execute(pool).await
}

pub fn setup(config: Config) {
    let dbs = DBs::new();

    //dbs.print_db();

    let pool_future_result = create_connection(&config);

    match block_on(pool_future_result) {
        Ok(pool) => {

            for db in &dbs.dbs {
                match block_on(drop_db(&db.name, &pool)) {
                    Ok(_) => {
                        println!("DB droped",);
                    },
                    Err(e) => println!("{:?}", e)
                }
            }
            
            let pools = create_pools_for_dbs(&config, &dbs, &pool);
            for (pool, db) in pools {

                let tables = &db.tables;
                for table in tables {

                    let name = &table.name;
                    let table_result = create_table(&pool, &table);

                    match block_on(table_result) {
                        Ok(_) => {
                            println!("New Table with name {} created", name);

                            let data_result = create_table_data(&pool, &table);

                            match block_on(data_result) {
                                Ok(_) => {
                                    println!("New Data created in Table with name {} created", name);
                                },
                                Err(e) => println!("New Data in Table \"{}\" couldn't be created. Error: {}", name, e)
                            }
                        },
                        Err(e) => println!("Table with name \"{}\" couldn't be created. Error: {}", name, e)
                    }
                }
            }
        },
        Err(e) => println!("One or mor pools couldn't be created")
    }
}
