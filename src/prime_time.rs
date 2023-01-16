use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
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
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buf = Vec::new();
    loop {
        buf.clear();
        match reader.read_until(b'\n', &mut buf) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                dbg!(str::from_utf8(&buf));
                let resp = if let Ok(request) = serde_json::from_slice::<Request>(&buf) {
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
                dbg!(&resp);
                stream
                    .write_all(&serde_json::to_vec(&resp).unwrap())
                    .unwrap();
                stream.write_all(b"\n").unwrap();
                if resp.method == "malformed" {
                    return;
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
