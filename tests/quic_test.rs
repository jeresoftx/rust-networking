use rust_networking::quic::{
    ConnectionMigration, QuicConnectionId, QuicError, QuicHandshake, QuicPacket, QuicStream,
    QuicStreamId,
};

#[test]
fn streams_keep_independent_packet_order() {
    let connection = QuicConnectionId::new("cid-mobile-1").unwrap();
    let lessons = QuicStreamId::new(0);
    let telemetry = QuicStreamId::new(4);

    let mut lessons_stream = QuicStream::new(lessons);
    let mut telemetry_stream = QuicStream::new(telemetry);

    lessons_stream
        .receive(QuicPacket::new(
            connection.clone(),
            lessons,
            2,
            b"lesson-page-2".to_vec(),
        ))
        .unwrap();
    telemetry_stream
        .receive(QuicPacket::new(
            connection.clone(),
            telemetry,
            1,
            b"rtt=28ms".to_vec(),
        ))
        .unwrap();
    lessons_stream
        .receive(QuicPacket::new(
            connection,
            lessons,
            1,
            b"lesson-page-1".to_vec(),
        ))
        .unwrap();

    assert_eq!(
        lessons_stream.payloads_in_order(),
        vec![b"lesson-page-1".as_slice(), b"lesson-page-2".as_slice()]
    );
    assert_eq!(
        telemetry_stream.payloads_in_order(),
        vec![b"rtt=28ms".as_slice()]
    );
}

#[test]
fn migration_preserves_connection_identity_when_address_changes() {
    let connection = QuicConnectionId::new("cid-mobile-2").unwrap();
    let migration = ConnectionMigration::new(
        connection.clone(),
        "wifi:10.0.0.8:4433",
        "lte:172.16.8.20:4433",
        "cambio de red móvil",
    );

    assert_eq!(migration.connection_id(), &connection);
    assert_eq!(migration.previous_path(), "wifi:10.0.0.8:4433");
    assert_eq!(migration.current_path(), "lte:172.16.8.20:4433");
    assert!(migration.preserves_connection_identity());
}

#[test]
fn handshake_requires_integrated_tls_13_and_http_3_protocol() {
    let connection = QuicConnectionId::new("cid-secure-1").unwrap();

    let handshake = QuicHandshake::negotiate(connection, "TLS 1.3", "h3").unwrap();

    assert!(handshake.is_secure());
    assert_eq!(handshake.tls_version(), "TLS 1.3");
    assert_eq!(handshake.application_protocol(), "h3");
}

#[test]
fn rejects_handshake_without_tls_13() {
    let connection = QuicConnectionId::new("cid-insecure-1").unwrap();

    let error = QuicHandshake::negotiate(connection, "TLS 1.2", "h3").unwrap_err();

    assert_eq!(
        error,
        QuicError::InsecureHandshake {
            tls_version: "TLS 1.2".to_string(),
        }
    );
}

#[test]
fn packet_loss_blocks_only_the_affected_stream() {
    let connection = QuicConnectionId::new("cid-loss-1").unwrap();
    let video = QuicStreamId::new(0);
    let chat = QuicStreamId::new(8);

    let mut video_stream = QuicStream::new(video);
    let mut chat_stream = QuicStream::new(chat);

    video_stream.mark_lost(2);
    chat_stream
        .receive(QuicPacket::new(connection, chat, 1, b"hola".to_vec()))
        .unwrap();

    assert!(video_stream.is_blocked_by_loss());
    assert!(!chat_stream.is_blocked_by_loss());
    assert_eq!(chat_stream.payloads_in_order(), vec![b"hola".as_slice()]);
}

#[test]
fn rejects_packet_for_a_different_stream() {
    let connection = QuicConnectionId::new("cid-stream-error").unwrap();
    let expected = QuicStreamId::new(0);
    let actual = QuicStreamId::new(4);
    let mut stream = QuicStream::new(expected);

    let error = stream
        .receive(QuicPacket::new(connection, actual, 1, b"fuera".to_vec()))
        .unwrap_err();

    assert_eq!(error, QuicError::UnexpectedStream { expected, actual });
}
