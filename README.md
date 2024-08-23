# RustDBMS

RustDBMS is a high-performance, concurrent in-memory database management system (DBMS) written in Rust. It provides efficient data storage, retrieval, and persistence with support for concurrency control and file-based locking.

## Features

- **Concurrency Control:** Utilizes Rust’s `RwLock` to allow safe concurrent access to data, supporting multiple readers and a single writer.
- **Data Persistence:** Stores data using JSON serialization, enabling persistence across program restarts.
- **File-Based Locking:** Implements file-based locking to prevent data corruption during file operations with support for shared and exclusive locks.
- **Command-Line Interface (CLI):** Includes a CLI for interacting with the database, including creating, reading, updating, and deleting collections and records.

## Installation

To get started with RustDBMS, follow these steps:

1. **Clone the Repository:**
   ```sh
   git clone https://github.com/yourusername/RustDBMS.git
   
2. **Navigate to the Project Directory:**

    ```sh
    cd RustDBMS

3. **Build the Project:**
    ```sh
    cargo build

4. **Run the Project:**
    ```sh 
    cargo run
   
## Usage
After building the project, you can use the CLI to interact with the database. Here are some common commands:

1. **Create a Collection:**
    ```sh
    cargo run -- create-collection <collection_name>

2. **Add a Record:**
    ```sh
    cargo run -- add-record <collection_name> <record_data>

3. **Retrieve Records:**
    ```sh
    cargo run -- get-records <collection_name>
4. **Delete a Collection:**
    ```sh
    cargo run -- delete-collection <collection_name>
Refer to the CLI documentation for more details on available commands and options.

## Documentation
For more detailed information about the project, including API documentation and design decisions, please refer to the generated documentation:

    cargo doc --open
    
## Contributing
We welcome contributions to RustDBMS! If you would like to contribute, please follow these guidelines:

1. Fork the Repository: Create your own fork of the repository on GitHub.
2. Create a Branch: Create a new branch for your changes.
3. Make Your Changes: Implement the changes or new features.
4. Submit a Pull Request: Submit a pull request to the main repository.

Please ensure that your code follows Rust's style guidelines and includes appropriate tests.

## License

RustDBMS is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Contact

For any questions or feedback, please reach out to [isherwoc@oregonstate.edu](mailto:isherwoc@oregonstate.edu).