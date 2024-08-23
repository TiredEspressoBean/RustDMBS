use crate::db::schema::{CollectionStorage, Record, Value, CollectionStorageHelper};
use crate::utils::error::DBError;
use std::collections::HashMap;
use std::{fs};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::sync::{Arc, RwLock};
use fs2::FileExt;

/// The main engine responsible for handling in-memory storage interactions.
///
/// `StorageEngine` manages collections of data stored in memory, allowing for
/// concurrent access and modification. The collections are stored in a
/// `HashMap`, where the keys are `String` identifiers for each collection,
/// and the values are `Arc<CollectionStorage>` for thread-safe shared ownership.
///
/// # Fields
///
/// * `collections` - A `RwLock`-protected `HashMap` that maps collection names
///   (`String`) to their respective `Arc<CollectionStorage>`. The `RwLock` allows
///   for multiple readers or one writer to access the collections concurrently.
///
/// # Notes
///
/// - This struct is designed for concurrent environments, making use of `RwLock`
///   and `Arc` to ensure safe access and modification of collections.
pub struct StorageEngine {
    collections: RwLock<HashMap<String, Arc<CollectionStorage>>>,
}

impl StorageEngine {
    /// Loads the storage engine information from a file
    ///
    /// # Notes
    /// The path parameter allows for relative position("DB.json" would point to "DB.json" in the
    /// same directory as the DBMS) or absolut positioning
    ///
    /// # Arguments
    /// - `filepath`: Path to reach file which will be read from or if needed, created
    ///
    /// # Returns
    /// - `Ok()`: File has been read and its information stored to in memory storage
    pub fn load_from_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Check for file location and create one if the file as requested isn't there
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => {
                self.create_empty_file(path)?;
                return Ok(());
            }
        };
        // Parse the information from the file as JSON information
        let collections_helper: HashMap<String, CollectionStorageHelper> = serde_json::from_str(&content)?;
        // Lock the storage before manipulating it, the write lock will decompose once the function
        // has run through. Decomposing works by dropping the write lock, which releases the lock.
        let mut collections_lock = self.collections.write().map_err(|_| Box::new(DBError::StorageError("Uhoh".into())))?;
        // Push the information fromm the file into the main data store
        *collections_lock = collections_helper.into_iter().map(|(name, helper)| {
            (name, Arc::new(CollectionStorage {
                name: helper.name,
                data: RwLock::new(helper.data),
            }))
        }).collect();

        Ok(())
    }
    /// Saves the storage engine information to a file as specified by the path parameter
    ///
    /// # Arguments
    /// - `path`: Path to reach file which will be used to save the DBMS information to
    ///
    /// # Returns
    /// - `Ok()`: File information has been saved
    /// - `Err(dyn std::error::Error)`: File is unable to be opened created or locked
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = path;

        let collections_lock = self.collections.read().map_err(|_| Box::new(DBError::StorageError("Failed to acquire read lock".into())))?;

        let collections_helper: HashMap<String, CollectionStorageHelper> = collections_lock.iter().map(|(name, collection)| {
            let data = collection.data.read().map_err(|_| Box::new(DBError::StorageError("Failed to acquire read lock on data".into()))).expect("Reason").clone();
            (name.clone(), CollectionStorageHelper {
                name: collection.name.clone(),
                data,
            })
        }).collect();

        // Serialize the HashMap to JSON
        let json_content = serde_json::to_string(&collections_helper).map_err(|e| Box::new(DBError::StorageError(e.to_string())))?;

        // Write the JSON content to the file
        let mut file = File::create(path).map_err(|e| Box::new(DBError::StorageError(e.to_string())))?;
        file.write_all(json_content.as_bytes()).map_err(|e| Box::new(DBError::StorageError(e.to_string())))?;

        Ok(())
    }
    /// Create an empty file for the storage to use for data persistence
    ///
    /// # Arguments
    /// - `path`: Relative or absolute path to the file to save from
    ///
    /// # Returns
    /// - `Ok()`: File has been made
    /// - `Err(dyn std::error::Error)`: Operation failed
    fn create_empty_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let empty_data = "{}";
        let mut file = File::create(path).map_err(|e| Box::new(DBError::StorageError(e.to_string())))?;
        file.write_all(empty_data.as_bytes()).map_err(|e| Box::new(DBError::StorageError(e.to_string())))?;

        let mut collections_lock = self.collections.write().map_err(|_| Box::new(DBError::StorageError("Failed to acquire write lock".into())))?;
        *collections_lock = HashMap::new();

        Ok(())
    }
    /// Parsing strings into basic data types to use with the CLI
    ///
    /// Parse what kind of information is being passed to save to the DBMS, and wraps it in a
    /// \<Value\> structure which would be used to save in a collections \<Record\> structure. Values
    /// currently can be Bool, Integer, Floats, and Strings(referred to as Text)
    ///
    /// # Arguments
    /// - `s`: &str (string reference) of the information that which will be recorded and stored
    ///
    /// # Returns
    /// - `Ok(<Value>)`: Data parsed and wrapped in \<Value\> structure
    /// - `Err(dyn std::error::Error)`: File is unable to be saved
    pub fn parse_value(&self, s: &str) -> Result<Value, DBError> {
        if let Ok(bool_val) = s.parse::<bool>() {
            Ok(Value::Bool(bool_val))
        } else if let Ok(int_val) = s.parse::<i32>() {
            Ok(Value::Integer(int_val))
        } else if let Ok(float_val) = s.parse::<f64>() {
            Ok(Value::Float(float_val))
        } else if let Ok(string_val) = s.parse::<String>() {
            Ok(Value::Text(string_val))
        } else {
            Err(DBError::SchemaError("Something went wrong with parsing".into()))
        }
    }
    /// Create a new collection
    ///
    /// # Arguments
    /// - `collection_name`: Key for hashmap of collections
    ///
    /// # Returns
    /// - `Ok()`: Collection successfully added to the DB
    /// - `Err(DBError)`: There will be an error either in writing to the Storage Engine, or another
    ///         collection already has the same name that which is being used to add to the DB.
    pub fn add_collection(&self, collection_name: &str) -> Result<(), DBError> {
        let mut collections = self.collections.write().map_err(|_| DBError::StorageError("Failed to write collection".into()))?;

        if collections.contains_key(collection_name) {
            return Err(DBError::StorageError("Collection already exists".into()));
        }

        collections.insert(
            collection_name.to_string(),
            Arc::new(CollectionStorage {
                name: collection_name.to_string(),
                data: RwLock::new(Vec::new()),
            }),
        );

        Ok(())
    }
    /// Read a particular collection by cloning the data within it and returning that cloned data
    ///
    /// # Arguments
    /// - `collection_name`: Key of the collection that is being read from
    ///
    /// # Returns
    /// - `Ok(Vec<Record>)`: Cloned data from the DB
    /// - `Err(DBError)`: There will be an error either in getting a read lock such as if a
    ///         write lock is on it.
    pub fn read_collection(&self, collection_name: &str) -> Result<Vec<Record>, DBError> {
        let collections = self.collections.read().map_err(|_| DBError::StorageError("Failed to obtain readlock".into()))?;

        if let Some(collection) = collections.get(collection_name) {
            let data = collection.data.read().map_err(|_| DBError::StorageError("Failed to read collection".into()));
            Ok(data?.clone())
        } else {
            Err(DBError::StorageError("Collection {collection_name} does not exist".parse().unwrap()))
        }
    }
    /// Delete a collection from the database
    ///
    /// # Arguments
    /// - `collection_name` Name of the collection to remove from the database
    ///
    /// # Returns
    /// - `Ok()`: Collection has been successfully deleted
    /// - `Err(DBError)`
    pub fn delete_collection(&self, collection_name: &str) -> Result<(), DBError> {
        let mut collections = self.collections.write().map_err(|_| DBError::StorageError("Failed to delete collection".into()))?;
        if collections.remove(collection_name).is_some() {
            Ok(())
        } else {
            Err(DBError::StorageError("Failed to delete collection".into()))
        }
    }
    /// List all collections within the DB
    ///
    /// # Returns
    /// - `Ok(vec![])` Empty vector to represent that there is no collections currently
    /// - `Ok(Vec<cloned collection keys>)` Returns a vector of cloned keys in the DB that point to
    ///         collections
    pub fn list_collections(&self) -> Result<Vec<String>, DBError> {
        let collections = self.collections.read().map_err(|_| DBError::StorageError("Failed to list collections".into()))?;
        if collections.is_empty() {
            Ok(vec![])
        } else {
            Ok(collections.keys().cloned().collect())
        }
    }
    /// Create a new record
    ///
    /// # Arguments
    /// - `collection_name`: Key to access the collection in the DB hashmap
    /// - `record`: \<Record\> object to add to the collection
    ///
    /// # Returns
    /// - `Ok()`: Record has been created within the DB
    /// - `DBError`: Likely either failed
    pub fn create_record(&self, collection_name: &str, record: Record) -> Result<(), DBError> {
        let collections = self.collections.read().map_err(|_| DBError::StorageError("Failed to get collect for record creation".into()))?;
        if let Some(collection) = collections.get(collection_name) {
            let mut data = collection.data.write().map_err(|_| DBError::StorageError("Failed to create record".into()));
            data?.push(record);
            Ok(())
        } else {
            Err(DBError::StorageError("Collection {} does not exist".into()))
        }
    }
    /// Read a particular record from a collection, and return a clone of that information
    ///
    /// # Arguments
    /// - `collection name`: Name of the collection to be accessed
    /// - `index`: i32 index of the record within the collection
    ///
    /// # Returns
    /// - `Record`: Copy of the record object as it was read from the  DB
    /// - `DBError`: Likely either that the collection was unable to be found or the record was unable
    ///         to be found/accessed
    pub fn read_record(&self, collection_name: &str, index: i32) -> Result<Record, DBError> {
        let collections = self.collections.read().map_err(|_| DBError::StorageError("Unable to find collection".into()))?;
        if let Some(collection) = collections.get(collection_name) {
            let data = collection.data.read().map_err(|_| DBError::StorageError("Unable to find record".into()))?;
            data.get(index as usize).cloned().ok_or(DBError::StorageError("Unable to access record".into()))
        } else {
            Err(DBError::StorageError(format!("Unable to find collection, {}", collection_name)))
        }
    }
    /// Update a record and return a copy of the new record as it is stored in the database
    ///
    /// # Arguments
    /// - `collection name`: Name of the collection to be accessed
    /// - `index`: i32 index of the record within the collection
    ///
    /// # Returns
    /// - `Record`: Copy of the record as it is in the storage now that it has been updated
    /// - `DBError`: Likely either was unable to find the collection, or the record that is to be updated
    pub fn update_record(&self, collection_name: &str, index: i32, record: Record) -> Result<Record, DBError> {
        let mut collections = self.collections.write().map_err(|_| DBError::StorageError("Failed to update record".into()))?;
        if let Some(collection) = collections.get_mut(collection_name) {
            let mut old_data = collection.data.write().map_err(|_| DBError::StorageError("Unable to find record location".into()))?;
            old_data[index as usize] = record;
            Ok(old_data[index as usize].clone())
        } else {
            Err(DBError::StorageError(format!("Unable to find record, {}", index)))
        }
    }
    /// Delete a particular record from a collection in the database
    ///
    /// # Arguments
    /// - `collection name`: Name of the collection to be accessed
    /// - `index`: i32 index of the record within the collection
    ///
    /// # Returns
    /// -``:
    pub fn delete_record(&self, collection_name: &str, index: i32) -> Result<Record, DBError> {
        let mut collections = self.collections.write().map_err(|_| DBError::StorageError("Failed to delete record".into()))?;
        if let Some(collection) = collections.get_mut(collection_name) {
            let mut record = collection.data.write().map_err(|_| DBError::StorageError("Failed to find record to delete".into()));
            Ok(record?.remove(index as usize))
        } else {
            Err(DBError::StorageError(format!("Unable to find record to delete at {}", index)))
        }
    }
}

/// Spin up an initial empty StorageEngine wrapped in an Arc for concurrent operations
///
/// # Returns
/// - `Ok(Arc<StorageEngine>)` on success.
/// - `Err(DBerror)` on failure.
pub fn init_storage() -> Result<Arc<StorageEngine>, DBError> {
    let storage_engine = StorageEngine {
        collections: RwLock::new(HashMap::new())
    };
    Ok(Arc::new(storage_engine))
}

/// Locks a file on disc for shared read access
///
/// # Arguments
/// - `filepath`: Path to reach file which will be locked
///
/// # Returns
/// - `Ok(file)`: File with read operations permissions
/// - `Err(std::io::Error)`: File is unable to be opened or locked
fn lock_file_for_reading(filepath: &str) -> Result<std::fs::File, std::io::Error> {
    let file = OpenOptions::new().read(true).open(filepath)?;
    file.lock_shared()?;
    Ok(file)
}
/// Locks a file on disc for exclusive write access
///
/// # Arguments
/// - `filepath`: Path to reach file which will be locked
///
/// # Returns
/// - `Ok(file)`: File with write exclusive operations permissions
/// - `Err(std::io::Error)`: File is unable to be opened created or locked
fn lock_file_for_writing(filepath: &str) -> Result<std::fs::File, std::io::Error> {
    let file = OpenOptions::new().write(true).create(true).open(filepath)?;
    file.lock_exclusive()?;
    Ok(file)
}

/// Unlocks a file from read write operations
///
/// # Arguments
/// - `filepath`: Path to reach file which will be unlocked
///
/// # Returns
/// - `Ok()`: File has been successfully unlocked
/// - `Err(std::io::Error)`: File was unable to be unlocked
fn unlock_file(filepath: &File) -> Result<(), std::io::Error> {
    filepath.unlock()?;
    Ok(())
}