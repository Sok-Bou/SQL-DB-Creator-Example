use crate::creator::db::{ DBs, DB, Table, reduce_str };

use futures::executor::block_on;
use serde_json::{ Value };

use sqlx::postgres::PgPool;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Error;

use sqlx::postgres::PgQueryResult;

pub struct ConfigPostgresSql {
    pub user: String,
    pub password: String,
    pub host: String
}

async fn create_connection(config: &ConfigPostgresSql) -> Result<Pool<Postgres>, Error> {
    let user = &config.user;
    let password = &config.password;
    let host = &config.host;

    let url = format!("postgres://{user}:{password}@{host}");

    PgPool::connect(&url).await
}

async fn create_db(name: &str, pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
    let query = format!("CREATE DATABASE {name}");
    sqlx::query(&query).execute(pool).await
}

async fn drop_db(name: &str, pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
    let query = format!("DROP DATABASE IF EXISTS {name}");
    sqlx::query(&query).execute(pool).await
}

async fn create_pool_for_db(config: &ConfigPostgresSql, db_name: &str) -> Result<Pool<Postgres>, Error> {
    let user = &config.user;
    let password = &config.password;
    let host = &config.host;

    let url = format!("postgres://{user}:{password}@{host}/{db_name}");

    PgPool::connect(&url).await
}

fn create_pools_for_dbs<'a>(config: &ConfigPostgresSql, dbs: &'a DBs, pool: &Pool<Postgres>) -> Vec::<(Pool<Postgres>, &'a DB)> {
    let mut pools: Vec::<(Pool<Postgres>, &DB)> = Vec::new();

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

async fn create_table(pool: &Pool<Postgres>, table: &Table) -> Result<PgQueryResult, Error> {
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

    println!("{query}");

    sqlx::query(&query).execute(pool).await
}

async fn create_table_data(pool: &Pool<Postgres>, table: &Table) -> Result<PgQueryResult, Error> {
    let mut column_names: Vec<&str> = Vec::new();
    let mut query = String::from("INSERT INTO ");
    query.push_str(&table.name);
    query.push_str(" (");

    let scheme = &table.scheme;
    for (key, _) in scheme {

        let line = format!("{}, ", key);
        query.push_str(&line);
        column_names.push(&key);
    }
    
    let mut query = reduce_str(&query, 0, 2);

    query.push_str(") VALUES ");

    for data_set in &table.data {
        query.push_str("(");
        for name in &column_names {
            for (key, value) in data_set {
                if key == name {
                    let mut value_new_string = String::from("");
    
                    match value {
                        Value::Null => println!("Null"),
                        Value::Bool(b) => {
                            value_new_string.push_str(&b.to_string());
                        },
                        Value::Number(number) => {
                            value_new_string.push_str(&number.to_string());
                        },
                        Value::String(string) => {
                            value_new_string.push('\'');
                            value_new_string.push_str(string);
                            value_new_string.push('\'');
                        },
                        Value::Array(value) => println!("value: {:?}", value),
                        Value::Object(obj) => println!("obj: {:?}", obj)
                    }
        
                    let line = format!("{}, ", value_new_string);
                    query.push_str(&line);
                }
            }
        }

        let mut query_new = reduce_str(&query, 0, 2);
        query_new.push_str("), ");

        query = query_new;
    }
    
    let mut query = reduce_str(&query, 0, 2);
    query.push_str(";");

    sqlx::query(&query).execute(pool).await
}

pub fn setup(config: ConfigPostgresSql) {
    let dbs = DBs::new();

    //dbs.print_db();

    let connection_pool_future_result = create_connection(&config);

    match block_on(connection_pool_future_result) {
        Ok(connection_pool) => {

            for db in &dbs.dbs {
                if let Err(e) = block_on(drop_db(&db.name, &connection_pool)) {
                    println!("Database \"{}\" couldn't be dropped", &db.name);
                    println!("{:?}", e);
                } else {
                    println!("Database \"{}\" dropped if existed", &db.name);
                }
            }
            
            let pools = create_pools_for_dbs(&config, &dbs, &connection_pool);
            for (pool, db) in pools {

                let db_name = &db.name;
                let tables = &db.tables;

                for table in tables {
                    
                    let table_name = &table.name;

                    let table_result = create_table(&pool, &table);

                    if let Err(e) = block_on(table_result) {
                        println!("Table with name \"{}\" couldn't be created.", table_name);
                        println!("{:?}", e);
                    } else {
                        println!("New Table with name \"{}\" created in Database \"{}\".", table_name, db_name);

                        let data_result = create_table_data(&pool, &table);

                        if let Err(e) = block_on(data_result) {
                            println!("Table \"{}\" of Database \"{}\" couldn't be filled with datasets.", table_name, db_name);
                            println!("{:?}", e);
                        } else {
                            println!("Table \"{}\" of Database \"{}\" successfully filled with datasets.", table_name, db_name);
                        }
                    }
                }
            }
        },
        Err(e) => {
            println!("One or mor pools couldn't be created");
            println!("{:?}", e);
        }
    }
}