use rust_networking::quic::{QuicConnectionId, QuicHandshake};

fn main() {
    let connection = QuicConnectionId::new("cid-solucion-handshake").unwrap();
    let handshake = QuicHandshake::negotiate(connection, "TLS 1.3", "h3").unwrap();

    assert!(handshake.is_secure());
    assert_eq!(handshake.application_protocol(), "h3");
}
