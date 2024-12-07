use std::net::{SocketAddrV4, UdpSocket};
use std::{env, io};

const IP: &str = "0.0.0.0";

fn main() {
    let args: Vec<_> = env::args().collect();
    let port = args[1]
        .parse::<u16>()
        .expect("port must exist as the first arg");
    let dest_ip = args[2]
        .parse::<SocketAddrV4>()
        .expect("destination socket address must exist as the second arg");

    println!("Binding on {}:{}/udp", IP, port);
    let socket = UdpSocket::bind((IP, port)).expect("couldn't bind to address");

    let mut stdin_buf = String::new();
    let stdin = io::stdin();
    loop {
        stdin
            .read_line(&mut stdin_buf)
            .expect("input should be valid byte sequence");
        socket
            .send_to(stdin_buf.as_bytes(), dest_ip)
            .expect("sending should succeed");
        stdin_buf.clear();

        let mut udp_buf = [0; 1024];
        socket.recv_from(&mut udp_buf).expect("receiving should succeed");
        println!("{}", String::from_utf8_lossy(&udp_buf));
    }
}
