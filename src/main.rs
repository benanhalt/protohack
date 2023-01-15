use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::thread;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8888")?;
    println!("listening on 8888");
    for stream in listener.incoming() {
        thread::spawn(|| {
            println!("accepted connection");
            let mut stream = stream.unwrap();
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
        });
    }
    return Ok(());
}
