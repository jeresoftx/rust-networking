use rust_networking::tcp::{SequenceNumber, TcpConnection, TcpSegment};

fn main() {
    let mut receiver = TcpConnection::established(SequenceNumber::new(1), SequenceNumber::new(1));

    let second = TcpSegment::data(SequenceNumber::new(6), b"mundo".to_vec());
    receiver.receive(second).unwrap();

    println!(
        "segmentos fuera de orden pendientes: {}",
        receiver.pending_segments()
    );

    let first = TcpSegment::data(SequenceNumber::new(1), b"hola ".to_vec());
    receiver.receive(first).unwrap();

    let delivered = String::from_utf8(receiver.received_payload().to_vec()).unwrap();
    println!("carga útil entregada: {delivered}");

    assert_eq!(delivered, "hola mundo");
}
