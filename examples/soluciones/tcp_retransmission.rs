use rust_networking::tcp::{SequenceNumber, TcpConnection};

fn main() {
    let mut cliente = TcpConnection::new_client(SequenceNumber::new(10));
    let syn = cliente.open().unwrap();

    let retransmision = cliente.retransmit_unacked().unwrap();

    assert_eq!(retransmision, vec![syn]);
}
