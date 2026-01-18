use std::collections::HashMap;
use std::io;

type Table = HashMap<String, String>;
type Database = HashMap<String, Table>;

fn main() {
    let mut database: Database = HashMap::new();
    menu(&mut database);
}

fn menu(database: &mut Database) {
    println!("=======================================");
    println!("FQL - Rust Based File Query Language");
    println!("=======================================");
    println!("1 > Create a database");
    println!("2 > Log into database");

    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("No option found");

    let option = option.trim().to_string();

    if option == "1" {
        create_database(database);
    } else {
        login_database(database);
    }
}

fn create_database(database: &mut Database) {
    println!("Enter your database name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("No database name found");
    let name: String = name.trim().to_string();
    database.insert(name, Table::new());
    println!("Database created successfully");
    menu(database);
}

fn login_database(database: &mut Database) {
    println!("Enter your database name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("No database name found");
    let name: String = name.trim().to_string();
    if database.contains_key(&name) {
        loop {
            println!("> ");
            let mut option = String::new();
            io::stdin().read_line(&mut option).expect("No option found");
            let mut parts = option.trim().split_whitespace();

            let command = parts.next();
            let key = parts.next();
            let value = parts.next();

            let table = database.get_mut(&name);

            if command == Some("add") {
                add_to_database(table, key, value);
            } else if command == Some("get") {
                get_from_database(table, key);
            }
            else if command == Some("exit") {
                break;
            }
        }
    } else {
        println!("Database not found");
    }
    menu(database);
}

fn add_to_database(table: Option<&mut Table>, key: Option<&str>, value: Option<&str>) {
    if let (Some(table), Some(key), Some(value)) = (table, key, value) {
        table.insert(key.to_string(), value.to_string());
    }
}

fn get_from_database(table: Option<&mut Table>, key: Option<&str>) {
    if let (Some(table), Some(key)) = (table, key) {
        println!("{}: {}", key, table.get(key).unwrap());
    }
}
