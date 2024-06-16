# Rust TCP/HTTP Socket Server

## 目录结构

| 目录/文件  | 说明            |
| ---------- | --------------- |
| tcpclient  | TCP 客户端目录  |
| tcpserver  | TCP 服务端目录  |
| http       | HTTP 模块目录   |
| httpserver | HTTP 服务器目录 |

## 运行方式

### TCP 服务器

#### 启动 TCP 服务器

```shell
# `--package` or `-p`
cargo run --package tcpserver
```

#### 运行 TCP 客户端访问服务器

```shell
# `--package` or `-p`
cargo run --package tcpclient
```

执行成功后终端中会打印：  
`Response from server: "Hello"`

### HTTP 服务器

#### 启动 HTTP 服务器

```shell
# `--package` or `-p`
# 运行于 http://localhost:8000
cargo run --package httpserver
```

#### 暴露的 HTTP 接口

| 接口                                                                       | 响应        |
| -------------------------------------------------------------------------- | ----------- |
| <http://localhost:8000>                                                    | index.html  |
| <http://localhost:8000/health>                                             | health.html |
| <http://localhost:8000/style.css>                                          | style.css   |
| <http://localhost:8000/api/shipping/orders>                                | orders.json |
| <http://localhost:8000/anyelse>（“anyelse” 可改为接口中未列出的任意文字 ） | 404.html    |

#### HTTP 模块单元测试

```shell
# `--package` or `-p`
cargo test --package http
```

## 制作流程

### 1. 创建 Workspace

参考：<https://kaisery.github.io/trpl-zh-cn/ch14-03-cargo-workspaces.html>

- 在 `cargo new` 创建的目录下再次调用 `cargo new` 创建子目录，则第一层目录会被视为 Workspace。
- 只在第一层目录内有 `Cargo.lock` 文件。
- 使用 `cargo run --package <package_name>` 来指定运行的包。
  - 参考：<https://doc.rust-lang.org/cargo/commands/cargo-run.html#package-selection>。

### 2. 构建 TCP 服务器

- `std::net`: 标准库模块，提供 TCP 和 UDP 通信。

### 3. 实现 HTTP 模块

- 实现解析 HTTP 请求的模块。
- 实现解析 HTTP 响应的模块。

### 4. 创建 HTTP 服务器

1. 使用 `TcpListener` 模块创建 TCP socket 服务器并侦听端口。
2. 创建 buffer 以接收 TCP 字节流形式的请求文。
   - 浏览器会自己添加许多表头，因此如果为写入请求文字节流准备的 buffer 过小就会出现 crul 上请求成功而浏览器上请求失败的情况。
3. 调用 HTTP 请求模块根据 buffer 解析请求。
4. 调用路由处理请求和 TCP 字节流。

### 5. 创建路由与处理器

1. 根据不同的路由创建并调用处理器。
2. 由处理器处理请求并调用 HTTP 响应模块生成响应，返回响应至路由。
3. 在路由中调用响应模块实例上的发送响应方法通过 TCP 字节流发送响应。
