use std::net::UdpSocket;
use std::{str, io};

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8000").expect("failed to connect the client with the socket");
    socket.connect("127.0.0.1: 8888").expect("drats, could not connect to the server group");
    loop {
        let mut input = String::new();  
        let mut buffer = [0u8; 1500];
        io::stdin().read_line(&mut input).expect("failed to read from terminal");

        socket.send(input.as_bytes()).expect("failed to write to the server");
        socket.recv_from(&mut buffer).expect("could not read into buffer");
        print!("{}", str::from_utf8(&buffer).expect("could not write buffer string"));
    }
}