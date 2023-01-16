use std::io::{Read, Write};
use std::net::TcpStream;

pub fn smoke_test(mut stream: TcpStream) {
    println!("accepted connection");
    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                println!("read data");
                stream.write(&buf[0..n]).expect("write failed");
            }
            _ => panic!("argh"),
        }
    }
    println!("closing")
}
