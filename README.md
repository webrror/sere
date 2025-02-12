# File Transfer with Iroh Blobs

<p>
<!--     <img alt="Actions" src="https://github.com/webrror/sere/actions/workflows/rust.yml/badge.svg"> -->
<!--     <img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/webrror/sere"> -->
    <img alt="Language" src="https://img.shields.io/badge/Rust-8A2BE2">
    <img alt="GitHub Issues or Pull Requests" src="https://img.shields.io/github/issues-raw/webrror/sere?color=8A2BE2">
    <img alt="GitHub Pull Requests" src="https://img.shields.io/github/issues-pr/webrror/sere?color=8A2BE2">
</p>


This Rust package provides a simple file transfer utility using the [`iroh`](https://github.com/n0-computer/iroh) and [`iroh_blobs`](https://github.com/n0-computer/iroh-blobs) libraries. You can use this tool to send and receive files using a peer-to-peer network setup.

> [!CAUTION] 
> This currently uses memory for the transfer, so it is not suitable for large files.

<!-- > [!NOTE] 
> Version with file system support is WIP -->

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
cargo run send [FILE_PATH]
```

### Receiving a File

To receive a file, use the following command:

```bash
cargo run receive [TICKET] [FILE_PATH]
```

## References

- [iroh](https://github.com/n0-computer/iroh)
- [iroh-blobs](https://github.com/n0-computer/iroh-blobs)
- [Unlimited, free file transfer, no account required [Youtube]](https://youtu.be/jl4cAkRTMT8?si=dtEXhC2fiqr7Rbsn)
- [sendme](https://www.iroh.computer/sendme)
- [Simple peer-to-peer file transfer tool](https://www.iroh.computer/docs/quickstart)
