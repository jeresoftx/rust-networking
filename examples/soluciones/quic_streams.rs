use rust_networking::quic::{QuicConnectionId, QuicPacket, QuicStream, QuicStreamId};

fn main() {
    let connection = QuicConnectionId::new("cid-solucion-streams").unwrap();
    let stream_id = QuicStreamId::new(0);
    let mut stream = QuicStream::new(stream_id);

    stream
        .receive(QuicPacket::new(connection, stream_id, 1, b"hola".to_vec()))
        .unwrap();

    assert_eq!(stream.payloads_in_order(), vec![b"hola".as_slice()]);
}
