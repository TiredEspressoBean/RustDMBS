//! # Test

use std::sync::{Arc, RwLock};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
/// Represents a collection of records in the database.
/// Each collection has a name and a vector of records stored with concurrent access control.
#[derive(Serialize, Deserialize)]
pub struct CollectionStorage {
    /// The name of the collection.
    pub name: String,

    /// The records stored in the collection, protected by an RwLock for concurrent access.
    pub data: RwLock<Vec<Record>>,
}

/// Represents a single record within a collection.
/// Each record contains a vector of values of various types.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    /// The values contained in this record.
    pub values: Vec<Value>,
}

/// Enum representing the different types of values that can be stored in a record.
/// It includes integer, float, boolean, and text values.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    /// Integer value.
    Integer(i32),

    /// Floating-point value.
    Float(f64),

    /// Boolean value.
    Bool(bool),

    /// Textual value.
    Text(String),

    Date(NaiveDate)
}

/// Enum representing the different data types that can be used.
/// Used for specifying the type of data expected in records or schemas.
#[derive(Serialize, Deserialize)]
pub enum DataType {
    /// Text data type.
    Text,

    /// Integer data type.
    Integer,

    /// Floating-point data type.
    Float,

    /// Boolean data type.
    Boolean,


}

/// A helper structure for reading from and writing to files.
/// It provides a way to mutate the collection storage for file operations.
#[derive(Serialize, Deserialize)]
pub struct CollectionStorageHelper {
    /// The name of the collection.
    pub name: String,

    /// The records in the collection, not protected by a lock.
    pub data: Vec<Record>,
}

impl CollectionStorageHelper {
    /// Converts the helper structure into a `CollectionStorage` instance,
    /// wrapping the data in an `RwLock` for concurrent access.
    ///
    /// # Returns
    ///
    /// An `Arc`-wrapped `CollectionStorage` instance with the collection's name and records.
    pub fn into_collection_storage(self) -> Arc<CollectionStorage> {
        Arc::new(CollectionStorage {
            name: self.name,
            data: RwLock::new(self.data),
        })
    }
}
