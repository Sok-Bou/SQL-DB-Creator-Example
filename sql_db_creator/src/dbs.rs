#[path = "db.rs"]
mod db;

#[path = "util.rs"]
mod util;

use util::sub_paths;
use db::DB;

pub struct DBs {
    pub dbs: Vec<DB>,
}

impl DBs {
    pub fn new() -> Self {
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

        Self {
            dbs: dbs
        }
    }

    pub fn print_db(&self) {
        let dbs = &self.dbs;
        for db in dbs {
            println!("{}", db.name);
    
            let tables = &db.tables;
    
            for table in tables {
                println!("{}", table.name);
                println!("{}", table.path);

                let scheme = &table.scheme;
                for (key, value) in scheme {
                    println!("    {key}, {value}");
                }

                let data = &table.data;
                for data_set in data {
                    for (key, value) in data_set {
                        println!("        {key}, {value}");
                    }
                }
            }
        }
    }
}
