# Spotify Stories

> CLI app with ORM experiment

This repository is an experiment for a CLI (command-line interface) app written in RUST that integrates an ORM (Object-Relational Mapping) using Diesel.

The app uses the Clap library for command parsing, which allows users to easily interact with the app through the command line. The ORM aspect of the app is handled by Diesel, which provides a simple way to interact with databases in a type-safe and efficient manner.

## Requirements

- RUST 1.55.0 or later
- Diesel CLI

## Installation

To install the app, clone this repository to your local machine and run the following command:

```powershell
cargo install --path .
```

## Usage

To run the app, simply type the name of the app followed by the desired command and any options or arguments. For the command list run:

```powershell
spotify_stories -h
```

## Contributing

If you would like to contribute to this project, please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the [MIT License](./LICENSE).
