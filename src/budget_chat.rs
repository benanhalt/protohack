use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
enum Channel {
    NewClient(usize, TcpStream),
    Msg(usize, String),
    End(usize),
}

pub fn main() {
    let (channel_tx, channel_rx) = mpsc::channel();

    thread::spawn(sender(channel_rx));

    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("listening on 8888");

    for (i, stream) in listener.incoming().enumerate() {
        println!("accepted connection");
        let stream = stream.unwrap();
        let channel = channel_tx.clone();
        let reader = BufReader::new(stream.try_clone().unwrap());

        channel_tx.send(Channel::NewClient(i, stream)).unwrap();

        thread::spawn(receiver(i, reader, channel));
    }
}

fn receiver(
    id: usize,
    mut reader: BufReader<TcpStream>,
    channel: mpsc::Sender<Channel>,
) -> impl FnMut() {
    return move || {
        let mut buf = Vec::new();
        loop {
            buf.clear();
            match reader.read_until(b'\n', &mut buf) {
                Ok(0) => {
                    break;
                }
                Ok(_) => {
                    let msg = str::from_utf8(&buf).unwrap().trim().to_string();
                    channel.send(Channel::Msg(id, msg)).unwrap();
                }
                _ => {
                    break;
                }
            }
        }
        channel.send(Channel::End(id)).unwrap();
    };
}

fn send(mut stream: &TcpStream, msg: &str) {
    stream.write_all(msg.as_bytes()).unwrap();
}

fn sender(channel_rx: mpsc::Receiver<Channel>) -> impl Fn() {
    return move || {
        let mut clients = HashMap::new();
        let mut names: HashMap<usize, String> = HashMap::new();

        for msg in &channel_rx {
            dbg!(&msg);
            match msg {
                Channel::NewClient(id, stream) => {
                    send(&stream, "Welcome. Enter your name.\n");
                    clients.insert(id, stream);
                }
                Channel::Msg(id, msg) => match names.get(&id) {
                    Some(name) => {
                        for (other_id, stream) in clients.iter() {
                            if names.contains_key(other_id) && *other_id != id {
                                send(&stream, &format!("[{}] {}\n", name, msg));
                            }
                        }
                    }
                    None => {
                        if !msg.chars().all(|c| c.is_alphanumeric()) {
                            let stream = clients.get(&id).unwrap();
                            send(&stream, "Name must be alphanumeric.\n");
                            stream.shutdown(Shutdown::Both).unwrap();
                            continue;
                        }
                        for (other_id, stream) in clients.iter() {
                            if names.contains_key(other_id) && *other_id != id {
                                send(&stream, &format!("* {} has connected.\n", msg));
                            }
                        }
                        let who: Vec<_> = names.values().map(|s| s.to_string()).collect();
                        send(
                            &clients.get(&id).unwrap(),
                            &format!("* In the chat: {}\n", who.join(", ")),
                        );
                        names.insert(id, msg);
                    }
                },
                Channel::End(id) => {
                    if names.contains_key(&id) {
                        for (other_id, stream) in clients.iter() {
                            if names.contains_key(other_id) && *other_id != id {
                                send(
                                    &stream,
                                    &format!("* {} has disconnected.\n", names.get(&id).unwrap()),
                                );
                            }
                        }
                        names.remove(&id);
                        clients.remove(&id);
                    }
                }
            }
        }
    };
}
