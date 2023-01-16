use serde::{Deserialize, Serialize};
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::str;

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

pub fn prime_time(mut stream: TcpStream) {
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
