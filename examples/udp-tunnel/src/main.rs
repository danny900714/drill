use std::net::{SocketAddr, UdpSocket};
use std::{env, io};

fn main() {
    let args: Vec<_> = env::args().collect();
    let bind_ip = args[1]
        .parse::<SocketAddr>()
        .expect("bind socket address must exist as the first arg");
    let dest_ip = args[2]
        .parse::<SocketAddr>()
        .expect("destination socket address must exist as the second arg");

    println!("Binding on {}/udp", bind_ip);
    let socket = UdpSocket::bind(bind_ip).expect("couldn't bind to address");

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
        socket
            .recv_from(&mut udp_buf)
            .expect("receiving should succeed");
        println!("{}", String::from_utf8_lossy(&udp_buf));
    }
}
