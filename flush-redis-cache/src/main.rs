use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;

    stream.write_all(b"*1\r\n$8\r\nFLUSHALL\r\n")?;

    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Redis response: {}", response);

    Ok(())
}
