# File Transfer with Iroh Blobs

This Rust package provides a simple file transfer utility using the [`iroh`](https://github.com/n0-computer/iroh) and [`iroh_blobs`](https://github.com/n0-computer/iroh-blobs) libraries. You can use this tool to send and receive files using a peer-to-peer network setup.

#### This currently uses memory for the transfer, so it is not suitable for large files.

#### Version with file system support is WIP

## Features
- Send files to a peer and generate a ticket for others to fetch the file.
- Receive files by providing a valid ticket.

## Requirements
- Rust (latest stable version)
- Tokio async runtime

## Installation

1. Clone the repository and navigate to the project directory:

    ```bash
    git clone https://github.com/webrror/sere.git
    cd sere
    ```


## Usage

The package provides two commands:
- `send`: Send a file.
- `receive`: Receive a file.

### Sending a File

To send a file, use the following command:

```bash
cargo run -- send [FILE_PATH]
