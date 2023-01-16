use serde::{Deserialize, Serialize};
use std::env;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    method: String,
    number: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    method: String,
    prime: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match &args[1][..] {
        "smoke_test" => {
            server(smoke_test);
        }
        "prime_time" => {
            server(prime_time);
        }
        "means_ends" => {
            server(means_ends);
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

fn means_ends(mut stream: TcpStream) {
    println!("accepted connection");
    struct Record {
        timestamp: i32,
        value: i64,
    }
    let mut db = Vec::new();
    let mut buf: [u8; 9] = [0; 9];
    loop {
        match stream.read_exact(&mut buf) {
            Ok(()) => match buf[0] {
                b'I' => {
                    let timestamp = i32::from_be_bytes(buf[1..5].try_into().unwrap());
                    let value = i32::from_be_bytes(buf[5..9].try_into().unwrap()).into();
                    db.push(Record { timestamp, value });
                }
                b'Q' => {
                    let min = i32::from_be_bytes(buf[1..5].try_into().unwrap());
                    let max = i32::from_be_bytes(buf[5..9].try_into().unwrap());
                    let mut sum = 0;
                    let mut n = 0;
                    for r in db.iter() {
                        if min <= r.timestamp && r.timestamp <= max {
                            n += 1;
                            sum += r.value;
                        }
                    }
                    let result: i32 = if n == 0 {
                        0
                    } else {
                        (sum / n).try_into().unwrap_or(0)
                    };
                    stream.write_all(&result.to_be_bytes()).unwrap();
                }
                other => {
                    dbg!(other);
                    break;
                }
            },
            _ => {
                break;
            }
        }
    }
}

fn smoke_test(mut stream: TcpStream) {
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

fn prime_time(mut stream: TcpStream) {
    println!("accepted connection");
    let mut buf: [u8; 1024] = [0; 1024];
    let mut request_str = Vec::new();
    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                println!("read data");
                request_str.extend_from_slice(&buf[0..n]);
                if buf[0..n].contains(&b'\n') {
                    let parts: Vec<&[u8]> = request_str.split(|&c| c == b'\n').collect();
                    let (&last, reqs) = parts.split_last().unwrap();
                    for req in reqs {
                        let resp = if let Ok(request) = serde_json::from_slice::<Request>(req) {
                            dbg!(&request);
                            if request.method != "isPrime" {
                                Response {
                                    method: "malformed".to_string(),
                                    prime: false,
                                }
                            } else {
                                Response {
                                    method: "isPrime".to_string(),
                                    prime: is_prime(request.number),
                                }
                            }
                        } else {
                            Response {
                                method: "malformed".to_string(),
                                prime: false,
                            }
                        };

                        stream
                            .write_all(&serde_json::to_vec(&resp).unwrap())
                            .unwrap();
                        stream.write_all(b"\n").unwrap();
                        if resp.method == "malformed" {
                            return;
                        }
                    }
                    request_str = last.to_vec();
                }
            }
            _ => panic!("argh"),
        }
    }
    println!("closing")
}

fn is_prime(number: f64) -> bool {
    if number < 2.0 {
        return false;
    }
    if number.fract().abs() > 1e-10 {
        return false;
    }

    let mut d = 2.0;
    while d <= number.sqrt() {
        if (number / d).fract() < 1e-10 {
            return false;
        }
        d += 1.0;
    }
    return true;
}
