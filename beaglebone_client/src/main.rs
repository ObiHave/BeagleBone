use std::net::UdpSocket;


fn main() {
    let client = UdpSocket::bind("192.168.7.2:8080").expect("Could not connect to 7.2:8080.");
    let mut buffer = [0, 100];
    println!("Listening on {}", client.local_addr().unwrap());
    loop {
        client.recv_from(&mut buffer).expect("Failed to recieve message.");
        println!("{:?}", buffer);
    }
}
