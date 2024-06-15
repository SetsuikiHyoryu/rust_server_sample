# Rust 全栈式开发 Web 应用

## 1. 创建 Workspace

参考：<https://kaisery.github.io/trpl-zh-cn/ch14-03-cargo-workspaces.html>

- 在 `Cargo new` 创建的目录下再次调用 `Cargo new` 创建子目录，则第一层目录会被视为 Workspace。
- 只在第一层目录内有 `Cargo.lock` 文件。

## 1. 构建 TCP 服务器

- `std::net`: 标准库模块，提供 TCP 和 UDP 通信。
