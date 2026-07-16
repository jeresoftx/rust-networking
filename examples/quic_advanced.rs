use rust_networking::quic::{
    QuicConnectionId, QuicHandshake, QuicPacket, QuicStream, QuicStreamId,
};

fn main() {
    let connection = QuicConnectionId::new("cid-advanced").unwrap();
    let handshake = QuicHandshake::negotiate(connection.clone(), "TLS 1.3", "h3").unwrap();
    let video_id = QuicStreamId::new(0);
    let chat_id = QuicStreamId::new(8);
    let mut video = QuicStream::new(video_id);
    let mut chat = QuicStream::new(chat_id);

    video.mark_lost(2);
    chat.receive(QuicPacket::new(connection, chat_id, 1, b"ok".to_vec()))
        .unwrap();

    println!("protocolo: {}", handshake.application_protocol());
    println!("video bloqueado: {}", video.is_blocked_by_loss());
    println!("chat bloqueado: {}", chat.is_blocked_by_loss());
}
