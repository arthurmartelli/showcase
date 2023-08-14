# Rust Multithreaded Web Server

This is a simple multithreaded web server written in Rust. It uses the std::net library to listen for incoming connections on a specified port, and serves files from a specified directory.

## Getting Started

To run the server, you'll need Rust installed on your machine. You can download it from the official Rust website: https://www.rust-lang.org/tools/install

Once Rust is installed, you can clone this repository and run the server using the following commands:

```bash
git clone https://github.com/yourusername/rust-multithreaded-web-server.git

cd rust-multithreaded-web-server

cargo run --release
```

The server will start listening for incoming connections on port `8080` by default. You can specify a different port using the `--port` option:

```bash
cargo run --release -- --port 8000
```

You can also specify a directory to serve files from using the `--dir` option:

```bash
cargo run --release -- --dir /path/to/files
```

## Features

This server has the following features:

- Multithreaded: The server uses Rust's std::thread library to handle incoming connections on separate threads, allowing it to handle multiple connections simultaneously.
- Simple: The server is designed to be easy to understand and modify. It uses a minimal amount of code and external dependencies.
- Static file serving: The server can serve static files from a specified directory.
- Customizable: The server can be customized using command-line options to specify the port and directory to serve files from.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

This project was inspired by Rust HTTP Server and the Rust Book.
