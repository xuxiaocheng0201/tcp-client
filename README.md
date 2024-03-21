# Tcp-Client

[![Crate](https://img.shields.io/crates/v/tcp-client.svg)](https://crates.io/crates/tcp-client)
![Crates.io License](https://img.shields.io/crates/l/tcp-client)

**Read this in other languages: [English](README.md), [简体中文](README_zh.md).**

# Description

Convenient client-side TCP service.
Also see [tcp-server](https://crates.io/crates/tcp-server) for server-side.

Based on [tcp-handler](https://crates.io/crates/tcp-handler).

With complete API [document](https://docs.rs/tcp-client/).


# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
tcp-client = "~0.2"
```


# Example

```rust,no_run
use tcp_client::define_client;
use tcp_client::errors::Result;

define_client!(pub CommonMyClient, MyClient, "MyTcpApplication");

impl MyClient {
    // define your method here.
    // example:
    async fn my_method(&mut self) -> Result<()> {
        self.check_func("my_method").await?;
        // ...
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut client = MyClient::connect("127.0.0.1:1234").await.unwrap();
    // use client.
    // example:
    client.my_method().await.unwrap();
}
```

# Version map

Versions map to [tcp-server](https://crates.io/crates/tcp-server) with the same protocol.
(Recommended for use in conjunction, otherwise unexpected bugs may occur.)

| client version   | server version  |
|------------------|-----------------|
| \>=0.2.0         | \>=0.3.0        |
| <0.2.0, \>=0.1.0 | <0.3.0 \>=0.2.0 |
| <0.1.0           | <0.2.0          |


# License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
