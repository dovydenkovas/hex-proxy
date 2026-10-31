# hex-proxy

[![Rust](https://github.com/dovydenkovas/hex-proxy/actions/workflows/rust.yml/badge.svg)](https://github.com/dovydenkovas/hex-proxy/actions/workflows/rust.yml)

A tiny, asynchronous TCP reverse proxy daemon.

`hex-proxy` allows you to set up multiple TCP reverse proxy rules that listen on specified addresses and redirect incoming connections to a target destination.

## Features

- Asynchronous proxying using tokio
- Configurable rules: Define multiple proxy mappings in a TOML configuration file.
- Lifecycle Hooks: Execute arbitrary shell commands on various connection events:
    - `on_open`: When a connection is successfully established with the destination.
    - `on_close`: When a connection is closed.
    - `on_fail`: When a connection to the destination fails.
- Lightweight: Minimal dependencies and high performance.

## Installation

### Prerequisites

You must have [Rust and Cargo](https://rustup.rs/) installed on your system.

### Building from source

1. Clone this repository to your local machine.
2. Go to the project directory.
3. Build the release version:

    ```bash
    cargo build --release
    ```

4. The executable will be available in `target/release/hex-proxy`.

## Usage

### Configuration

Create a configuration file (e.g., `config.toml`) with the following structure:

```toml
[[proxy]]
listen_address = "127.0.0.1:8080"
target_address = "127.0.0.1:9090"
on_open = "echo 'New connection established'"
on_close = "echo 'Connection closed'"
on_fail = "echo 'Failed to connect to destination'"

[[proxy]]
listen_address = "0.0.0.0:3000"
target_address = "example.com:80"
```

### Running

Run the proxy utility by specifying the configuration file:

```bash
./target/release/hex-proxy --config config.toml
```

## Configuration Details

The configuration uses a list of `proxy` objects:

| Field            | Type             | Description                                                               |
| :--------------- | :--------------- | :------------------------------------------------------------------------ |
| `listen_address` | `String`         | The address and port to listen on (e.g., `127.0.0.1:8080`).               |
| `target_address` | `String`         | The destination address and port to redirect to (e.g., `127.0.0.1:9090`). |
| `on_open`        | `Option<String>` | Shell command to execute when a connection is successfully opened.        |
| `on_close`       | `Option<String>` | Shell command to execute when a connection is closed.                     |
| `on_fail`        | `Option<String>` | Shell command to execute when connection to the destination fails.        |

## License

Copyright (C) 2026 Aleksandr Dovydenkov <asd@altlinux.org>.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

See the [LICENSE](LICENSE) file for full details.
