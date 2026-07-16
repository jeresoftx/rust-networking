use rust_networking::tcp::{SequenceNumber, TcpConnection, TcpSegment};

fn main() {
    let mut receiver = TcpConnection::established(SequenceNumber::new(1), SequenceNumber::new(1));

    receiver
        .receive(TcpSegment::data(SequenceNumber::new(6), b"world".to_vec()))
        .unwrap();
    receiver
        .receive(TcpSegment::data(SequenceNumber::new(1), b"hello".to_vec()))
        .unwrap();

    assert_eq!(receiver.received_payload(), b"helloworld");
}
