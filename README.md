# Rusty Shell

Rusty Shell is a custom shell implemented in Rust. It provides various commands to manage files and directories.

## Table of Contents

- [Rusty Shell](#rusty-shell)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Build the project](#build-the-project)
  - [Usage](#usage)
  - [Commands](#commands)
  - [Testing](#testing)
  - [Docker](#docker)
  - [Binary](#binary)

## Installation

To install Rusty Shell, you need to have Rust installed on your system. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/). Or you can also use docker (Check Run in docker)

Clone the repository:

```sh
git clone https://github.com/yourusername/rusty-shell.git
cd rusty-shell
```

## Build the project
```sh
cargo build --release
```

## Usage
```sh
cargo run
```

## Commands

Rusty Shell supports the following commands:

- copy: Copy files or directories.
- list: List files or directories.
- move: Move or rename files or directories.
- remove: Remove files or directories.

## Testing

```sh
cargo test
```

## Docker

If you have docker installed and running you can use Makefile commands. Here the list:

- `make build`: Build the app
- `make run` : Run the app
- `make test`: Run unit tests
- `make clean`: Clean docker containers and images
- `make stop`: Stops app container

## Binary

Addionally, if you don't want to build and run the app using `cargo` I also added the `./rusty` binary to run the result of the app. You just need to open a terminal and run one of the following commands:

- `./rusty ls <directory>` : List all the files in the directory
- `./rusty cp <source_file_path> <destination_path>`: Copy source_file_path into destination_folder
- `./rusty mv <source_file_path> <destination_path>`: Move source_file_path into destination_folder
- `./rusty rm <file_path>`: Remove file_path
