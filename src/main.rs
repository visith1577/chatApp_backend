use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::{self, TryRecvError};


const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 64;

fn main() {
    let mut client =  TcpStream::connect(LOCAL).expect("stream connection failed");
    client.set_nonblocking(true).expect("Could not connect to server");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg  = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("message recv: {:#?}", msg);
            },
            Err(e) if e.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection failed");
                break;
            }
        }
        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("Writing to the socket failed!");
                println!("message sent: {:?}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }
        thread::sleep(std::time::Duration::from_millis(100));
    });
    println!("write a message");
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("reading from stdin failed");
        let msg = buf.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            break
        }
    }
    println!("bye bye");
}
