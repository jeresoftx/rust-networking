use rust_networking::quic::{QuicConnectionId, QuicPacket, QuicStream, QuicStreamId};

fn main() {
    let connection = QuicConnectionId::new("cid-basic").unwrap();
    let stream_id = QuicStreamId::new(0);
    let mut stream = QuicStream::new(stream_id);

    stream
        .receive(QuicPacket::new(
            connection,
            stream_id,
            1,
            b"primer paquete".to_vec(),
        ))
        .unwrap();

    println!("cargas recibidas: {}", stream.payloads_in_order().len());
}
