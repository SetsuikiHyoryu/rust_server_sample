use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> Result<()> {
    // > After creating a TcpListener by binding it to a socket address, it listens for incoming TCP connections. These can be accepted by calling accept or by iterating over the Incoming iterator returned by incoming.
    // > The socket will be closed when the value is dropped.
    // See: https://doc.rust-lang.org/std/net/struct.TcpListener.html
    //
    // “?” 是错误传播的简写。
    // See: https://kaisery.github.io/trpl-zh-cn/ch09-02-recoverable-errors-with-result.html#%E4%BC%A0%E6%92%AD%E9%94%99%E8%AF%AF%E7%9A%84%E7%AE%80%E5%86%99-%E8%BF%90%E7%AE%97%E7%AC%A6
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Running on port 8000...");

    // `listener.accept()` 只会接受一次连接。
    // `listener.incoming()` 返回一个迭代器，这个迭代器会监听每个连接。
    // 每个连接都是一个字节流，其类型为 `TcpStream`。
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    println!("Connection established!");

    let mut buffer = [0; 1024];

    // 将接受到的数据写入 buffer
    // `read` 有中断的风险，但 `read_exact` 在 buffer 读满之前会阻塞，需要按需选择。
    // stream.read_exact(&mut buffer)?;
    let _bytes_length = stream.read(&mut buffer)?;

    // 将 buffer 中的数据作为响应发送
    // `write` / `write_all` 的关系同 `read` / `read_exact`。
    stream.write_all(&buffer)?;

    Ok(())
}
