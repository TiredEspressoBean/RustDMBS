mod db;
mod utils;

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
