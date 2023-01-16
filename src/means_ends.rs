use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

pub fn means_ends(mut stream: TcpStream) {
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
