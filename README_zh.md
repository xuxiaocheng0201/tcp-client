# Tcp处理-Client端

[![Crate](https://img.shields.io/crates/v/tcp-client.svg)](https://crates.io/crates/tcp-client)
![Crates.io License](https://img.shields.io/crates/l/tcp-client)

**其他语言版本：[English](README.md)，[简体中文](README_zh.md)。**

# 描述

便捷的TCP服务-客户端。
服务端请见 [tcp-server](https://crates.io/crates/tcp-server)。

基于 [tcp-handler](https://crates.io/crates/tcp-handler)。

API [文档](https://docs.rs/tcp-client/)已完善。


# 用法

将以下内容添加到你的`Cargo.toml`：

```toml
[dependencies]
tcp-client = "~0.1"
```


# 示例

```rust,no_run
use tcp_client::define_client;
use tcp_client::errors::Result;

define_client!(pub CommonMyClient, MyClient, "MyTcpApplication");

impl MyClient {
    // 在此处定义你的方法
    // 示例：
    async fn my_method(&mut self) -> Result<(), NetworkError> {
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


# 版本对照表

与 [tcp-server](https://crates.io/crates/tcp-server) 板条箱具有相同协议的版本。
（推荐配套使用，否则可能会出现意料之外的BUG）

| client version    | server version   |
|-------------------|------------------|
| \>=0.2.0          | \>=0.3.0         |
| <0.2.0, \>=0.1.0  | <0.3.0 \>=0.2.0  |
| <0.1.0            | <0.2.0           |


# License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
