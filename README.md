# MNIST/EMNIST Data Viewer (Rust)

This project is a Rust-based command-line application for parsing and displaying data from the MNIST and EMNIST datasets. It serves as a Rust port of an original Python script, developed for performance enhancement and as a learning exercise for the Rust programming language.

The application reads the binary IDX file format used by both MNIST and EMNIST, with support for both uncompressed and gzipped files. It can also perform the necessary image transformations (rotation and mirroring) required for the EMNIST dataset.

## Features

- Parses MNIST and EMNIST IDX file formats.
- Handles both uncompressed and gzipped (`.gz`) data files.
- Displays image data as ASCII art in the terminal.
- Shows label data corresponding to images.
- Supports EMNIST-specific image transformations.
- Interactive command-line interface to select files and data entries.

## Project Structure

The project is organized into two main files:

- `src/main.rs`: Contains the main application logic, including the user interface for selecting files and displaying data.
- `src/mnist_loader.rs`: A module responsible for the core logic of loading, parsing, and transforming the dataset files. It provides the `MnistData` struct.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) programming language and Cargo package manager.

## Building and Running

1.  **Clone the repository:**
    ```sh
    git clone <repository-url>
    cd mnist-viewer
    ```

2.  **Build and run the application:**
    Use the following command to compile and run the project:
    ```sh
    cargo run
    ```
    The application will start and prompt you to select a data file to inspect.

## Running Tests

To run the unit tests for the project, use this command:
```sh
cargo test
```
The tests primarily verify the image transformation logic in the `mnist_loader` module.

## Dependencies

This project relies on the following external crates:

- `byteorder`: For reading binary data with specific endianness.
- `flate2`: For handling Gzip-compressed files.
