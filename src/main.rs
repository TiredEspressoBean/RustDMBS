//! # RustDBMS
//!
//! RustDBMS is an experimental database management system (DBMS) implemented in Rust.
//! This project aims to explore database concepts and provide a simple, educational example
//! of a database management system using Rust.
//!
//! ## Features
//!
//! - CLI Interface: Interact with the DBMS through a command-line interface.
//! - Collection Management: Create, read, update, and delete collections of records.
//! - Record Operations: Perform CRUD operations on individual records within collections.
//! - Data Persistence: Store and load data from a JSON file for persistence across sessions.
//!
//! ## Possible New Features
//!
//! As this project is an exploratory study by a CS student with limited production experience
//! in using or implementing fully-fledged DBMS systems, any suggestions or contributions
//! from the community are highly valued. Below are some ideas for potential new features:
//!
//! - **API Interface**: Implement a RESTful or GraphQL API interface to allow external applications
//!   to interact with the DBMS, making it accessible over the network. This feature is planned to be
//!   made.
//!
//! - **Node-Based Distribution**: Design a distributed architecture where the DBMS can
//!   run across multiple nodes, improving scalability and fault tolerance.
//!
//! - **SQL-Like Query Engine**: Develop a query engine capable of parsing and executing
//!   SQL or SQL-like queries, making the DBMS more versatile and user-friendly.
//!
//! - **Web Dashboard**: Create a web-based dashboard for visualizing data, monitoring system
//!   performance, and managing the DBMS, providing a user-friendly interface.
//!
//! - **User Permission and Authentication**: Introduce a user management system with
//!   role-based access control, allowing administrators to define permissions for different users.
//!
//! - **Automated Backup**: Implement features for automated backup and restore, ensuring data
//!   can be easily recovered in case of failures or data corruption.
//!
//! - **Key and Index-Based Collections**: Enhance the DBMS by introducing key-based collections
//!   and indexing mechanisms, improving data retrieval speed and query efficiency.
//!
//! - **Table-Like Structures**: Expand upon the concept of collections to include table-like
//!   structures, enabling more complex data relationships and queries.
//!
//! - **ACID Support**: Implement support for Atomicity, Consistency, Isolation, and Durability
//!   (ACID) transactions, ensuring data integrity and reliability during complex operations.
//!
//! - **Indexing**: Introduce indexing mechanisms such as B-trees or hash indexes to optimize
//!   data retrieval and improve query performance.
//!
//! ## Examples
//!
//! Below are some examples demonstrating how to use RustDBMS:
//!
//! **Creating a Collection:**
//!
//! ```sh
//! $ rustdbms-cli col create my_collection
//! ```
//!
//! **Adding a Record to a Collection:**
//!
//! ```sh
//! $ rustdbms-cli col read my_collection
//! ```
//!
//! **Listing All Collections:**
//!
//! ```sh
//! $ rustdbms-cli col list
//! ```
//!
//! **Deleting a Collection:**
//!
//! ```sh
//! $ rustdbms-cli col delete my_collection
//! ```
//!
//! **Updating a Collection:**
//!
//! ```sh
//! $ rustdbms-cli col update my_collection
//! ```
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rustdbms = "0.1.0"
//! ```
//!
//! To use the CLI tool, install it via Cargo:
//!
//! ```sh
//! cargo install rustdbms-cli
//! ```
//!
//! ## Getting Started
//!
//! To get started with RustDBMS, you can explore the CLI commands provided or integrate the library
//! into your own Rust project. For more detailed usage and documentation, refer to the [docs.rs page](https://docs.rs/rustdbms).
//!
//! ## Contributing
//!
//! Contributions are welcome! Please open issues or pull requests on the [GitHub repository](https://github.com/yourusername/rustdbms).
//!
//! ## License
//!
//! RustDBMS is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.


mod db;
mod utils;
mod API;

use log::trace;
use std::{env, io};
use std::sync::Arc;
use serde_json::Value::String;
use crate::db::schema::{Record, Value};
use crate::db::storage::{init_storage, StorageEngine};
use crate::utils::error::DBError;
use crate::utils::logger::init_logger;





/// Main core function
///
/// Spin up a storage engine, initialize the storage and as of currently go to a CLI to interact with
/// the database.
fn main() -> Result<(), DBError> {

    /// Init logging functionality
    init_logger();

    trace!("this is a trace");
    let storage = init_storage()?;

    if let Err(e) = storage.load_from_file("Db.json"){
        eprintln!("DB JSON not loaded to DB!");
    }


    // println!("Please enter \"CLI\" for command line operations");
    // let mut interface_input = std::string::String::new();
    // io::stdin().read_line(&mut interface_input).expect("Failed to read line");
    // let interface_input = interface_input.trim();
    // match  interface_input {
    //     "CLI" => {
    //         cli_interface(storage).expect("TODO: panic message");
    //     }
    //     _ => {}
    // }
    cli_interface(storage).expect("TODO: panic message");

    Ok(())
}
/// Looping CLI for the DBMS
///
/// Supported commands: \n\
///
/// col | collection list                                   List each collection in the database
///
/// col | collection read \<collection name\>                 List each record in the collection
///
/// col | collection create \<collection name\>               Create collection named \<collection name\>
///
/// col | collection delete \<collection name\>               Delete collection named \<collection name\>
///
/// col | collection update \<collection name\>               Update collection named \<collection name\>
///
/// rec | record create \<collection name\> \<record\>          Updates collection to include \<record\>
///
/// rec | record read \<collection name\> \<record index\>      Reads a record and prints it to the console
///
/// rec | record update \<collection name\> \<record index\>    Replaces a records information
///
/// rec | record delete \<collection name\> \<record index\>    Deletes the record at the record index
///
/// exit                                                    Exits the DBMS
///
/// save                                                    Saves the DBMS to Db.json
///
/// help                                                    Displays the supported commands
fn cli_interface(storage: Arc<StorageEngine>) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "\
Welcome to the DBMS CLI!\n\
Supported commands: \n\
col | collection list                                   List each collection in the database\n\
col | collection read <collection name>                 List each record in the collection\n\
col | collection create <collection name>               Create collection named <collection name>\n\
col | collection delete <collection name>               Delete collection named <collection name>\n\
col | collection update <collection name>               Update collection named <collection name>\n\
rec | record create <collection name> <record>          Updates collection to include <record>\n\
rec | record read <collection name> <record index>      Reads a record and prints it to the console\n\
rec | record update <collection name> <record index>    Replaces a records information \n\
rec | record delete <collection name> <record index>    Deletes the record at the record index \n\
exit                                                    Exits the DBMS \n\
save                                                    Saves the DBMS to Db.json \n\
help                                                    Displays the supported commands
    ");
    loop {
        // CLI interface implementation
        let mut input = std::string::String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        let args: Vec<&str> = input.split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        match args[0] {
            "exit" => {
                break Ok(())
            }
            "save" => {
                storage.save_to_file("Db.json").expect("Failed to save");
            }
            "help" => {
                println!(
                    "Supported commands: \n\
col | collection list                                   List each collection in the database\n\
col | collection read <collection name>                 List each record in the collection\n\
col | collection create <collection name>               Create collection named <collection name>\n\
col | collection delete <collection name>               Delete collection named <collection name>\n\
col | collection update <collection name>               Update collection named <collection name>\n\
rec | record create <collection name> <record>          Updates collection to include <record>\n\
rec | record read <collection name> <record index>      Reads a record and prints it to the console\n\
rec | record update <collection name> <record index>    Replaces a records information \n\
rec | record delete <collection name> <record index>    Deletes the record at the record index \n\
exit                                                    Exits the DBMS \n\
save                                                    Saves the DBMS to Db.json \n\
help                                                    Displays the supported commands"
                )
            }
            "rec" | "record" => {
                match args[1] {
                    "create" => {
                        if args.len() < 4 {
                            println!("Usage: rec create <collection name> <record>")
                        } else {
                            let collection_name = args[2];
                            let start_index = input.find(collection_name).unwrap();
                            let data = storage.parse_value(input[start_index+2..].trim());
                            let record = Record { values:vec![data?] };
                            match storage.create_record(collection_name, record) {
                                Ok(()) => println!("Added record to collection: {}", args[2]),
                                Err(e) => eprintln!("Unable to create new record")
                            }
                        }
                    }
                    "update" => {
                        if args.len() < 5 {
                            println!("Usage: rec update <collection_name> <index> <data>")
                        } else {
                            let collection_name = args[2];
                            let index = args[3].parse::<i32>().unwrap();
                            let start_index = input.find(collection_name).unwrap();
                            let data = storage.parse_value(input[start_index+2..].trim());
                            let record = Record { values:vec![data?] };
                            match storage.update_record(collection_name, index, record) {
                                Ok(record) => { println!("{:?}", record.values) }
                                Err(e) => eprintln!("{}", e)
                            }
                        }
                    }
                    "read" => {
                        if args.len() != 4 {
                            println!("Usage: rec read <collection_name> <index>")
                        } else {
                            let collection_name = args[2];
                            let index = args[3].parse::<i32>().unwrap();
                            match storage.read_record(collection_name, index) {
                                Ok(record) => { println!("{} - {:?}", index, record) }
                                Err(e) => eprintln!("{}", e)
                            }
                        }
                    }
                    "delete" => {
                        if args.len() != 4 {
                            println!("Usage: rec delete <collection_name> <index>")
                        } else {
                            let collection_name = args[2];
                            let index = args[3].parse::<i32>().unwrap();
                            match storage.delete_record(collection_name, index) {
                                Ok(record) => {println!("{} - {:?} has been deleted", index, record.values)}
                                Err(e) => eprintln!("{}", e)
                            }
                        }
                    }
                    _ => {
                        eprintln!("Unknown command: {}", args[1])
                    }
                }
            }
            "col" | "collection" => {
                match args[1] {
                    "create" => {
                        if args.len() != 3 {
                            println!("Usage: db create <collection_name>")
                        } else {
                            let collection_name = args[2];
                            match storage.add_collection(collection_name) {
                                Ok(_) => println!("Collection {} added!", collection_name),
                                Err(e) => eprintln!("Error while making {}: {}", collection_name, e)
                            }
                        }
                    }
                    "delete" => {
                        if args.len() != 3 { println!("Usage: db delete <collection_name>") } else {
                            let collection_name = args[1];
                            match storage.delete_collection(collection_name) {
                                Ok(_) => println!("Collection {} deleted!", collection_name),
                                Err(e) => eprintln!("Error while deleting {}: {}", collection_name, e),
                            }
                        }
                    }
                    "read" => {
                        if args.len() != 3 { println!("Usage: db read <collection_name>") } else {
                            let collection_name = args[2];
                            match storage.read_collection(collection_name) {
                                Ok(records) => {
                                    if records.is_empty() {
                                        println!("No records found in {}", collection_name);
                                    } else {
                                        for (i, record) in records.iter().enumerate() {
                                            println!("{} - {:?}", i + 1, record.values);
                                        }
                                    }
                                }
                                Err(e) => eprintln!("Error while retrieving {}: {}", collection_name, e)
                            }
                        }
                    }
                    "list" => {
                        match storage.list_collections() {
                            Ok(collections) => {
                                if collections.is_empty() {
                                    println!("There are no collections currently");
                                } else {
                                    for collection_name in collections {
                                        println!("- {}", collection_name);
                                    }
                                }
                            }
                            Err(e) => eprintln!("Error listing collections: {}.", e)
                        }
                    }
                    _ => {
                        eprintln!("Unknown command: {}", args[1])
                    }
                }
            }
            _ => {
                eprintln!("Unknown command {}", args[0])
            }
        }
    }

}
