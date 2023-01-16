use std::env;
use std::net::{TcpListener, TcpStream};
use std::thread;

mod means_ends;
mod prime_time;
mod smoke_test;

fn main() {
    let args: Vec<String> = env::args().collect();

    match &args[1][..] {
        "smoke_test" => {
            server(smoke_test::smoke_test);
        }
        "prime_time" => {
            server(prime_time::prime_time);
        }
        "means_ends" => {
            server(means_ends::means_ends);
        }
        other => {
            dbg!(other);
        }
    }
}

fn server(handler: fn(TcpStream)) {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("listening on 8888");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            handler(stream);
        });
    }
}
