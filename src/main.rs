use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            let mut s = String::new();
            stream.read_to_string(&mut s).unwrap();
            println!("{}", s);
            println!("Banana!");
        });
    }
}
