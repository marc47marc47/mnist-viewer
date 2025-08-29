# GEMINI.md

## Project Overview

This project is a Rust-based command-line application for parsing and displaying data from the MNIST and EMNIST datasets. It was originally a Python script, which has been ported to Rust for performance and to learn Rust.

The application reads the binary IDX file format used by MNIST and EMNIST, and can handle both uncompressed and gzipped files. It can also perform the necessary image transformations (rotation and mirroring) for the EMNIST dataset.

The main logic is encapsulated in the `mnist_loader` module, which provides a `MnistData` struct for loading the data. The `main.rs` file contains the application logic for loading the data, printing statistics, and displaying a sample image.

## Building and Running

To build and run the project, use the following command:

```sh
cargo run
```

This will compile and run the application, which will load the MNIST data from the `data` directory and display the first image.

To run the tests, use the following command:

```sh
cargo test
```

## Development Conventions

*   **Modules:** The data loading logic is separated into a `mnist_loader` module.
*   **Error Handling:** The application uses `std::io::Result` for error handling.
*   **Dependencies:** The project uses `byteorder` for reading binary data and `flate2` for handling gzipped files.
*   **Testing:** The `mnist_loader` module includes a unit test for the image transformation logic.
