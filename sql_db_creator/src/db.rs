#[path = "util.rs"]
mod util;

use util::sub_paths;
use util::get_last_of_split;
use util::get_first_of_split;

use std::fs;
use std::collections::HashMap;

use serde_json::{ Value };

pub struct Table {
    pub name: String,
    pub path: String,
    pub scheme: HashMap<String, String>,
    pub data: Vec<HashMap<String, String>>
}

pub struct DB {
    pub name: String,
    pub tables: Vec<Table>
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn create_scheme(content: &str) -> HashMap<String, String> {
    let mut scheme: HashMap<String, String> = HashMap::new();

    scheme
}

fn create_data(content: &str) -> Vec<HashMap<String, String>> {
    let mut data: Vec<HashMap<String, String>> = Vec::new();

    let value_hashmap: HashMap<String, Value> = serde_json::from_str(content).unwrap();

    for (key, value) in value_hashmap {
        match value {
            Value::Null => println!("key: {key}, value: Null"),
            Value::Bool(b) => println!("key: {key}, value (bool): {b}"),
            Value::Number(number) =>  println!("key: {key}, value (number): {number}"),
            Value::String(string) =>  println!("key: {key}, value (string): {string}"),
            Value::Array(array) => {
                if key == "data" {
                    for a in array {    
                        let v: HashMap<String, Value> = serde_json::from_value(a).unwrap();
                        let mut data_set: HashMap<String, String> = HashMap::new();

                        for (key, value) in v {
                            data_set.insert(key, value.to_string());
                        }

                        data.push(data_set);
                    }
                }
            },
            Value::Object(obj) => println!("key: {key}, value (obj): {:?}", obj)
        }
    }

    data
}

impl DB {
    pub fn new(db_name_path: &str) -> Self {

        let name = match get_last_of_split(db_name_path, "/") {
            Some(name) => name,
            None => ""
        };

        let mut tables: Vec::<Table> = Vec::new();

        let table_name_paths = match sub_paths(&db_name_path) {
            Ok(paths) => paths,
            Err(_) => return DB {
                name: String::from(""),
                tables: Vec::new()
            }
        };

        for table_name_path in table_name_paths {

            let mut scheme: HashMap<String, String> = HashMap::new();
            let mut data: Vec<HashMap<String, String>> = Vec::new();
            match fs::read_to_string(&table_name_path) {
                Ok(content) => {
                    println!("{content}");
                    scheme = create_scheme(&content);
                    data = create_data(&content);
                },
                Err(e) => println!("{e}")            
            }
            
            match get_last_of_split(&table_name_path, "/") {
                Some(name) => {

                    match get_first_of_split(name, ".") {
                        Some(name) => tables.push(Table {
                            name: name.to_string(),
                            path: table_name_path,
                            scheme: scheme,
                            data: data
                        }),
                        None => tables.push(Table {
                            name: String::from(""),
                            path: String::from(""),
                            scheme: scheme,
                            data: data
                        })
                    }
                },
                None => tables.push(Table {
                    name: String::from(""),
                    path: String::from(""),
                    scheme: scheme,
                    data: data
                })
            };
        }

        DB {
            name: String::from(name),
            tables: tables
        }
    }
}

struct NewExport {
    first_name: String,
    last_name: String,
    gender: String,
    date_of_birth: String,
    address: String
}

struct Export {
    first_name: String,
    last_name: String,
    gender: String,
    date_of_birth: String,
    address: NewExport
}