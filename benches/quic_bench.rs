use std::hint::black_box;
use std::time::Instant;

use rust_networking::quic::{
    ConnectionMigration, QuicConnectionId, QuicHandshake, QuicPacket, QuicStream, QuicStreamId,
};

fn despachar_paquetes_por_flujo(iteraciones: usize) {
    for i in 0..iteraciones {
        let connection = QuicConnectionId::new(format!("cid-{i}")).unwrap();
        let stream_id = QuicStreamId::new((i % 8) as u64);
        let mut stream = QuicStream::new(stream_id);
        stream
            .receive(QuicPacket::new(
                connection,
                stream_id,
                i as u64,
                b"payload".to_vec(),
            ))
            .unwrap();
        black_box(stream.payloads_in_order().len());
    }
}

fn negociar_conexiones(iteraciones: usize) {
    for i in 0..iteraciones {
        let connection = QuicConnectionId::new(format!("cid-secure-{i}")).unwrap();
        let secure = QuicHandshake::negotiate(connection, "TLS 1.3", "h3")
            .unwrap()
            .is_secure();
        black_box(secure);
    }
}

fn registrar_migraciones(iteraciones: usize) {
    for i in 0..iteraciones {
        let connection = QuicConnectionId::new(format!("cid-mobile-{i}")).unwrap();
        let migration = ConnectionMigration::new(
            connection,
            "wifi:10.0.0.8:4433",
            "lte:172.16.8.20:4433",
            "cambio de red",
        );
        black_box(migration.preserves_connection_identity());
    }
}

fn main() {
    let iteraciones = 30_000usize;

    let start = Instant::now();
    despachar_paquetes_por_flujo(iteraciones);
    let despacho_elapsed = start.elapsed();

    let start = Instant::now();
    negociar_conexiones(iteraciones);
    let negociacion_elapsed = start.elapsed();

    let start = Instant::now();
    registrar_migraciones(iteraciones);
    let migracion_elapsed = start.elapsed();

    println!("benchmark de quic (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("despacho por flujo: {despacho_elapsed:?}");
    println!("negociación integrada: {negociacion_elapsed:?}");
    println!("registro de migraciones: {migracion_elapsed:?}");
}
