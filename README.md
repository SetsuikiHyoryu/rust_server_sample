# Rust 全栈式开发 Web 应用

## 目录结构

| 目录/文件 | 说明          |
| --------- | ------------- |
| client    | 客户端目录    |
| server    | 服务端目录    |
| http      | HTTP 模块目录 |

## 工作流

### 1. 创建 Workspace

参考：<https://kaisery.github.io/trpl-zh-cn/ch14-03-cargo-workspaces.html>

- 在 `cargo new` 创建的目录下再次调用 `cargo new` 创建子目录，则第一层目录会被视为 Workspace。
- 只在第一层目录内有 `Cargo.lock` 文件。
- 使用 `cargo run --package <package_name>` 来指定运行的包。
  - 参考：<https://doc.rust-lang.org/cargo/commands/cargo-run.html#package-selection>。

### 2. 构建 TCP 服务器

- `std::net`: 标准库模块，提供 TCP 和 UDP 通信。

### 3. 实现 HTTP 模块

- 实现解析 HTTP 请求的功能。
