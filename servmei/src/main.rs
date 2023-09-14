use std::{env, str};
use std::net::{UdpSocket, Ipv4Addr};

fn main() {
    let mcast_group: Ipv4Addr = "239.0.0.1".parse().unwrap();
    let port = 6000u16;
    let any : Ipv4Addr = "0.0.0.0".parse().unwrap();

    let buffer = [0u8; 1600];
    if env::args().count() > 1 {
        let socket = UdpSocket::bind((any, port)).expect("failed to bind to client socket");
        socket.join_multicast_v4(&mcast_group, &any).expect("failed to join the multicast group");
        print!("{}", str::from_utf8(&buffer).expect("failed to write buffer as a string"));
    } else {
        let socket = UdpSocket::bind((any, 0)).expect("failed to bind to socket");
        socket.send_to("Here is some data for you!".as_bytes(), (mcast_group, port)).expect("failed to write the data");
    }
}
