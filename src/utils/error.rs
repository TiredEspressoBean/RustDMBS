use std::fmt;

#[derive(Debug)]
pub enum DBError {
    StorageError(String),
    OperationError(String),
    QueryError(String),
    SchemaError(String),
    GeneralError(String),
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DBError::StorageError(msg) => write!(f, "StorageError: {}", msg),
            DBError::OperationError(msg) => write!(f, "OperationError: {}", msg),
            DBError::QueryError(msg) => write!(f, "QueryError: {}", msg),
            DBError::GeneralError(msg) => write!(f, "GeneralError: {}", msg),
            DBError::SchemaError(msg) => write!(f, "GeneralError: {}", msg)
        }
    }
}

impl std::error::Error for DBError {}

pub fn storage_error(msg: &str) -> DBError {
    DBError::StorageError(msg.to_string())
}

pub fn operation_error(msg: &str) -> DBError {
    DBError::OperationError(msg.to_string())
}

pub fn query_error(msg: &str) -> DBError {
    DBError::QueryError(msg.to_string())
}

pub fn general_error(msg: &str) -> DBError {
    DBError::GeneralError(msg.to_string())
}

pub fn schema_error(msg: &str) -> DBError {
    DBError::GeneralError(msg.to_string())
}
