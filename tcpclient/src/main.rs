use std::{
    io::{Read, Result, Write},
    net::TcpStream,
    str,
};

fn main() -> Result<()> {
    // > After creating a TcpStream by either connecting to a remote host or accepting a connection on a TcpListener, data can be transmitted by reading and writing to it.
    //
    // > The connection will be closed when the value is dropped. The reading and writing portions of the connection can also be shut down individually with the shutdown method.
    // See: https://doc.rust-lang.org/std/net/struct.TcpStream.html
    let mut stream = TcpStream::connect("127.0.0.1:8000")?;

    // 向服务器发送数据
    stream.write_all("Hello".as_bytes())?;
    handle_response(stream)?;

    Ok(())
}

fn handle_response(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 5];

    // 将服务器的响应写入 buffer
    let _bypes_length = stream.read(&mut buffer)?;

    let response = str::from_utf8(&buffer);
    println!("Response from server: {:?}", response.unwrap());

    Ok(())
}
